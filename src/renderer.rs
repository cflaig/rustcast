use crate::camera::Camera;
use crate::shape::Shape;
use crate::types::{Hit, Light, Ray, find_first_hit};
use glam::Vec3;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use std::fmt::Debug;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(PartialEq, IntoStaticStr, EnumIter, Clone, Copy)]
pub enum RenderMode {
    Normals,
    Raycast,
    Raytrace,
    Pathtracing,
}
#[derive(Copy, Clone, Debug)]
struct RenderPixel {
    color: Vec3,
    sample_count: u32,
}
pub struct Renderer {
    frame_buffer: Vec<RenderPixel>,
    width: usize,
    height: usize,
    render_mode: RenderMode,
    camera: Camera,
    light: Vec<Light>,
    shapes: Vec<Shape>,
}

impl Renderer {
    pub fn new(
        width: usize,
        height: usize,
        render_mode: RenderMode,
        camera: Camera,
        light: Vec<Light>,
        shapes: Vec<Shape>,
    ) -> Self {
        let frame_buffer = vec![
            RenderPixel {
                color: Vec3::new(0.0, 0.0, 0.0),
                sample_count: 0,
            };
            width * height
        ];
        Self {
            frame_buffer,
            width,
            height,
            render_mode,
            camera,
            light,
            shapes,
        }
    }
    pub fn render(&mut self) -> Vec<Vec3> {
        let samples = match self.render_mode {
            RenderMode::Pathtracing => 10,
            _ => 1,
        };

        self.frame_buffer
            .par_chunks_mut(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                let mut rng: SmallRng = SmallRng::from_os_rng();

                for x in 0..self.width {
                    let mut color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples {
                        let ray = self.camera.generate_ray(
                            x as f32 / self.width as f32,
                            y as f32 / self.height as f32,
                        );

                        let best_hit =
                            find_first_hit(self.shapes.iter().map(|s| s.intersect(&ray)));

                        color += match self.render_mode {
                            RenderMode::Normals => render_normals(best_hit),
                            RenderMode::Raycast => raycast(&self.camera, &ray, best_hit),
                            RenderMode::Raytrace => {
                                raytrace(&self.light, &self.shapes, &ray, best_hit)
                            }
                            RenderMode::Pathtracing => {
                                pathtrace(&self.shapes, &ray, best_hit, &mut rng)
                            }
                        };
                    }

                    row[x].color += color;
                    row[x].sample_count += samples as u32;
                }
            });

        self.frame_buffer
            .iter()
            .map(|p| p.color / (p.sample_count as f32))
            .collect::<Vec<Vec3>>()
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
                    let p = hit.point(&ray) + hit.normal * ORIGIN_BIAS;
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

            if cur_hit.material.emission > 0.0 {
                incoming_light += ray_light * cur_hit.material.emission * cur_hit.material.color;
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
