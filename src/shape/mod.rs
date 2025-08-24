use glam::Vec3;

use crate::types::{Hit, Material, Ray, Transform, Transformable};

pub enum Shape {
    UnitBox {
        material: Material,
    },
    Sphere {
        center: Vec3,
        radius: f32,
        material: Material,
    },
    Plane {
        normal: Vec3,
        d: f32,
        material: Material,
    },
    Cylinder {
        material: Material,
    },
    Cone {
        material: Material,
    },
    TransformedShape {
        shape: Box<Shape>,
        transform: Transform,
    },
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Shape::TransformedShape { shape, transform } => {
                let transformed_ray = ray.to_local_coordinates(transform);
                shape
                    .intersect(&transformed_ray)
                    .map(|hit| hit.to_global_coordinates(transform))
            }
            Shape::UnitBox { material } => {
                let mut min = f32::MAX;
                let mut max = f32::MIN;
                let mut min_pos = 0;
                let mut max_pos = 0;

                for i in 0..3 {
                    let (t_near, t_far) = if ray.direction[i] < 0.0 {
                        (
                            (1.0 - ray.origin[i]) / ray.direction[i],
                            (-1.0 - ray.origin[i]) / ray.direction[i],
                        )
                    } else {
                        (
                            (-1.0 - ray.origin[i]) / ray.direction[i],
                            (1.0 - ray.origin[i]) / ray.direction[i],
                        )
                    };
                    if t_near > max {
                        max = t_near;
                        max_pos = i;
                    }
                    if t_far < min {
                        min = t_far;
                        min_pos = i;
                    }
                }

                if max <= min {
                    let (t, pos) = if max > 0.0 {
                        (max, max_pos)
                    } else {
                        (min, min_pos)
                    };
                    let p = ray.origin + ray.direction * t;
                    let mut n = Vec3::new(0.0, 0.0, 0.0);
                    n[pos] = 1.0f32 * p[pos].signum();
                    Some(Hit::new(t, n, *material))
                } else {
                    None
                }
            }
            Shape::Sphere {
                material,
                center,
                radius,
            } => {
                let oc = ray.origin - center;
                let b = ray.direction.dot(oc);
                let c = oc.dot(oc) - radius * radius;
                let discriminant = b * b - c;
                if discriminant < 0.0 {
                    None
                } else {
                    let t = -b - discriminant.sqrt();
                    let p = ray.origin + ray.direction * t;
                    let n = (p - center).normalize();
                    Some(Hit::new(t, n, *material))
                }
            }
            Shape::Plane {
                normal,
                d,
                material,
            } => {
                let cos = normal.dot(ray.direction);
                if cos.abs() < f32::EPSILON {
                    None
                } else {
                    let t = (d - normal.dot(ray.origin)) / cos;
                    if t < 0.0 {
                        None
                    } else {
                        Some(Hit::new(t, normal.normalize(), *material))
                    }
                }
            }
            Shape::Cylinder { material } => {
                let mut best_hit =
                    intersect_cap_with_radius_one(ray, 1.0, Vec3::new(0.0, 0.0, 1.0), material);

                if let Some(hit_side) =
                    intersect_cap_with_radius_one(ray, 0.0, Vec3::new(0.0, 0.0, -1.0), material)
                {
                    best_hit = best_hit.filter(|hit| hit.t < hit_side.t).or(Some(hit_side))
                }

                if let Some(hit_side) =
                    intersect_cylinder_infinite(ray, material).filter(test_if_hits_between_0_1(ray))
                {
                    best_hit = best_hit.filter(|hit| hit.t < hit_side.t).or(Some(hit_side))
                }

                best_hit
            }
            Shape::Cone { material } => {
                let mut best_hit =
                    intersect_cap_with_radius_one(ray, 0.0, Vec3::new(0.0, 0.0, -1.0), material);

                if let Some(hit_side) =
                    intersect_cone_infinite(ray, material).filter(test_if_hits_between_0_1(ray))
                {
                    best_hit = best_hit.filter(|hit| hit.t < hit_side.t).or(Some(hit_side))
                }

                best_hit
            }
        }
    }
}

fn intersect_cap_with_radius_one(
    ray: &Ray,
    cap_z_plane: f32,
    hit_normal: Vec3,
    material: &Material,
) -> Option<Hit> {
    let t = (cap_z_plane - ray.origin.z) / ray.direction.z;
    let p = ray.origin + ray.direction * t;
    if t > 0.0 && (p.y * p.y + p.x * p.x) < 1.0 {
        Some(Hit::new(t, hit_normal, *material))
    } else {
        None
    }
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<f32> {
    let discriminant = b * b - 4f32 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2f32 * a))
    }
}

fn intersect_cylinder_infinite(ray: &Ray, material: &Material) -> Option<Hit> {
    let a = ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y;
    let b = 2f32 * (ray.direction.x * ray.origin.x + ray.direction.y * ray.origin.y);
    let c = ray.origin.x * ray.origin.x + ray.origin.y * ray.origin.y - 1.0;
    solve_quadratic(a, b, c).map(|t| {
        let p = ray.origin + ray.direction * t;
        Hit::new(t, Vec3::new(p.x, p.y, 0.0), *material)
    })
}

#[allow(dead_code)]
fn intersect_cone_infinite_quadratic(ray: &Ray, material: Material) -> Option<Hit> {
    let a = ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y;
    let b =
        2f32 * (ray.direction.x * ray.origin.x + ray.direction.y * ray.origin.y) + ray.direction.z;
    let c = ray.origin.x * ray.origin.x + ray.origin.y * ray.origin.y - 1.0 + ray.origin.z;
    solve_quadratic(a, b, c).map(|t| {
        let p = ray.origin + ray.direction * t;
        Hit::new(
            t,
            Vec3::new(2.0 * p.x, 2.0 * p.y, p.z).normalize(),
            material,
        )
    })
}

fn intersect_cone_infinite(ray: &Ray, material: &Material) -> Option<Hit> {
    let a = ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y
        - ray.direction.z * ray.direction.z;
    let b = 2f32
        * (ray.direction.x * ray.origin.x + ray.direction.y * ray.origin.y
            - ray.direction.z * ray.origin.z
            + ray.direction.z);
    let c = ray.origin.x * ray.origin.x + ray.origin.y * ray.origin.y
        - (1.0 - ray.origin.z) * (1.0 - ray.origin.z);
    solve_quadratic(a, b, c).map(|t| {
        let p = ray.origin + ray.direction * t;
        Hit::new(t, Vec3::new(p.x, p.y, 1.0 - p.z).normalize(), *material)
    })
}

fn test_if_hits_between_0_1(ray: &Ray) -> impl Fn(&Hit) -> bool {
    |h| {
        let p = h.point(ray);
        p.z > 0.0 && p.z < 1.0
    }
}
