use crate::camera::Camera;
use crate::shape::Shape;
use crate::types::{Hit, Light, Ray, find_first_hit};
use glam::Vec3;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub enum RenderMode {
    Normals,
    Raycast,
    Raytrace,
    Pathtracing,
}

pub fn draw_frame(
    frame_buffer: &mut Vec<f32>,
    width: u32,
    height: u32,
    render_mode: RenderMode,
    camera: &Camera,
    light: &Vec<Light>,
    shapes: &Vec<Shape>,
) -> u32 {
    let width = width as usize;
    let height = height as usize;

    let mut rng: SmallRng = SmallRng::from_os_rng();

    let samples = match render_mode {
        RenderMode::Pathtracing => 10,
        _ => 1,
    };

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3;

            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let ray = camera.generate_ray(x as f32 / width as f32, y as f32 / height as f32);

                let best_hit = find_first_hit(shapes.iter().map(|s| s.intersect(&ray)));

                color += match render_mode {
                    RenderMode::Normals => render_normals(best_hit),
                    RenderMode::Raycast => raycast(&camera, &ray, best_hit),
                    RenderMode::Raytrace => raytrace(light, shapes, &ray, best_hit),
                    RenderMode::Pathtracing => pathtrace(shapes, &ray, best_hit, &mut rng),
                };
            }

            frame_buffer[idx] += color.x;
            frame_buffer[idx + 1] += color.y;
            frame_buffer[idx + 2] += color.z;
        }
    }
    samples
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

fn pathtrace(shapes: &Vec<Shape>, ray: &Ray, best_hit: Option<Hit>, rng: &mut SmallRng) -> Vec3 {
    best_hit.map_or(Vec3::new(0.0, 0.0, 0.0), |hit| {
        let mut ray_light = Vec3::new(1.0, 1.0, 1.0);
        let mut incoming_light = Vec3::new(0.0, 0.0, 0.0);
        let mut cur_hit = hit;
        let mut cur_ray = *ray;
        for _ in 0..5 {
            let mut new_d = sample_random_on_sphere(rng);
            let cos_n_d = new_d.dot(cur_hit.normal);
            if cos_n_d < 0.0 {
                new_d -= 2.0 * cos_n_d * cur_hit.normal; //new_d.reflect(h.normal)
            }

            if (cur_hit.material.ambient > 0.0) {
                incoming_light += ray_light * cur_hit.material.ambient * cur_hit.material.color;
                break;
            }
            ray_light *= cur_hit.material.color * new_d.dot(cur_hit.normal) * 2.0;

            let new_origin = cur_hit.point(&cur_ray) + cur_hit.normal * 0.001;
            cur_ray = Ray {
                origin: new_origin,
                direction: new_d,
            };
            cur_hit = match find_first_hit(shapes.iter().map(|s| s.intersect(&cur_ray))) {
                Some(h) => h,
                None => break,
            };
        }
        incoming_light
    })
}

pub fn sample_random_on_sphere(rng: &mut SmallRng) -> Vec3 {
    //z: latitude of the sphere
    let z: f32 = rng.random_range(-1.0..=1.0);
    let phi: f32 = rng.random_range(0.0..=std::f32::consts::TAU);
    // Convert spherical to Cartesian.
    let r_xy = (1.0f32 - z * z).sqrt(); // circle radius at latitude z
    Vec3::new(r_xy * phi.cos(), r_xy * phi.sin(), z)
}
