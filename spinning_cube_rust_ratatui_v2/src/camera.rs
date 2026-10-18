use crate::vec3::Vec3;

pub struct Camera {
    pub center: Vec3,
    pub direction: Vec3,

    viewport_width: f32,
    viewport_height: f32,

    focal_length: f32,
    vfov: f32,
    vup: Vec3,
    move_speed: f32,
    turn_speed: f32,

    screen_width: i16,
    screen_height: i16,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00_loc: Vec3,
}

impl Camera {
    pub fn default() -> Self {
        let mut camera = Self {
            center: Vec3::default(),
            direction: Vec3::default(),
            viewport_width: 0.0,
            viewport_height: 0.0,

            focal_length: 0.0,
            vfov: 0.0,
            vup: Vec3::default(),
            move_speed: 0.0,
            turn_speed: 0.0,

            screen_width: 0,
            screen_height: 0,

            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_00_loc: Vec3::default(),
        };

        let theta = camera.vfov.to_radians();
        let h = (theta / 2.0).tan();

        camera.update_calculation();
        camera
    }

    fn update_position() {}
    fn update_direction() {}
    fn update_calculation(&mut self) {
        let w: Vec3 = -self.direction.unit_vector();
        let u: Vec3 = self.vup.cross(w).unit_vector();
        let v: Vec3 = w.cross(u).unit_vector();

        let viewport_u: Vec3 = self.viewport_width * u;
        let viewport_v: Vec3 = self.viewport_height * -v;

        self.pixel_delta_u = viewport_u / self.screen_width as f32;
        self.pixel_delta_v = viewport_v / self.screen_height as f32;

        let viewport_upper_left =
            self.center - (w * self.focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
