use crate::vec3::Vec3;
use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect, Alignment},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

pub struct RendererInfo {
    pub fps: u32,
    pub delta_time: u128,
    pub clicked_x: u16,
    pub clicked_y: u16,
    pub drag_dir: Vec3,
    pub is_dragging: bool,
}

pub struct ObjectInfo {}

#[derive(PartialEq)]
pub enum Panes {
    Renderer,
    Object,
}

pub struct DisplayManager {
    pub currently_selected_info: Panes,
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

        let columns = Layout::horizontal([
            Constraint::Percentage(20), // Col 0 (Left)
            Constraint::Percentage(80), // Col 1
        ])
        .split(container_rect);

        Self {
            currently_selected_info: Panes::Renderer,
            render_area: columns[1],
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

            if self.currently_selected_info == Panes::Renderer {
                self.draw_renderer_info(frame, middle_cell, renderer_info);
            } else if self.currently_selected_info == Panes::Object {
                self.draw_object_info(frame, middle_cell, object_info);
            }

            self.draw_keybinds(frame, bottom_cell);

            self.draw_object(frame, object_area, buffer);
        })?;
        Ok(())
    }

    fn split_frame(&self, area: Rect) -> (Rect, Rect, Rect, Rect) {
        let columns = Layout::horizontal([
            Constraint::Percentage(20), // Col 0 (Left)
            Constraint::Percentage(80), // Col 1
        ])
        .split(area);

        // 2. Split Row 0 into 4 columns -> Column 0 is Top-Left
        let first_column = Layout::vertical([
            Constraint::Percentage(15), // Row 0 (Top)
            Constraint::Percentage(65), // Row 1
            Constraint::Percentage(20), // Row 2 (Bottom)
        ])
        .split(columns[0]);

        (
            columns[1],
            first_column[0],
            first_column[1],
            first_column[2],
        )
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
        let render_info_text = format!(
            "Resolution: {}x{}\nFPS: {}\nMS per/frame: {}\nClicked: X:{}, Y:{}\nDrag dir: X:{}, Y:{}, Z:{}\nIs dragging: {}",
            self.render_area.width,
            self.render_area.height,
            renderer_info.fps,
            renderer_info.delta_time,
            renderer_info.clicked_x,
            renderer_info.clicked_y,
            renderer_info.drag_dir.x,
            renderer_info.drag_dir.y,
            renderer_info.drag_dir.z,
            renderer_info.is_dragging
        );
        let block = Paragraph::new(render_info_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_object_info(&self, frame: &mut Frame, area: Rect, object_info: &ObjectInfo) {
        let render_info_text = format!("Something");

        let block = Paragraph::new(render_info_text)
            .block(
                Block::default()
                    .title("Object information")
                    .borders(Borders::ALL),
            )
            .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_object(
        &self,
        frame: &mut Frame,
        area: Rect,
        buffer: &Vec<char>,
    ) {
        let frame_buffer = frame.buffer_mut();

        for y in 0..self.render_area.height {
            for x in 0..self.render_area.width {
                let screen_x = area.x + x as u16;
                let screen_y = area.y + y as u16;

                if let Some(cell) = frame_buffer.cell_mut((screen_x, screen_y)) {
                    cell.set_char(buffer[(x + y * self.render_area.width) as usize]);
                }
            }
        }
    }

    fn draw_keybinds(&self, frame: &mut Frame, area: Rect) {
        let keybind_text = "Quit: Esc or Q";

        let block = Paragraph::new(keybind_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default());
        frame.render_widget(block, area);
    }
}
