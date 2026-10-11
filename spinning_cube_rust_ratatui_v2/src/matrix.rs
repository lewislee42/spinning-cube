

pub struct Matrix3 {
    pub m: [[f32; 3]; 3]
}

impl Matrix3 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            m: [
                [1.0, 0.0, 0.0],
                [0.0,   c,  -s],
                [0.0,   s,   c]
            ]
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            m: [
                [  c, 0.0,   s],
                [0.0, 1.0, 0.0],
                [ -s, 0.0,   c]
            ]
        }
    }

    pub fn multiply(&self, other: &Matrix3) -> Matrix3 {
        let mut result = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        Matrix3 { m: result }
    }
}
