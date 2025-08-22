use glam::{Mat4, Vec3};

use crate::shape::{Cone, Cylinder, Plane, Shape, Sphere, TransformedShape, UnitBox};
use crate::types::{Camera, Material, Transform};

// Scene builders
pub fn make_default_scene() -> (Camera, Vec3, Vec<Box<dyn Shape>>) {
    let camera = Camera::new(
        Vec3::new(0.0, -5.0, -1.25),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = Vec3::new(2.0, -2.0, 3.0);

    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Cone { material: red }),
        Box::new(Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            material: blue,
        }),
    ];
    (camera, light, shapes)
}

#[allow(dead_code)]
pub fn make_scene_with_eight_boxes() -> (Camera, Vec3, Vec<Box<dyn Shape>>) {
    let camera = Camera::new(
        Vec3::new(0.0, -15.0, 5.),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = Vec3::new(3.0, -2.0, 4.0);

    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    //let red = Material { color: Vec3::new(1.0, 0.0, 0.0), ambient: 0.3, reflection: 0.0, shininess: 0.0, specular_coef: 0.0 };

    let mut shapes: Vec<Box<dyn Shape>> = vec![Box::new(Plane {
        normal: Vec3::new(0.0, 0.0, 1.0),
        d: -2.0,
        material: blue,
    })];

    for i in 0..8 {
        let mut matrix = Mat4::from_rotation_z(std::f32::consts::PI / 16.0 * i as f32);
        matrix = Mat4::from_translation(Vec3::new(-7.5 + 2.5 * i as f32, 0.0, 0.0)) * matrix;
        let transform = Transform::new(matrix);

        shapes.push(Box::new(TransformedShape::new(
            Box::new(UnitBox {}),
            transform,
        )));
    }
    (camera, light, shapes)
}

#[allow(dead_code)]
pub fn make_scene_cylinder_plane() -> (Camera, Vec3, Vec<Box<dyn Shape>>) {
    let camera = Camera::new(
        Vec3::new(0.0, -5.0, -0.75),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = Vec3::new(2.0, -2.0, 3.0);

    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Cone { material: red }),
        Box::new(Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            material: blue,
        }),
    ];
    (camera, light, shapes)
}

pub fn make_cornell_scene() -> (Camera, Vec3, Vec<Box<dyn Shape>>) {
    let camera = Camera::new(
        Vec3::new(0.0, -5.0, 0.5),
        Vec3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
    );
    let light = Vec3::new(0.0, -1.0, 1.8);

    let white = Material {
        color: Vec3::splat(0.9),
        ambient: 0.2,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let red = Material {
        color: Vec3::new(0.9, 0.1, 0.1),
        ambient: 0.2,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let green = Material {
        color: Vec3::new(0.1, 0.9, 0.1),
        ambient: 0.2,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: 0.0,
            material: white,
        }),
        Box::new(Plane {
            normal: Vec3::new(0.0, 0.0, -1.0),
            d: -2.0,
            material: white,
        }),
        Box::new(Plane {
            normal: Vec3::new(0.0, -1.0, 0.0),
            d: -3.0,
            material: white,
        }),
        Box::new(Plane {
            normal: Vec3::new(1.0, 0.0, 0.0),
            d: -2.0,
            material: red,
        }),
        Box::new(Plane {
            normal: Vec3::new(-1.0, 0.0, 0.0),
            d: -2.0,
            material: green,
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.2, 0.2, 0.5),
            radius: 0.5,
            material: white,
        }),
        Box::new(Cylinder { material: white }),
    ];
    (camera, light, shapes)
}

pub fn make_axes_scene() -> (Camera, Vec3, Vec<Box<dyn Shape>>) {
    let camera = Camera::new(
        Vec3::new(1.5, 4.0, 1.35),
        Vec3::new(0.5, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = Vec3::new(2.0, 4.0, 3.0);

    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let green = Material {
        color: Vec3::new(0.0, 1.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let white = Material {
        color: Vec3::splat(0.9),
        ambient: 0.2,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let shaft_r = 0.05f32;
    let shaft_l = 1.5f32;
    let tip_r = 0.12f32;
    let tip_l = 0.35f32;

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

    let s_shaft = Mat4::from_scale(Vec3::new(shaft_r, shaft_r, shaft_l));
    let s_tip = Mat4::from_scale(Vec3::new(tip_r, tip_r, tip_l));
    let t_tip = Mat4::from_translation(Vec3::new(0.0, 0.0, shaft_l));

    // X axis (red)
    let r_x = Mat4::from_rotation_y(std::f32::consts::FRAC_PI_2);

    shapes.push(Box::new(TransformedShape::new(
        Box::new(Cylinder { material: red }),
        Transform::new(r_x * s_shaft),
    )));
    shapes.push(Box::new(TransformedShape::new(
        Box::new(Cone { material: red }),
        Transform::new(r_x * t_tip * s_tip),
    )));

    // Y axis (green)
    let r_y = Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_2);
    shapes.push(Box::new(TransformedShape::new(
        Box::new(Cylinder { material: green }),
        Transform::new(r_y * s_shaft),
    )));
    shapes.push(Box::new(TransformedShape::new(
        Box::new(Cone { material: green }),
        Transform::new(r_y * t_tip * s_tip),
    )));

    // Z axis (blue)
    shapes.push(Box::new(TransformedShape::new(
        Box::new(Cylinder { material: blue }),
        Transform::new(s_shaft),
    )));
    shapes.push(Box::new(TransformedShape::new(
        Box::new(Cone { material: blue }),
        Transform::new(t_tip * s_tip),
    )));

    shapes.push(Box::new(Plane {
        normal: Vec3::new(0.0, 0.0, 1.0),
        d: -2.0,
        material: white,
    }));

    (camera, light, shapes)
}
