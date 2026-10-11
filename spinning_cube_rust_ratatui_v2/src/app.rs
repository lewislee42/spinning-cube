use crate::object::Object;
use crate::vec3::Vec3;
use crate::input_handler::InputHandler;

use color_eyre::Result;
use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};


pub struct App {
    delta_time: Duration,
    fps: u32,
    last_frame: Instant,

    input_handler: InputHandler,

    object: Object,
    object_distance: f32,
    object_rotation_speed: f32,

    buffer: Vec<char>, // Change to use image/frame instead
    z_buffer: Vec<f32>,

    screen_width: u32,
    screen_height: u32,
}

impl App {
    pub fn default() -> Self {
        let screen_width: u32;
        let screen_height: u32;

        if let Ok((width, height)) = crossterm::terminal::size() {
            screen_width = width as u32;
            screen_height = height as u32;
        } else {
            std::process::exit(1);
        }

        Self {
            delta_time: Duration::default(),
            fps: 0,
            last_frame: Instant::now(),
            input_handler: InputHandler::default(),
            object: Object::default(),
            object_distance: 60.0,
            object_rotation_speed: 0.005,
            buffer: vec![' '; (screen_width * screen_height) as usize],
            z_buffer: vec![0.0; (screen_width * screen_height) as usize],
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }

    fn calculate_luminance(&mut self, normal: &Vec3) -> f32 {
        // Light direction normalized
        let light_dir_x: f32 = 0.0;
        let light_dir_y: f32 = 0.7071;
        let light_dir_z: f32 = -0.7071;

        let dir = self.object.apply_rotation_on_point(normal);
        (dir.x * light_dir_x) + (dir.y * light_dir_y) + (dir.z * light_dir_z)
    }

    fn calculate_screen_pos(&mut self, point: &Vec3, normal: &Vec3) -> Result<()> {
        let rotated_point = self.object.apply_rotation_on_point(point);
        let k_1 = 40.0;

        let ooz: f32 = 1.0 / (rotated_point.z + self.object_distance);
        let screen_pos_x: i32 =
            (self.screen_width as f32 / 2.0 + k_1 * ooz * rotated_point.x * 2.0) as i32;
        let screen_pos_y: i32 =
            (self.screen_height as f32 / 2.0 + k_1 * ooz * rotated_point.y) as i32;

        let index: i32 = (screen_pos_x + screen_pos_y * self.screen_width as i32) as i32;
        // println!("index: {}", index);

        if index >= 0 && index < (self.screen_width as i32 * self.screen_height as i32) as i32 {
            if ooz > self.z_buffer[index as usize] {
                let luminance: f32 = self.calculate_luminance(normal);

                let l: f32 = if luminance > 0.0 { luminance } else { 0.0 };

                let mut luminance_index: i32 = (l * 8.0) as i32;
                if luminance_index > 8 {
                    luminance_index = 8;
                }

                self.z_buffer[index as usize] = ooz;
                self.buffer[index as usize] = ".,-~:;#$@".as_bytes()[luminance_index as usize] as char;
            }
        }
        Ok(())
    }

    fn clear_buffers(&mut self) {
        self.buffer.fill(' ');
        self.z_buffer.fill(0.0);
    }

    fn draw_keybinds(&self, frame: &mut Frame, area: Rect) {
        let keybind_text = "Quit: Esc or Q";

        let block = Paragraph::new(keybind_text)
            .block(Block::default().title("Keybinds").borders(Borders::ALL))
            .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_object_info(&self, frame: &mut Frame, area: Rect) {
        let render_info_text = format!(
            "Something"
        );

        let block = Paragraph::new(render_info_text)
            .block(
                Block::default()
                    .title("Object information")
                    .borders(Borders::ALL),
            )
            .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_screen_info(&self, frame: &mut Frame, area: Rect) {
        let render_info_text = format!(
            "Resolution: {}x{}\nFPS: {}\nMS per/frame: {}\nClicked: X:{}, Y:{}\nDrag dir: X:{}, Y:{}, Z:{}\nIs dragging: {}",
            self.screen_width,
            self.screen_height,
            self.fps,
            self.delta_time.as_millis(),
            self.input_handler.clicked_x,
            self.input_handler.clicked_y,
            self.input_handler.drag_dir.x,
            self.input_handler.drag_dir.y,
            self.input_handler.drag_dir.z,
            self.input_handler.is_dragging
        );

        let block = Paragraph::new(render_info_text)
            .block(
                Block::default()
                    .title("Render information")
                    .borders(Borders::ALL),
            )
            .style(Style::default());
        frame.render_widget(block, area);
    }

    fn draw_cube(&self, frame: &mut Frame, area: Rect) {
        let buffer = frame.buffer_mut();

        for i in 0..self.screen_height {
            for j in 0..self.screen_width {
                let cell = &mut buffer[(j as u16, i as u16)]; // can use cell_mut too
                cell.set_symbol(&self.buffer[(j + i * self.screen_width) as usize].to_string());
            }
        }

        let block = Block::bordered();
        frame.render_widget(block, area);
    }

    fn print_frame(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();

            let columns = Layout::horizontal([
                Constraint::Percentage(20), // Col 0 (Left)
                Constraint::Percentage(20), // Col 1
                Constraint::Percentage(20), // Col 2
                Constraint::Percentage(20), // Col 3
                Constraint::Percentage(20), // Col 4 (Right)
            ])
            .split(area);

            // 2. Split Row 0 into 4 columns -> Column 0 is Top-Left
            let first_column = Layout::vertical([
                Constraint::Percentage(40), // Row 0 (Top)
                Constraint::Percentage(30), // Row 1
                Constraint::Percentage(30), // Row 2 (Bottom)
            ])
            .split(columns[0]);

            let top_cell = first_column[0];
            let middle_cell = first_column[1];
            let bottom_cell = first_column[2];

            self.draw_cube(frame, area);
            self.draw_screen_info(frame, top_cell);
            self.draw_object_info(frame, middle_cell);
            self.draw_keybinds(frame, bottom_cell);
        })?;
        Ok(())
    }

    fn calculate_fps(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame);
        self.last_frame = now;
        let dt_seconds = self.delta_time.as_secs_f32();
        self.fps = if dt_seconds > 0.0 {
            (1.0 / dt_seconds) as u32
        } else {
            0
        };
    }

