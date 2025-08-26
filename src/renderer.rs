use crate::camera::Camera;
use crate::types::{find_first_hit, Light};
use glam::Vec3;
use crate::shape::Shape;

pub fn draw_frame(
    frame: &mut [u8],
    width: u32,
    height: u32,
    draw_normals: bool,
    camera: &Camera,
    light: &Vec<Light>,
    shapes: &Vec<Shape>,
) {
    let width = width as usize;
    let height = height as usize;

    let mut frame_buffer = vec![0.0f32; width * height * 3];

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3;

            let ray = camera.generate_ray(x as f32 / width as f32, y as f32 / height as f32);

            let best_hit = find_first_hit(shapes.iter().map(|s| s.intersect(&ray)));

            let color = best_hit.map_or(Vec3::new(0.0, 0.0, 0.0), |hit| {
                let l = (camera.pos - hit.point(&ray)).normalize();
                let brightness = l.dot(hit.normal).max(0.0);
                if (draw_normals) {
                    hit.normal + Vec3::new(1.0, 1.0, 1.0)
                } else {
                    hit.material.ambient * hit.material.color
                        + (1.0 - hit.material.ambient) * brightness * hit.material.color
                }
            });

            frame_buffer[idx] = color.x;
            frame_buffer[idx + 1] = color.y;
            frame_buffer[idx + 2] = color.z;
        }
    }

    let max = frame_buffer
        .clone()
        .into_iter()
        .reduce(f32::max)
        .unwrap_or(0.0);
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            let idx_f = (y * width + x) * 3;
            let m = if max > 0.0 { 255.0 / max } else { 0.0 };
            frame[idx] = (frame_buffer[idx_f] * m) as u8;
            frame[idx + 1] = (frame_buffer[idx_f + 1] * m) as u8;
            frame[idx + 2] = (frame_buffer[idx_f + 2] * m) as u8;
            frame[idx + 3] = 255;
        }
    }
}
