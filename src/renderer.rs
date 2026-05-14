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
                                raytrace(&self.light, &self.shapes, &ray, best_hit, 10)
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
        let texture = hit.texture;
        let material = texture.material_at(&hit, &ray);
        material.ambient * material.color
            + (1.0 - material.ambient) * brightness * material.color
    })
}

const ORIGIN_BIAS: f32 = 1e-4;

fn raytrace(light: &Vec<Light>, shapes: &Vec<Shape>, ray: &Ray, best_hit: Option<Hit>, max_depth: usize) -> Vec3 {

    const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    if max_depth == 0 {
        return BLACK;
    }

    best_hit.map_or(Vec3::new(0.0, 0.0, 0.0), |hit| {
        let texture = hit.texture;
        let material = texture.material_at(&hit, &ray);

        if material.reflection > 0.0 {
            let reflected_ray = Ray {
                origin: hit.point(&ray) + hit.normal * ORIGIN_BIAS,
                direction: ray.direction - 2.0 * ray.direction.dot(hit.normal) * hit.normal //ray.direction.reflect(hit.normal)
            };
            let reflected_hit = find_first_hit(shapes.iter().map(|s| s.intersect(&reflected_ray)));
            return raytrace(light, shapes, &reflected_ray, reflected_hit, max_depth-1);
        }
        if material.transparency > 0.0 {
            let eta = 1.0/material.ior;
            let offset = if ray.direction.dot(hit.normal) < 0.0 { -ORIGIN_BIAS } else { ORIGIN_BIAS };

           return  match refract_ray(&ray.direction, hit.normal, eta ).map(|r| {
                let refracted_ray = Ray {
                    origin: hit.point(&ray) + hit.normal * offset,
                    direction: r,
                };

                let refracted_hit = find_first_hit(shapes.iter().map(|s| s.intersect(&refracted_ray)));
                raytrace(light, shapes, &refracted_ray, refracted_hit, max_depth-1)
            }) {
                None => {
                    let reflected_ray = Ray {
                        origin: hit.point(&ray) + hit.normal * ORIGIN_BIAS,
                        direction: ray.direction - 2.0 * ray.direction.dot(hit.normal) * hit.normal //ray.direction.reflect(hit.normal)
                    };
                    let reflected_hit = find_first_hit(shapes.iter().map(|s| s.intersect(&reflected_ray)));
                    raytrace(light, shapes, &reflected_ray, reflected_hit, max_depth-1)
                },
                Some(v) => v
            };
        }
        material.ambient * material.color
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
                                (1.0 - material.ambient) * light * material.color
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
        let mut bias = ORIGIN_BIAS;
        let mut new_direction;
        let mut new_origin ;

        for bounce in 0..12 {
            let material = cur_hit.texture.material_at(&cur_hit, &cur_ray);
            if material.emission > 0.0 {
                incoming_light += ray_light * material.emission * material.color;
                break;
            }

            if material.reflection > 0.0 {
                new_direction = cur_ray.direction - 2.0 * cur_ray.direction.dot(cur_hit.normal) * cur_hit.normal;
            } else if material.transparency > 0.0 {
                let eta = 1.0/material.ior;
                bias = if cur_ray.direction.dot(cur_hit.normal) < 0.0 { -ORIGIN_BIAS } else { ORIGIN_BIAS };

                match refract_ray(&cur_ray.direction, cur_hit.normal, eta ) {
                    None => {
                        bias = ORIGIN_BIAS;
                        new_direction = cur_ray.direction - 2.0 * cur_ray.direction.dot(cur_hit.normal) * cur_hit.normal;
                    },
                    Some(v) => {
                        new_direction = v;
                    }
                };
            } else {
                new_direction = sample_random_on_sphere(rng);
                let cos_n_d = new_direction.dot(cur_hit.normal);
                if cos_n_d < 0.0 {
                    new_direction -= 2.0 * cos_n_d * cur_hit.normal; //new_d.reflect(h.normal)
                }
                ray_light *= material.color * new_direction.dot(cur_hit.normal) * 2.0;
            }

            if bounce > 2 {
                let russian_roulette_probability = ray_light.max_element();
                if rng.random_range(0.0..1.0) > russian_roulette_probability {
                    break;
                }
                ray_light /= russian_roulette_probability;
            }

            new_origin = cur_hit.point(&cur_ray) + cur_hit.normal * bias;
            cur_ray = Ray {
                origin: new_origin,
                direction: new_direction,
            };
            cur_hit = match find_first_hit(shapes.iter().map(|s| s.intersect(&cur_ray))) {
                Some(h) => h,
                None => break,
            };
        }
        incoming_light
    })
}

pub fn refract_ray(r: &Vec3, mut n: Vec3, mut eta: f32) -> Option<Vec3> {
    let mut cos = r.dot(n);
    if cos > 0.0 {
        n = -n;
        eta = 1.0/eta;
        cos = -cos;
    }
    let k = 1.0 - eta * eta * (1.0 - cos * cos);
    if k <= 0.0 {
        None
    } else {
        Some(eta * r - (eta * cos + f32::sqrt(k))*n)
    }
}

pub fn sample_random_on_sphere(rng: &mut SmallRng) -> Vec3 {
    //z: latitude of the sphere
    let z: f32 = rng.random_range(-1.0..=1.0);
    let phi: f32 = rng.random_range(0.0..=2.0 * std::f32::consts::PI);
    // Convert spherical to Cartesian.
    let r_xy = (1.0f32 - z * z).sqrt(); // circle radius at latitude z
    Vec3::new(r_xy * phi.cos(), r_xy * phi.sin(), z)
}

pub fn sample_cosine_weighted_hemisphere(rng: &mut SmallRng, normal: Vec3) -> Vec3 {
    let s = sample_random_on_sphere(rng);
    (s + normal).normalize()
}
