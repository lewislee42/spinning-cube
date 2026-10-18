use crate::vec3::Vec3;
use color_eyre::Result;
use enum_cycling_derive::EnumCycle;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph},
    DefaultTerminal, Frame,
};

pub struct RendererInfo {
    pub fps: u32,
    pub delta_time: u128,
    pub clicked_x: u16,
    pub clicked_y: u16,
    pub drag_dir: Vec3,
    pub is_dragging: bool,
    pub selected_pane: Panes,
    pub is_auto_rotating: bool,
}

pub struct ObjectInfo {
    pub rotation_velocity: Vec3,
}

#[derive(EnumCycle, PartialEq, Clone, Copy)]
pub enum Panes {
    Renderer,
    Object,
}

pub struct DisplayManager {
    pub obj_render_area: Rect,
    // renderer_info_pane_scroll: f32,
    // object_info_pane_scroll: f32,
}

impl DisplayManager {
    pub fn default() -> Self {
        let mut container_rect = Rect::new(0, 0, 0, 0);
        if let Ok((width, height)) = crossterm::terminal::size() {
            container_rect.width = width as u16;
            container_rect.height = height as u16;
        } else {
            std::process::exit(1);
        }

        let render_sections = Self::get_render_area(container_rect);

        Self {
            obj_render_area: render_sections[1],
            // renderer_info_pane_scroll: 5.0,
            // object_info_pane_scroll: 5.0,
        }
    }

    pub fn print_frame(
        &mut self,
        terminal: &mut DefaultTerminal,
        renderer_info: &RendererInfo,
        object_info: &ObjectInfo,
        buffer: &Vec<char>,
    ) -> Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();

            let (object_area, top_cell, middle_cell, bottom_cell) = self.split_frame(area);
            self.obj_render_area = object_area;

            self.draw_title(frame, top_cell);

            if renderer_info.selected_pane == Panes::Renderer {
                self.draw_renderer_info(frame, middle_cell, renderer_info);
            } else if renderer_info.selected_pane == Panes::Object {
                self.draw_object_info(frame, middle_cell, object_info);
            }

            self.draw_keybinds(frame, bottom_cell);

            self.draw_object(frame, buffer);
        })?;
        Ok(())
    }

    fn get_render_area(area: Rect) -> [Rect; 2] {
        Layout::horizontal([Constraint::Percentage(15), Constraint::Fill(1)]).areas(area)
    }

    fn split_frame(&self, area: Rect) -> (Rect, Rect, Rect, Rect) {
        let columns = Self::get_render_area(area);

        // 2. Split Row 0 into 4 columns -> Column 0 is Top-Left
        let info_rows = Layout::vertical([
            Constraint::Length(5),      // Row 0 (Top)
            Constraint::Fill(1),        // Row 1
            Constraint::Percentage(20), // Row 2 (Bottom)
        ])
        .split(columns[0]);

        (columns[1], info_rows[0], info_rows[1], info_rows[2])
    }

    fn draw_title(&self, frame: &mut Frame, area: Rect) {
        let title = "▓▀▄▀▄  ▄▀▀▀▄ █▀▀▀▄ ▄▀▀▀▀ █   █ ▄▀▀▀▀ █▀▀▀▄
▒ ▀  ▀ ▓▄▄▄▀ ▓▄▄▄▀ ▒     ▓▀▀▀▀ ▓▀▀   ▓▄▄▄▀
░    █ ▒   █ ▒   █ ▀▄▄▄▄ ▒   █ ░▄▄▄▄ ▒   █";
        let block = Paragraph::new(title)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_renderer_info(&self, frame: &mut Frame, area: Rect, renderer_info: &RendererInfo) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" State information ")
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            });

        let left_block = Paragraph::new(vec![
            Line::from("FPS:"),
            Line::from("Frametime:"),
            Line::from("Clicked:"),
            Line::from("Drag direction:"),
            Line::from("Is dragging?:"),
            Line::from("Auto rotate:"),
        ])
        .block(block.clone())
        .alignment(Alignment::Left);

        let right_block = Paragraph::new(vec![
            Line::from(format!("{}", renderer_info.fps,)),
            Line::from(format!("{}", renderer_info.delta_time)),
            Line::from(format!(
                "X:{}, Y:{}",
                renderer_info.clicked_x, renderer_info.clicked_y
            )),
            Line::from(format!(
                "X:{}, Y:{}, Z:{}",
                renderer_info.drag_dir.x, renderer_info.drag_dir.y, renderer_info.drag_dir.z
            )),
            Line::from(format!("{}", renderer_info.is_dragging)),
            Line::from(format!("{}", renderer_info.is_auto_rotating)),
        ])
        .block(block.clone())
        .alignment(Alignment::Right);

        frame.render_widget(left_block, area);
        frame.render_widget(right_block, area);
    }

    fn draw_object_info(&self, frame: &mut Frame, area: Rect, object_info: &ObjectInfo) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Object information ")
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            });

        let left_block = Paragraph::new(vec![Line::styled("Rotation velocity:", Style::default())])
            .block(block.clone())
            .alignment(Alignment::Left)
            .style(Style::default());

        let right_block = Paragraph::new(vec![Line::from(format!(
            "Pitch:{:.3}, Yaw:{:.3}",
            object_info.rotation_velocity.x, object_info.rotation_velocity.y
        ))])
        .block(block.clone())
        .alignment(Alignment::Right);

        frame.render_widget(left_block, area);
        frame.render_widget(right_block, area);
    }

    fn draw_object(&self, frame: &mut Frame, buffer: &Vec<char>) {
        let frame_buffer = frame.buffer_mut();

        for y in 1..self.obj_render_area.height - 1 {
            for x in 1..self.obj_render_area.width - 1 {
                let screen_x = self.obj_render_area.x + x as u16;
                let screen_y = self.obj_render_area.y + y as u16;

                if let Some(cell) = frame_buffer.cell_mut((screen_x, screen_y)) {
                    cell.set_char(buffer[(x + y * self.obj_render_area.width) as usize]);
                }
            }
        }

        let block = Block::default().borders(Borders::ALL).title_bottom(
            Line::from(format!(
                " {} x {} ",
                self.obj_render_area.width, self.obj_render_area.height
            ))
            .right_aligned(),
        );
        frame.render_widget(block, self.obj_render_area);
    }

    fn draw_keybinds(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Keybind info ")
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            });

        let info_block =
            Paragraph::new(vec![Line::from("Quit"), Line::from("Swap displayed data")])
                .alignment(Alignment::Left)
                .block(block.clone());

        let key_block = Paragraph::new(vec![Line::from("Esc or Q"), Line::from("A or D")])
            .alignment(Alignment::Right)
            .block(block);

        frame.render_widget(info_block, area);
        frame.render_widget(key_block, area);
    }
}
