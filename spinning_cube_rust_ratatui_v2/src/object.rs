// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   object.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lewislee <lewlee@student.42.fr>            +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/10/11 18:38:22 by lewislee          #+#    #+#             //
//   Updated: 2026/10/11 22:38:06 by lewislee         ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::matrix::Matrix3;
use crate::vec3::Vec3;

pub struct Object {
    pub rotation_matrix: Matrix3, // Not used as of now
    pub rotation_velocity: Vec3,

    // Temp variables to remove in the future
    pub width: f32,
    pub increment_speed: f32,
}

impl Object {
    pub fn default() -> Self {
        Self {
            rotation_matrix: Matrix3::identity(),
            rotation_velocity: Vec3::default(),
            width: 20.0,
            increment_speed: 0.3,
        }
    }

    pub fn apply_rotation_on_point(&mut self, point: &Vec3) -> Vec3 {
        let m = &self.rotation_matrix.m;

        let x = m[0][0] * point.x + m[0][1] * point.y + m[0][2] * point.z;
        let y = m[1][0] * point.x + m[1][1] * point.y + m[1][2] * point.z;
        let z = m[2][0] * point.x + m[2][1] * point.y + m[2][2] * point.z;

        Vec3::default_vec(x, y, z)
    }

    pub fn rotate(&mut self, rotation_amount: &Vec3) {
        let delta_matrix_x = Matrix3::rotation_x(rotation_amount.x);
        let delta_matrix_y = Matrix3::rotation_y(rotation_amount.y);

        // need to understand how using this method works
        let combined_delta = delta_matrix_y.multiply(&delta_matrix_x);
        self.rotation_matrix = self.rotation_matrix.multiply(&combined_delta);
    }
}
