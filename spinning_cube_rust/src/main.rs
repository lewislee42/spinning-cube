use color_eyre::Result;
use std::thread;
use std::time::Duration;
use std::io::stdout;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

const SCREEN_WIDTH: i32 = 338;
const SCREEN_HEIGHT: i32 = 91;

struct SpinningCube {
    rotate_x: f32,
    rotate_y: f32,
    rotate_z: f32,

    cube_width: f32,

    distance_from_cam: f32,
    k_1: f32,
    increment_speed: f32,
    rotation_speed: f32,

    buffer: Vec<char>,
    z_buffer: Vec<f32>,
}

impl SpinningCube {
    fn default() -> Self {
        Self {
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
            cube_width: 20.0,
            distance_from_cam: 60.0,
            k_1: 40.0,
            increment_speed: 0.3,
            rotation_speed: 0.005,
            buffer: vec![' '; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            z_buffer: vec![0.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.rotate_x.sin() * self.rotate_y.sin() * self.rotate_z.cos()
            - k * self.rotate_x.cos() * self.rotate_y.sin() * self.rotate_z.cos()
            + j * self.rotate_x.cos() * self.rotate_z.sin()
            + k * self.rotate_x.sin() * self.rotate_z.sin()
            + i * self.rotate_y.cos() * self.rotate_z.cos()
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.rotate_x.cos() * self.rotate_z.cos()
            + k * self.rotate_x.sin() * self.rotate_z.cos()
            - j * self.rotate_x.sin() * self.rotate_y.sin() * self.rotate_z.sin()
            + k * self.rotate_x.cos() * self.rotate_y.sin() * self.rotate_z.sin()
            - i * self.rotate_y.cos() * self.rotate_z.sin()
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        k * self.rotate_x.cos() * self.rotate_y.cos()
            - j * self.rotate_x.sin() * self.rotate_y.cos()
            + i * self.rotate_y.sin()
    }

    fn calculate_luminance(&self, normal_x: f32, normal_y: f32, normal_z: f32) -> f32 {
        // Light direction normalized
        let light_dir_x: f32 = 0.0;
        let light_dir_y: f32 = 0.7071;
        let light_dir_z: f32 = -0.7071;

        let dir_x: f32 = self.calculate_x(normal_x, normal_y, normal_z);
        let dir_y: f32 = self.calculate_y(normal_x, normal_y, normal_z);
        let dir_z: f32 = self.calculate_z(normal_x, normal_y, normal_z);
        (dir_x * light_dir_x) + (dir_y * light_dir_y) + (dir_z * light_dir_z)
    }

    fn calculate_rotated_surface(
        &mut self,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        normal_x: f32,
        normal_y: f32,
        normal_z: f32,
    ) -> Result<()> {
        // Calculate the rotated position of the surface
        let translated_pos_x: f32 = self.calculate_x(pos_x, pos_y, pos_z);
        let translated_pos_y: f32 = self.calculate_y(pos_x, pos_y, pos_z);
        let translated_pos_z: f32 = self.calculate_z(pos_x, pos_y, pos_z) + self.distance_from_cam;

        let ooz: f32 = 1.0 / translated_pos_z;
        let screen_pos_x: i32 =
            (SCREEN_WIDTH as f32 / 2.0 + self.k_1 * ooz * translated_pos_x * 2.0) as i32;
        let screen_pos_y: i32 =
            (SCREEN_HEIGHT as f32 / 2.0 + self.k_1 * ooz * translated_pos_y) as i32;

        let index: i32 = screen_pos_x + screen_pos_y * SCREEN_WIDTH;

        if index >= 0 && index < SCREEN_WIDTH * SCREEN_HEIGHT {
            if ooz > self.z_buffer[index as usize] {
                let luminance: f32 = self.calculate_luminance(normal_x, normal_y, normal_z);

                let l: f32 = if luminance > 0.0 { luminance } else { 0.0 };

                let mut luminance_index: i32 = (l * 10.0) as i32;
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

    fn clear_buffers(&mut self) -> Result<()> {
        self.buffer.fill(' ');
        self.z_buffer.fill(0.0);
        Ok(())
    }

    fn print_cube(&self) -> Result<()> {
        for i in 0..SCREEN_WIDTH * SCREEN_HEIGHT {
            print!("{}", self.buffer[i as usize]);

            if i % SCREEN_WIDTH == 0 {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn should_close(&self) -> Result<bool> {
        let mut should_close: bool = false;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == event::KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => should_close = true,
                        _ => {}
                    }
                }
            }
        }

        Ok(should_close)
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.should_close()? {
                break;
            }

            self.clear_buffers()?;

            let mut i: f32 = -self.cube_width;

            while i < self.cube_width {
                let mut j: f32 = -self.cube_width;

                while j < self.cube_width {
                    self.calculate_rotated_surface(i, j, self.cube_width, 0.0, 0.0, 1.0)?; // Front  (0, 0, 1)
                    self.calculate_rotated_surface(-self.cube_width, j, i, -1.0, 0.0, 0.0)?; // Left   (-1, 0, 0)
                    self.calculate_rotated_surface(self.cube_width, j, -i, 1.0, 0.0, 0.0)?; // Right  (1, 0, 0)
                    self.calculate_rotated_surface(i, j, -self.cube_width, 0.0, 0.0, -1.0)?; // Back   (0, 0, -1)
                    self.calculate_rotated_surface(i, self.cube_width, -j, 0.0, 1.0, 0.0)?; // Top    (0, 1, 0)
                    self.calculate_rotated_surface(i, -self.cube_width, j, 0.0, -1.0, 0.0)?; // Bottom (0, -1, 0)
                    j += self.increment_speed;
                }

                i += self.increment_speed;
            }

            self.print_cube()?;

            self.rotate_x += self.rotation_speed;
            self.rotate_y += self.rotation_speed;
            self.rotate_z += self.rotation_speed;

            thread::sleep(Duration::from_micros(1500));
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut spinning_cube = SpinningCube::default();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    spinning_cube.run()?;

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
