use glam::{Quat, Vec3};

use crate::types::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub up: Vec3,
    pub right: Vec3,
    pub look_dir: Vec3,
    pub pos: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, look_at: Vec3, up: Vec3, fov: f32) -> Self {
        let look_dir = (look_at - pos).normalize();
        let up = (up - look_dir.dot(up) * look_dir).normalize() * fov;
        let right = look_dir.cross(up);
        Camera {
            up,
            right,
            look_dir,
            pos,
        }
    }
    pub fn generate_ray(&self, x: f32, y: f32) -> Ray {
        let direction =
            ((x - 0.5f32) * self.right + (1f32 - y - 0.5f32) * self.up + self.look_dir).normalize();
        Ray {
            origin: self.pos,
            direction,
        }
    }

    pub fn move_along_up(&mut self, amount: f32) {
        let dir = self.up.normalize();
        self.pos += dir * amount;
    }

    pub fn move_along_right(&mut self, amount: f32) {
        let dir = self.right.normalize();
        self.pos += dir * amount;
    }

    pub fn move_along_look(&mut self, amount: f32) {
        let dir = self.look_dir.normalize();
        self.pos += dir * amount;
    }

    fn fov(&self) -> f32 {
        self.up.length()
    }

    fn re_orthonormalize(&mut self) {
        // Preserve fov magnitude encoded in up/right lengths
        let fov = self.fov().max(1e-6);
        let l = self.look_dir.normalize();
        // Make up orthogonal to look
        let mut u = (self.up - l * self.up.dot(l)).normalize();
        let mut r = l.cross(u).normalize();
        // Rescale up/right back to fov
        u *= fov;
        r *= fov;
        self.look_dir = l;
        self.up = u;
        self.right = r;
    }

    pub fn yaw(&mut self, angle: f32) {
        // Rotate look and right around up axis
        let axis = self.up.normalize();
        let q = Quat::from_axis_angle(axis, angle);
        self.look_dir = q * self.look_dir;
        self.right = q * self.right;
        self.re_orthonormalize();
    }

    pub fn pitch(&mut self, angle: f32) {
        // Rotate look and up around right axis
        let axis = self.right.normalize();
        let q = Quat::from_axis_angle(axis, angle);
        self.look_dir = q * self.look_dir;
        self.up = q * self.up;
        self.re_orthonormalize();
    }
}
