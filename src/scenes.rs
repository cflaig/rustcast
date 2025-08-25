use glam::{Mat4, Vec3};
use std::f32::consts::PI;

use crate::camera::Camera;
use crate::shape::Shape;
use crate::types::{Material, Transform};

// Scene builders
pub fn make_default_scene() -> (Camera, Vec3, Vec<Shape>) {
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

    let shapes: Vec<Shape> = vec![
        Shape::Cone { material: red },
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            material: blue,
        },
    ];
    (camera, light, shapes)
}

#[allow(dead_code)]
pub fn make_scene_with_eight_boxes() -> (Camera, Vec3, Vec<Shape>) {
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
    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let mut shapes: Vec<Shape> = vec![Shape::Plane {
        normal: Vec3::new(0.0, 0.0, 1.0),
        d: -2.0,
        material: blue,
    }];

    for i in 0..8 {
        let mut matrix = Mat4::from_rotation_z(std::f32::consts::PI / 16.0 * i as f32);
        matrix = Mat4::from_translation(Vec3::new(-7.5 + 2.5 * i as f32, 0.0, 0.0)) * matrix;
        let transform = Transform::new(matrix);

        shapes.push(Shape::TransformedShape {
            shape: Box::new(Shape::UnitBox { material: red }),
            transform,
        });
    }
    (camera, light, shapes)
}

#[allow(dead_code)]
pub fn make_scene_cylinder_plane() -> (Camera, Vec3, Vec<Shape>) {
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

    let shapes: Vec<Shape> = vec![
        Shape::Cone { material: red },
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            material: blue,
        },
    ];
    (camera, light, shapes)
}

pub fn make_cornell_scene() -> (Camera, Vec3, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(0.0, -7.0, 0.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
    );
    let light = Vec3::new(0.0, -0.75, 1.9);

    let white = Material {
        color: Vec3::splat(0.9),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let white_light = Material {
        color: Vec3::splat(0.9),
        ambient: 1.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let red = Material {
        color: Vec3::new(0.9, 0.1, 0.1),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let green = Material {
        color: Vec3::new(0.1, 0.9, 0.1),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let blue = Material {
        color: Vec3::new(0.1, 0.1, 0.9),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };

    let shapes: Vec<Shape> = vec![
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            material: white,
        },
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, -1.0),
            d: -2.0,
            material: blue,
        },
        Shape::Plane {
            normal: Vec3::new(0.0, -1.0, 0.0),
            d: -2.0,
            material: blue,
        },
        Shape::Plane {
            normal: Vec3::new(1.0, 0.0, 0.0),
            d: -2.0,
            material: red,
        },
        Shape::Plane {
            normal: Vec3::new(-1.0, 0.0, 0.0),
            d: -2.0,
            material: green,
        },
        Shape::Sphere {
            center: Vec3::new(-1.2, -0.2, 0.5),
            radius: 0.66666,
            material: white,
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::Cylinder { material: white }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(-1.111, -1.333, -2.0))
                    * Mat4::from_scale(Vec3::new(0.25, 0.25, 1.5)),
            ),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::Cone { material: white }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(1.5, 0.5, -2.0))
                    * Mat4::from_scale(Vec3::new(0.25, 0.25, 1.0)),
            ),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::UnitBox { material: white }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.0, 0.25, -1.24))
                    * Mat4::from_rotation_z(PI / 6.0)
                    * Mat4::from_scale(Vec3::new(0.75, 0.75, 0.75)),
            ),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::UnitBox {
                material: white_light,
            }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.0, -0.5, 1.99998))
                    * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.00001)),
            ),
        },
    ];
    (camera, light, shapes)
}

pub fn make_axes_scene() -> (Camera, Vec3, Vec<Shape>) {
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

    let mut shapes: Vec<Shape> = Vec::new();

    let s_shaft = Mat4::from_scale(Vec3::new(shaft_r, shaft_r, shaft_l));
    let s_tip = Mat4::from_scale(Vec3::new(tip_r, tip_r, tip_l));
    let t_tip = Mat4::from_translation(Vec3::new(0.0, 0.0, shaft_l));

    // X axis (red)
    let r_x = Mat4::from_rotation_y(std::f32::consts::FRAC_PI_2);

    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cylinder { material: red }),
        transform: Transform::new(r_x * s_shaft),
    });
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cone { material: red }),
        transform: Transform::new(r_x * t_tip * s_tip),
    });

    // Y axis (green)
    let r_y = Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_2);
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cylinder { material: green }),
        transform: Transform::new(r_y * s_shaft),
    });
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cone { material: green }),
        transform: Transform::new(r_y * t_tip * s_tip),
    });

    // Z axis (blue)
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cylinder { material: blue }),
        transform: Transform::new(s_shaft),
    });
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cone { material: blue }),
        transform: Transform::new(t_tip * s_tip),
    });

    shapes.push(Shape::Plane {
        normal: Vec3::new(0.0, 0.0, 1.0),
        d: -2.0,
        material: white,
    });

    (camera, light, shapes)
}