    fn handle_object_rotation(&mut self) {
        let rotate_amount: Vec3;
        if self.input_handler.is_dragging == true {
            self.object.rotation_velocity = self.input_handler.drag_dir;
        }
        rotate_amount = self.object.rotation_velocity * self.object_rotation_speed;
         //   rotate_amount = Vec3::default_vec(self.object.rotation_speed, self.object.rotation_speed, self.object.rotation_speed);
        
        self.object.rotate(&rotate_amount);
     
        self.object.rotation_velocity *= 0.996;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.last_frame = Instant::now();

        loop {
            self.input_handler.get_input()?;

            if self.input_handler.should_close == true {
                break;
            }

            self.calculate_fps();

            self.clear_buffers();


            let mut i: f32 = -self.object.width;

            while i < self.object.width {
                let mut j: f32 = -self.object.width;

                while j < self.object.width {
                    self.calculate_screen_pos(
                        &Vec3::default_vec(i, j, self.object.width),
                        &Vec3::default_vec(0.0, 0.0, 1.0),
                    )?; // Front  (0, 0, 1)
                    self.calculate_screen_pos(
                        &Vec3::default_vec(-self.object.width, j, i),
                        &Vec3::default_vec(-1.0, 0.0, 0.0),
                    )?; // Left   (-1, 0, 0)
                    self.calculate_screen_pos(
                        &Vec3::default_vec(self.object.width, j, -i),
                        &Vec3::default_vec(1.0, 0.0, 0.0),
                    )?; // Right  (1, 0, 0)
                    self.calculate_screen_pos(
                        &Vec3::default_vec(i, j, -self.object.width),
                        &Vec3::default_vec(0.0, 0.0, -1.0),
                    )?; // Back   (0, 0, -1)
                    self.calculate_screen_pos(
                        &Vec3::default_vec(i, self.object.width, -j),
                        &Vec3::default_vec(0.0, 1.0, 0.0),
                    )?; // Top    (0, 1, 0)
                    self.calculate_screen_pos(
                        &Vec3::default_vec(i, -self.object.width, j),
                        &Vec3::default_vec(0.0, -1.0, 0.0),
                    )?; // Bottom (0, -1, 0)
                    j += self.object.increment_speed;
                }

                i += self.object.increment_speed;
            }

            self.print_frame(terminal)?;

            self.handle_object_rotation();

            // self.object.rotate(&Vec3::default_vec(
            //     self.object_rotation_speed,
            //     self.object_rotation_speed,
            //     self.object_rotation_speed,
            // ))
        }

        Ok(())
    }
}
