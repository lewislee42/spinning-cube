use crate::vec3::Vec3;

pub struct Object {
    pub position: Vec3,
    pub rotation: Vec3, // Not used as of now

    // Temp variables to remove in the future
    pub width: f32,
    k_1: f32,
    pub increment_speed: f32,
}

impl Object {
    pub fn default() -> Self {
        Self {
            position: Vec3::default(),
            rotation: Vec3::default(),
            width: 20.0,
            k_1: 40.0,
            increment_speed: 0.3,
        }
    }

    pub fn default_position(position: Vec3) -> Self {
        Self {
            position: position,
            ..Self::default()
        }
    }

    pub fn default_pos_rot(position: Vec3, rotation: Vec3) -> Self {
        Self {
            position: position,
            rotation: rotation,
            ..Self::default()
        }
    }

    fn rotate_x(&mut self, point_on_surface: &Vec3) -> f32 {
        point_on_surface.y * self.rotation.x.sin() * self.rotation.y.sin() * self.rotation.z.cos()
            - point_on_surface.z
                * self.rotation.x.cos()
                * self.rotation.y.sin()
                * self.rotation.z.cos()
            + point_on_surface.y * self.rotation.x.cos() * self.rotation.z.sin()
            + point_on_surface.z * self.rotation.x.sin() * self.rotation.z.sin()
            + point_on_surface.x * self.rotation.y.cos() * self.rotation.z.cos()
    }

    fn rotate_y(&mut self, point_on_surface: &Vec3) -> f32 {
        point_on_surface.y * self.rotation.x.cos() * self.rotation.z.cos()
            + point_on_surface.z * self.rotation.x.sin() * self.rotation.z.cos()
            - point_on_surface.y
                * self.rotation.x.sin()
                * self.rotation.y.sin()
                * self.rotation.z.sin()
            + point_on_surface.z
                * self.rotation.x.cos()
                * self.rotation.y.sin()
                * self.rotation.z.sin()
            - point_on_surface.x * self.rotation.y.cos() * self.rotation.z.sin()
    }

    fn rotate_z(&mut self, point_on_surface: &Vec3) -> f32 {
        point_on_surface.z * self.rotation.x.cos() * self.rotation.y.cos()
            - point_on_surface.y * self.rotation.x.sin() * self.rotation.y.cos()
            + point_on_surface.x * self.rotation.y.sin()
    }

    pub fn apply_rotation_on_point(&mut self, point_on_surface: &Vec3) -> Vec3 {
        Vec3::default_vec(
            self.rotate_x(point_on_surface),
            self.rotate_y(point_on_surface),
            self.rotate_z(point_on_surface),
        )
    }

    pub fn rotate(&mut self, rotation_amount: Vec3) {
        self.rotation.x += rotation_amount.x;
        self.rotation.y += rotation_amount.y;
        self.rotation.z += rotation_amount.z;
    }
}
