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
    object_rotation_speed: f32,

    buffer: Vec<char>, // Change to use image/frame instead
    z_buffer: Vec<f32>,

    is_auto_rotating: bool,
    max_depth: u32,
    min_depth: u32,
    sum_depth: u32,
    clicked_raymarch_depth: u32,
    clicked_raymarch_distance: f32,
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
            object_rotation_speed: 0.005,
            buffer: vec![],
            z_buffer: vec![],
            is_auto_rotating: true,
            max_depth: 0,
            min_depth: 0,
            sum_depth: 0,
            clicked_raymarch_depth: 0,
            clicked_raymarch_distance: 0.0,
        };
        let render_width = app.display_manager.obj_render_area.width;
        let render_height = app.display_manager.obj_render_area.height;

        app.buffer = vec![' '; (render_width * render_height) as usize];
        app.z_buffer = vec![0.0; (render_width * render_height) as usize];
        app
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
            max_depth: self.max_depth,
            min_depth: self.min_depth,
            sum_depth: self.sum_depth,
            clicked_raymarch_depth: self.clicked_raymarch_depth,
            clicked_raymarch_distance: self.clicked_raymarch_distance,
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

        let is_x_in_range = self.object.rotation_velocity.x.abs() < 0.01;
        let is_y_in_range = self.object.rotation_velocity.y.abs() < 0.01;

        if is_x_in_range == true && is_y_in_range == true {
            return true;
        }

        false
    }

    fn handle_object_rotation(&mut self) {
        if self.input_handler.is_dragging == true {
            self.object.rotation_velocity = self.input_handler.drag_dir;
        }

        let rotate_amount: Vec3;
        self.is_auto_rotating = self.should_auto_rotate();
        if self.is_auto_rotating == false {
            rotate_amount = self.object.rotation_velocity * self.object_rotation_speed;
            self.object.rotation_velocity *= 0.996;
        } else {
            rotate_amount = Vec3::default_vec(0.003, 0.003, 0.003);
        }

        self.object.rotate(&rotate_amount);
    }

    fn calculate_update_object_v2(&mut self) {
        let ray_origin = Vec3::default_vec(0.0, 0.0, -3.0);
        let screen_width = self.display_manager.obj_render_area.width as f32;
        let screen_height = self.display_manager.obj_render_area.height as f32;
        self.sum_depth = 0;

        for i in 0..screen_height as i32 {
            for j in 0..screen_width as i32 {
                let u = (j as f32 - 0.5 * screen_width) / screen_height;
                let v = (i as f32 - 0.5 * screen_height) / screen_height * 2.0;
                let mut color = Vec3::default_vec(0.0, 0.0, 0.0);

                let ray_dir = Vec3::default_vec(u, v, 1.0).normalize();

                let (depth, distance) = self.ray_march_algo(ray_origin, ray_dir);
                if depth > self.max_depth {
                    self.max_depth = depth;
                    if self.min_depth == 0 {
                        self.min_depth = depth;
                    }
                } else if depth > 0 && depth < self.min_depth {
                    self.min_depth = depth;
                }
                self.sum_depth += depth;

                if distance < 100.0 {
                    let point = ray_origin + ray_dir * distance;
                    let normal = self.get_normal(point);

                    let light_position = Vec3::default_vec(1.0, 2.0, -3.0);
                    let light_direction = (light_position - point).normalize();

                    let diffuse = f32::max(normal.dot(light_direction), 0.0);

                    color = Vec3::default_vec(0.9, 0.2, 0.3) * diffuse + Vec3::default_n(0.5);
                }

                color.x = color.x * 10.0;
                color.x = color.x.clamp(0.0, 9.0);
                let luminance_index = color.x as usize;
                let index: i32 = (j + i * screen_width as i32) as i32;
                self.buffer[index as usize] = " .,-~:;#$@".as_bytes()[luminance_index] as char;
            }
        }
    }

    fn get_normal(&mut self, point: Vec3) -> Vec3 {
        let distance = self.find_closest_obj(point);
        let e_x = 0.01;
        let e_y = 0.0;

        let e_conf1 = Vec3::default_vec(e_x, e_y, e_y);
        let e_conf2 = Vec3::default_vec(e_y, e_x, e_y);
        let e_conf3 = Vec3::default_vec(e_y, e_y, e_x);

        let normal = distance
            - Vec3::default_vec(
                self.find_closest_obj(point - e_conf1),
                self.find_closest_obj(point - e_conf2),
                self.find_closest_obj(point - e_conf3),
            );

        normal.normalize()
    }

    fn ray_march_algo(&mut self, ray_ori: Vec3, ray_dir: Vec3) -> (u32, f32) {
        let mut distance_traveled = 0.0;
        let max_step = 100;
        let surface_threshold = 0.005;
        let max_distance = 100.0;
        let mut depth = 0;

        while depth < max_step {
            let point = ray_ori + ray_dir * distance_traveled;

            let distance_to_closest_obj = self.find_closest_obj(point);

            distance_traveled += distance_to_closest_obj;

            if distance_to_closest_obj < surface_threshold || distance_traveled > max_distance {
                break;
            }
            depth += 1;
        }

        (depth, distance_traveled)
    }

    fn find_closest_obj(&mut self, point: Vec3) -> f32 {
        let p = self.object.apply_rotation_on_point(&point);
        let cube_p = self.object.apply_rotation_on_point(&Vec3::default());

        let local_p = p - cube_p;

        let cube_size = Vec3::default_n(0.75);
        self.sd_cube(local_p, cube_size)
    }

    fn sd_cube(&self, point: Vec3, cube_size: Vec3) -> f32 {
        let q = point.abs() - cube_size;

        Vec3::max(q, Vec3::default()).length() + q.x.max(q.y.max(q.z)).min(0.0)
    }

    fn handle_click(&mut self) {
        if self.input_handler.clicked == true {
            let screen_width = self.display_manager.obj_render_area.width as f32;
            let screen_height = self.display_manager.obj_render_area.height as f32;
            let u = (self.input_handler.clicked_x as f32 - 0.5 * screen_width) / screen_height;
            let v =
                (self.input_handler.clicked_y as f32 - 0.5 * screen_height) / screen_height * 2.0;
            let ray_origin = Vec3::default_vec(0.0, 0.0, -3.0);
            let ray_dir = Vec3::default_vec(u, v, 1.0).normalize();

            (self.clicked_raymarch_depth, self.clicked_raymarch_distance) =
                self.ray_march_algo(ray_origin, ray_dir);
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.last_frame = Instant::now();

        loop {
            self.input_handler.get_input()?;

            if self.input_handler.should_close == true {
                break;
            }
            self.handle_click();

            self.calculate_fps();

            self.clear_buffers();

            self.calculate_update_object_v2();

            self.print_frame(terminal)?;

            self.handle_object_rotation();
        }

        Ok(())
    }
}
