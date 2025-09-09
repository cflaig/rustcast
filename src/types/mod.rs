use glam::{Mat4, Vec3, Vec4};
use rand::prelude::SmallRng;
use rand::Rng;
use crate::shape::Shape;

pub trait Transformable {
    fn to_local_coordinates(&self, transform: &Transform) -> Self;
    fn to_global_coordinates(&self, transform: &Transform) -> Self;
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Vec3,
    pub ambient: f32,
    pub reflection: f32,
    pub shininess: f32,
    pub specular_coef: f32,
}
pub enum LightSource {
    Point {
        position: Vec3,
        color: Vec3,
    },
    Quad {
        color: Vec3,
    },
    TransformedShape {
        light_source: Box<LightSource>,
        transform: Transform,
    },
}

impl LightSource {
    pub fn sample(&self, rng: &mut SmallRng) -> (Vec3, Vec3, Vec3, f32) {
        match self {
            LightSource::Point { position, color } => {
                (*position, Vec3::ZERO, *color, 999999.0)
            }
            LightSource::Quad { color } => {
                let x = rng.random_range(-0.5..=0.5);
                let y = rng.random_range(-0.5..=0.5);
                let pdf = 1.0;
                (Vec3::new(x,y,0.0),Vec3::new(0.0,0.0,1.0), *color, pdf)
            }
            LightSource::TransformedShape { light_source, transform } => {
                let (p,n, c,pdf) = light_source.sample(rng);
                let world_p = transform.local_to_global(p.extend(1.0)).truncate();
                let world_n = transform.local_normal_to_global(n);
                let world_pdf = transform.inverse.determinant()/world_n.length();
                (world_p,world_n.normalize(),c,world_pdf)
            }
        }
    }
}

pub struct Hit {
    pub t: f32,
    pub normal: Vec3,
    pub material: Material,
}

impl Hit {
    pub fn new(t: f32, normal: Vec3, material: Material) -> Self {
        Hit {
            t,
            normal,
            material,
        }
    }
    pub fn point(&self, ray: &Ray) -> Vec3 {
        ray.origin + ray.direction * self.t
    }
}

pub fn find_first_hit(shape_iterator: impl IntoIterator<Item = Option<Hit>>) -> Option<Hit> {
    shape_iterator.into_iter().filter_map(|s| s).min_by(|x, y| {
        if (x.t < y.t) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
}

impl Transformable for Ray {
    fn to_local_coordinates(&self, transform: &Transform) -> Self {
        Ray {
            origin: transform
                .global_to_local(self.origin.extend(1.0))
                .truncate(),
            direction: transform
                .global_to_local(self.direction.extend(0.0))
                .truncate(),
        }
    }
    fn to_global_coordinates(&self, transform: &Transform) -> Self {
        todo!()
    }
}

impl Transformable for Hit {
    fn to_local_coordinates(&self, transform: &Transform) -> Self {
        todo!()
    }
    fn to_global_coordinates(&self, transform: &Transform) -> Self {
        Hit::new(
            self.t,
            transform.local_normal_to_global(self.normal).normalize(),
            self.material,
        )
    }
}

pub struct Transform {
    matrix: Mat4,
    inverse: Mat4,
}

impl Transform {
    pub fn new(matrix: Mat4) -> Self {
        Transform {
            matrix,
            inverse: matrix.inverse(),
        }
    }

    pub fn global_to_local(&self, v: Vec4) -> Vec4 {
        self.inverse * v
    }

    pub fn local_to_global(&self, v: Vec4) -> Vec4 {
        self.matrix * v
    }
    pub fn local_normal_to_global(&self, n: Vec3) -> Vec3 {
        (self.inverse.transpose() * n.extend(0.0))
            .truncate()
    }
}

mod test {
    use rand::SeedableRng;
    use super::*;

    #[test]
    fn test_light_pdf() {
        let light =         LightSource::TransformedShape {
            light_source: Box::new(LightSource::Quad {
                color: Vec3::new(1.0, 1.0, 1.0),
            }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.0, 0.0, 1.95))
                * Mat4::from_scale(Vec3::new(2.0, 2.0, 5.0)))
            ,        };

        let mut rng = SmallRng::from_seed([0;32]);
        let (_,_,_,pdf )= light.sample(&mut rng);

        println!("test: {}", pdf);

    }
}