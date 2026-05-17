use std::ops::Add;
use glam::{Mat4, Vec2, Vec3, Vec4};

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
pub enum Texture {
    Constant(Material),
    Brick(Material, Material),
    Checkboard(Material, Material),
}

impl Texture {
    pub fn material_at(&self, hitpoint: &Hit, ray: &Ray) -> Material {
        match self {
            Texture::Constant(mat) => *mat,
            Texture::Brick(mat1, mat2) => {
                if hitpoint.point(ray).add(Vec3::new(0.34243, 0.56789, 0.42345)).floor().dot(Vec3::new(1.0, 1.0, 1.0)) as i32 % 2 != 0
                { *mat1 }
                else { *mat2 }
            },
            Texture::Checkboard(mat1, _mat2) => {
                    *mat1
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub color: Vec3,
    pub reflection: f32,
    pub roughness: f32,
    pub transparency: f32,
    pub ior: f32, // Index of Refraction
    // path tracer
    pub emission: f32,
    // ray tracer
    pub ambient: f32,
}


#[derive(Copy, Clone, Debug)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
}

pub struct Hit {
    pub t: f32,
    pub normal: Vec3,
    pub uv: Vec2,
    //pub local_coords: Vec3,
    pub texture: Texture,
}

impl Hit {
    pub fn new(t: f32, normal: Vec3, uv: Vec2, texture: Texture) -> Self {
        Hit {
            t,
            normal,
            uv,
            texture,
        }
    }
    pub fn point(&self, ray: &Ray) -> Vec3 {
        ray.origin + ray.direction * self.t
    }
}

pub fn find_first_hit(shape_iterator: impl IntoIterator<Item = Option<Hit>>) -> Option<Hit> {
    shape_iterator.into_iter().filter_map(|s| s).min_by(|x, y| {
        if x.t < y.t {
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
    fn to_global_coordinates(&self, _transform: &Transform) -> Self {
        todo!()
    }
}

impl Transformable for Hit {
    fn to_local_coordinates(&self, _transform: &Transform) -> Self {
        todo!()
    }
    fn to_global_coordinates(&self, transform: &Transform) -> Self {
        Hit::new(
            self.t,
            transform.local_normal_to_global(self.normal),
            self.uv,
            self.texture,
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
            .normalize()
    }
}
