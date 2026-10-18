use crate::input_handler::InputHandler;
use crate::vec3::Vec3;
use crate::{
    display_manager::{DisplayManager, ObjectInfo, RendererInfo},
    object::Object,
};

use color_eyre::Result;
use std::time::{Duration, Instant};

use ratatui::DefaultTerminal;

pub struct App {
    delta_time: Duration,
    fps: u32,
    last_frame: Instant,

    input_handler: InputHandler,
    display_manager: DisplayManager,

    object: Object,
    object_distance: f32,
    object_rotation_speed: f32,

    buffer: Vec<char>, // Change to use image/frame instead
    z_buffer: Vec<f32>,

    is_auto_rotating: bool,
}

impl App {
    pub fn default() -> Self {
        let mut app = Self {
            delta_time: Duration::default(),
            fps: 0,
            last_frame: Instant::now(),
            input_handler: InputHandler::default(),
            display_manager: DisplayManager::default(),
            object: Object::default(),
            object_distance: 60.0,
            object_rotation_speed: 0.005,
            buffer: vec![],
            z_buffer: vec![],
            is_auto_rotating: true,
        };
        let render_width = app.display_manager.obj_render_area.width;
        let render_height = app.display_manager.obj_render_area.height;

        app.buffer = vec![' '; (render_width * render_height) as usize];
        app.z_buffer = vec![0.0; (render_width * render_height) as usize];
        app
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
        let render_width = self.display_manager.obj_render_area.width;
        let render_height = self.display_manager.obj_render_area.height;

        let ooz: f32 = 1.0 / (rotated_point.z + self.object_distance);
        let screen_pos_x: i32 =
            (render_width as f32 / 2.0 + k_1 * ooz * rotated_point.x * 2.0) as i32;
        let screen_pos_y: i32 = (render_height as f32 / 2.0 + k_1 * ooz * rotated_point.y) as i32;

        let index: i32 = (screen_pos_x + screen_pos_y * render_width as i32) as i32;

        if index >= 0 && index < (render_width as i32 * render_height as i32) as i32 {
            if ooz > self.z_buffer[index as usize] {
                let luminance: f32 = self.calculate_luminance(normal);

                let l: f32 = if luminance > 0.0 { luminance } else { 0.0 };

                let mut luminance_index: i32 = (l * 8.0) as i32;
                if luminance_index > 8 {
                    luminance_index = 8;
                }

                self.z_buffer[index as usize] = ooz;
                self.buffer[index as usize] =
                    ".,-~:;#$@".as_bytes()[luminance_index as usize] as char;
            }
        }
        Ok(())
    }

    fn clear_buffers(&mut self) {
        self.buffer.fill(' ');
        self.z_buffer.fill(0.0);
    }

    fn print_frame(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let renderer_info = self.get_renderer_info();
        let object_info = self.get_object_info();
        self.display_manager
            .print_frame(terminal, &renderer_info, &object_info, &self.buffer)
    }

    fn get_renderer_info(&self) -> RendererInfo {
        RendererInfo {
            fps: self.fps,
            delta_time: self.delta_time.as_millis(),
            clicked_x: self.input_handler.clicked_x,
            clicked_y: self.input_handler.clicked_y,
            drag_dir: self.input_handler.drag_dir,
            is_dragging: self.input_handler.is_dragging,
            selected_pane: self.input_handler.selected_pane,
            is_auto_rotating: self.is_auto_rotating,
        }
    }

    fn get_object_info(&self) -> ObjectInfo {
        ObjectInfo {
            rotation_velocity: self.object.rotation_velocity,
        }
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

    fn should_auto_rotate(&mut self) -> bool {
        if self.input_handler.is_dragging == true {
            return false;
        }

        let is_x_in_range = self.object.rotation_velocity.x.abs() < 0.001;
        let is_y_in_range = self.object.rotation_velocity.y.abs() < 0.001;

        if is_x_in_range == true && is_y_in_range == true {
            return true;
        }

        false
    }

    fn handle_object_rotation(&mut self) {
        if self.input_handler.is_dragging == true {
            self.object.rotation_velocity = self.input_handler.drag_dir;
        }

        let mut rotate_amount: Vec3;
        self.is_auto_rotating = self.should_auto_rotate();
        if self.is_auto_rotating == false {
            rotate_amount = self.object.rotation_velocity * self.object_rotation_speed;
            self.object.rotation_velocity *= 0.996;
        } else {
            rotate_amount = Vec3::default_vec(0.003, 0.003, 0.003);
        }

        self.object.rotate(&rotate_amount);
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
        }

        Ok(())
    }
}
