use crate::vec3::Vec3;
use color_eyre::Result;
use enum_cycling_derive::EnumCycle;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Padding},
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
    pub render_area: Rect,
    renderer_info_pane_scroll: f32,
    object_info_pane_scroll: f32,
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
            render_area: render_sections[1],
            renderer_info_pane_scroll: 5.0,
            object_info_pane_scroll: 5.0,
        }
    }

    pub fn print_frame(
        &self,
        terminal: &mut DefaultTerminal,
        renderer_info: &RendererInfo,
        object_info: &ObjectInfo,
        buffer: &Vec<char>,
    ) -> Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();

            let (object_area, top_cell, middle_cell, bottom_cell) = self.split_frame(area);

            self.draw_title(frame, top_cell);

            if renderer_info.selected_pane == Panes::Renderer {
                self.draw_renderer_info(frame, middle_cell, renderer_info);
            } else if renderer_info.selected_pane == Panes::Object {
                self.draw_object_info(frame, middle_cell, object_info);
            }

            self.draw_keybinds(frame, bottom_cell);

            self.draw_object(frame, object_area, buffer);
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
        let block = Paragraph::new(vec![
            Line::styled("Frames info", Style::default().add_modifier(Modifier::BOLD)),
            Line::styled(
                format!(
                    "FPS: {} | Frametime: {}ms",
                    renderer_info.fps, renderer_info.delta_time
                ),
                Style::default(),
            ),
            Line::styled("Input info", Style::default().add_modifier(Modifier::BOLD)),
            Line::styled(
                format!(
                    "Clicked: X:{}, Y:{}",
                    renderer_info.clicked_x, renderer_info.clicked_y
                ),
                Style::default(),
            ),
            Line::styled(
                format!(
                    "Drag direction: X:{}, Y:{}, Z:{}",
                    renderer_info.drag_dir.x, renderer_info.drag_dir.y, renderer_info.drag_dir.z
                ),
                Style::default(),
            ),
            Line::styled(
                format!("Is dragging?: {}", renderer_info.is_dragging),
                Style::default(),
            ),
        ])
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_object_info(&self, frame: &mut Frame, area: Rect, object_info: &ObjectInfo) {
        let block = Paragraph::new(vec![
            Line::styled(
                "Rotation info",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Line::styled(
                format!(
                    "Rotation velocity: Pitch:{}, Yaw:{}",
                    object_info.rotation_velocity.x, object_info.rotation_velocity.y
                ),
                Style::default(),
            ),
        ])
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_object(&self, frame: &mut Frame, area: Rect, buffer: &Vec<char>) {
        let frame_buffer = frame.buffer_mut();

        for y in 1..self.render_area.height - 1 {
            for x in 1..self.render_area.width - 1 {
                let screen_x = area.x + x as u16;
                let screen_y = area.y + y as u16;

                if let Some(cell) = frame_buffer.cell_mut((screen_x, screen_y)) {
                    cell.set_char(buffer[(x + y * self.render_area.width) as usize]);
                }
            }
        }

        let block = Block::default().borders(Borders::ALL).title_bottom(
            Line::from(format!(
                " {} x {} ",
                self.render_area.width, self.render_area.height
            ))
            .right_aligned(),
        );
        frame.render_widget(block, area);
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

        let info_block = Paragraph::new(vec![
            Line::styled("Quit", Style::default()),
            Line::styled("Swap displayed data", Style::default()),
        ])
        .alignment(Alignment::Left)
        .block(block.clone());

        let key_block = Paragraph::new(vec![
            Line::styled("Esc or Q", Style::default()),
            Line::styled("A or D", Style::default()),
        ])
        .alignment(Alignment::Right)
        .block(block);

        frame.render_widget(info_block, area);
        frame.render_widget(key_block, area);
    }
}
