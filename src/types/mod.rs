use glam::{Mat4, Vec3, Vec4};

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

#[derive(Copy, Clone, Debug)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
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
            transform.local_normal_to_global(self.normal),
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
            .normalize()
    }
}
