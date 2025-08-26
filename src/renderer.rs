use crate::camera::Camera;
use crate::shape::Shape;
use crate::types::{Hit, Light, Ray, find_first_hit};
use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
    Normals,
    Raycast,
    Raytrace,
    Pathtracing,
}

pub fn draw_frame(
    frame: &mut [u8],
    width: u32,
    height: u32,
    render_mode: RenderMode,
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

            let color = match render_mode {
                RenderMode::Normals => render_normals(best_hit),
                RenderMode::Raycast => raycast(&camera, &ray, best_hit),
                RenderMode::Raytrace => raytrace(light, shapes, &ray, best_hit),
                RenderMode::Pathtracing => pathtrace(shapes, &ray, best_hit),
            };

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

fn render_normals(best_hit: Option<Hit>) -> Vec3 {
    best_hit.map_or(Vec3::new(0.0, 0.0, 0.0), |hit| {
        hit.normal + Vec3::new(1.0, 1.0, 1.0)
    })
}
fn raycast(camera: &Camera, ray: &Ray, best_hit: Option<Hit>) -> Vec3 {
    best_hit.map_or(Vec3::new(0.0, 0.0, 0.0), |hit| {
        let l = (camera.pos - hit.point(&ray)).normalize();
        let brightness = l.dot(hit.normal).max(0.0);
        hit.material.ambient * hit.material.color
            + (1.0 - hit.material.ambient) * brightness * hit.material.color
    })
}

fn raytrace(light: &Vec<Light>, shapes: &Vec<Shape>, ray: &Ray, best_hit: Option<Hit>) -> Vec3 {
    const ORIGIN_BIAS: f32 = 1e-4;
    const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    best_hit.map_or(Vec3::new(0.0, 0.0, 0.0), |hit| {
        hit.material.ambient * hit.material.color
            + light
                .iter()
                .map(|l| {
                    let mut p = hit.point(&ray) + hit.normal * ORIGIN_BIAS;
                    let distance = (p - l.position).length();
                    let light_ray = Ray {
                        origin: p,
                        direction: (l.position - p) / distance,
                    };

                    find_first_hit(shapes.iter().map(|s| s.intersect(&light_ray)))
                        .filter(|h| h.t > ORIGIN_BIAS && h.t < distance - ORIGIN_BIAS)
                        .map_or_else(
                            || {
                                let light = light_ray.direction.dot(hit.normal).max(0.0) * l.color;
                                (1.0 - hit.material.ambient) * light * hit.material.color
                            },
                            |_| BLACK,
                        )
                })
                .reduce(|a, b| a + b)
                .unwrap_or(BLACK)
    })
}

fn pathtrace(shapes: &Vec<Shape>, ray: &Ray, best_hit: Option<Hit>) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}
