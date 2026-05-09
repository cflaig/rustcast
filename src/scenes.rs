use glam::{Mat4, Vec3};
use std::f32::consts::{PI};

use crate::camera::Camera;
use crate::shape::Shape;
use crate::types::{Light, Material, Texture, Transform};

// Scene builders
pub fn make_default_scene() -> (Camera, Vec<Light>, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(0.0, -5.0, -1.25),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = vec![Light {
        position: Vec3::new(2.0, -2.0, 3.0),
        color: Vec3::new(1.0, 1.0, 1.0),
    }];

    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };

    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };

    let shapes: Vec<Shape> = vec![
        Shape::Cone { texture: Texture::Constant(red) },
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            texture: Texture::Constant(blue),
        },
    ];
    (camera, light, shapes)
}

#[allow(dead_code)]
pub fn make_scene_with_eight_boxes() -> (Camera, Vec<Light>, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(0.0, -15.0, 5.),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = vec![Light {
        position: Vec3::new(3.0, -2.0, 4.0),
        color: Vec3::new(1.0, 1.0, 1.0),
    }];

    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };
    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };

    let mut shapes: Vec<Shape> = vec![Shape::Plane {
        normal: Vec3::new(0.0, 0.0, 1.0),
        d: -2.0,
        texture: Texture::Constant(blue),
    }];

    for i in 0..8 {
        let mut matrix = Mat4::from_rotation_z(std::f32::consts::PI / 16.0 * i as f32);
        matrix = Mat4::from_translation(Vec3::new(-7.5 + 2.5 * i as f32, 0.0, 0.0)) * matrix;
        let transform = Transform::new(matrix);

        shapes.push(Shape::TransformedShape {
            shape: Box::new(Shape::UnitBox { texture: Texture::Constant(red) }),
            transform,
        });
    }
    (camera, light, shapes)
}

#[allow(dead_code)]
pub fn make_scene_cylinder_plane() -> (Camera, Vec<Light>, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(0.0, -5.0, -0.75),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = vec![Light {
        position: Vec3::new(2.0, -2.0, 3.0),
        color: Vec3::new(1.0, 1.0, 1.0),
    }];

    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };

    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };

    let shapes: Vec<Shape> = vec![
        Shape::Cone { texture: Texture::Constant(red) },
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            texture: Texture::Constant(blue),
        },
    ];
    (camera, light, shapes)
}

pub fn make_cornell_scene() -> (Camera, Vec<Light>, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(0.0, -7.0, 0.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
    );
    
    const NUM_LIGHTS: usize = 3;
    const LIGHT_INTENSITY: f32 = 1.0 / NUM_LIGHTS as f32 / NUM_LIGHTS as f32;
    let mut light: Vec<Light> = Vec::new();
    for x in 0..NUM_LIGHTS {
        for y in 0..NUM_LIGHTS {
            light.push( Light {
                position: Vec3::new(
                    0.96 * (x as f32) / (NUM_LIGHTS as f32 - 1.0) - 0.48 ,
                    0.96 * (y as f32) / (NUM_LIGHTS as f32 - 1.0) + 0.02,
                    1.98,
                ),
                color: Vec3::splat(LIGHT_INTENSITY*1.2),
            })
        }
    }

    let white = Material {
        color: Vec3::splat(0.9),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.0,
    };
    let light_gray = Material {
        color: Vec3::splat(0.6),
        ..white
    };
    let white_light = Material {
        color: Vec3::splat(0.9),
        ambient: 1.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 1.0,
    };
    let red = Material {
        color: Vec3::new(0.9, 0.1, 0.1),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.0,
    };
    let gray_red = Material {
        color: Vec3::new(0.7, 0.2, 0.2),
        ..red
    };
    let green = Material {
        color: Vec3::new(0.1, 0.9, 0.1),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.0,
    };
    let gray_green = Material {
        color: Vec3::new(0.2, 0.7, 0.2),
        ..green
    };

    let _blue = Material {
        color: Vec3::new(0.1, 0.1, 0.9),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.0,
    };

    let shapes: Vec<Shape> = vec![
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -2.0,
            texture: Texture::Brick(white, light_gray),
        },
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, -1.0),
            d: -2.0,
            texture: Texture::Constant(white),
        },
        Shape::Plane {
            normal: Vec3::new(0.0, -1.0, 0.0),
            d: -2.0,
            texture: Texture::Constant(white),
        },
        Shape::Plane {
            normal: Vec3::new(1.0, 0.0, 0.0),
            d: -2.0,
            texture: Texture::Brick(red, gray_red),
        },
        Shape::Plane {
            normal: Vec3::new(-1.0, 0.0, 0.0),
            d: -2.0,
            texture: Texture::Brick(green, gray_green),
        },
        Shape::Sphere {
            center: Vec3::new(-1.2, -0.2, 0.5),
            radius: 0.66666,
            texture: Texture::Constant(white),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::Cylinder { texture: Texture::Constant(white) }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(-1.111, -1.333, -2.0))
                    * Mat4::from_scale(Vec3::new(0.25, 0.25, 1.5)),
            ),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::Cone { texture: Texture::Constant(white) }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(1.5, 0.5, -2.0))
                    * Mat4::from_scale(Vec3::new(0.25, 0.25, 1.0)),
            ),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::UnitBox { texture: Texture::Constant(white) }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.0, 0.25, -1.24))
                    * Mat4::from_rotation_z(PI / 6.0)
                    * Mat4::from_scale(Vec3::new(0.75, 0.75, 0.75)),
            ),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::Square {
                texture: Texture::Constant(white_light),
            }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.0, 0.5, 1.99998))
                    * Mat4::from_rotation_x(PI),
            ),
        },
    ];
    (camera, light, shapes)
}

pub fn make_box_scene() -> (Camera, Vec<Light>, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(0.0, -6.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        2.0,
    );

    const NUM_LIGHTS: usize = 3;
    const LIGHT_INTENSITY: f32 = 1.0 / NUM_LIGHTS as f32 / NUM_LIGHTS as f32;
    let mut light: Vec<Light> = Vec::new();
    for x in 0..NUM_LIGHTS {
        for y in 0..NUM_LIGHTS {
            light.push( Light {
                position: Vec3::new(
                    1.92 * (x as f32) / (NUM_LIGHTS as f32 - 1.0) - 0.96 ,
                    1.92 * (y as f32) / (NUM_LIGHTS as f32 - 1.0)  + 0.04,
                    3.96,
                ),
                color: Vec3::splat(LIGHT_INTENSITY*1.2),
            })
        }
    }

    let white = Material {
        color: Vec3::splat(0.9),
        ambient: 0.1,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.0,
    };
    let white_light = Material {
        color: Vec3::splat(0.9),
        ambient: 1.0,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 1.0,
    };
    let gray = Material {
        color: Vec3::splat(0.7),
        ..white
    };
    let red = Material {
        color: Vec3::new(0.9, 0.1, 0.1),
        ..white
    };
    let red_gray = Material {
        color: Vec3::new(0.8, 0.6, 0.6),
        ..white
    };
    let green = Material {
        color: Vec3::new(0.1, 0.9, 0.1),
        ..white
    };
    let green_gray = Material {
        color: Vec3::new(0.6, 0.8, 0.6),
        ..white
    };
    let blue = Material {
        color: Vec3::new(0.1, 0.1, 0.9),
        ..white
    };
    let blue_gray = Material {
        color: Vec3::new(0.6, 0.6, 0.8),
        ..white
    };
    let yellow = Material {
        color: Vec3::new(0.9, 0.9, 0.1),
        ..white
    };
    let yellow_gray = Material {
        color: Vec3::new(0.8, 0.8, 0.6),
        ..white
    };

    let transparent_mat = Material {
        color: Vec3::splat(1.0),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 100.0,
        specular_coef: 0.9,
        transparency: 0.9,
        ior: 1.5,
        emission: 0.0,
    };

    let transparent_mat2 = Material {
        color: Vec3::splat(1.0),
        ambient: 0.0,
        reflection: 0.0,
        shininess: 100.0,
        specular_coef: 0.9,
        transparency: 0.9,
        ior: 1.3,
        emission: 0.0,
    };

    let reflective_mat = Material {
        color: Vec3::splat(1.0),
        ambient: 0.0,
        reflection: 0.9,
        shininess: 100.0,
        specular_coef: 0.9,
        transparency: 0.0,
        ior: 1.5,
        emission: 0.0,
    };

    let shapes: Vec<Shape> = vec![
        // Boden: weiss grau
        Shape::Plane {
            normal: Vec3::new(0.0, 0.0, 1.0),
            d: -4.0,
            texture: Texture::Brick(white, gray),
        },
        // Hintere Wand: blau, graublau
        Shape::Plane {
            normal: Vec3::new(0.0, -1.0, 0.0),
            d: -4.0,
            texture: Texture::Brick(blue, blue_gray),
        },
        // Linke Wand: Rot, rotgrau gebrickt
        Shape::Plane {
            normal: Vec3::new(1.0, 0.0, 0.0),
            d: -4.0,
            texture: Texture::Brick(red, red_gray),
        },
        // Rechte Wand: Grün, graugrün gebrickt
        Shape::Plane {
            normal: Vec3::new(-1.0, 0.0, 0.0),
            d: -4.0,
            texture: Texture::Brick(green, green_gray),
        },
        // Vorderwand (hinter der Kamera): Gelb, gelbgrau gebrickt
        Shape::Plane {
            normal: Vec3::new(0.0, 1.0, 0.0),
            d: -6.5,
            texture: Texture::Brick(yellow, yellow_gray),
        },
        // Decke
        Shape::TransformedShape {
            shape: Box::new(Shape::Plane {
                normal: Vec3::new(0.0, 0.0, -1.0),
                d: -4.0,
                texture: Texture::Constant(white),
            }),
            transform: Transform::new(Mat4::IDENTITY),
        },
        // Lichtquelle an der Decke
        Shape::TransformedShape {
            shape: Box::new(Shape::Square {
                texture: Texture::Constant(white_light),
            }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.0, 1.0, 3.99996))
                    * Mat4::from_rotation_x(PI)
                    * Mat4::from_scale(Vec3::splat(2.0)),
            ),
        },
        // Transparente Kugel links
        Shape::Sphere {
            center: Vec3::new(-1.9, 0.8, -1.0),
            radius: 1.6,
            texture: Texture::Constant(transparent_mat),
        },
        // Reflektierende Kugel rechts
        Shape::Sphere {
            center: Vec3::new(1.9, 0.4, -1.0),
            radius: 1.6,
            texture: Texture::Constant(reflective_mat),
        },
        Shape::TransformedShape {
            shape: Box::new(Shape::Cone {
                texture: Texture::Constant(transparent_mat2),
            }),
            transform: Transform::new(
                Mat4::from_translation(Vec3::new(0.50, -1.10, -2.0))
                    * Mat4::from_rotation_z(PI/5.0)
                    * Mat4::from_rotation_y(-1.0*PI/2.0)
                    * Mat4::from_scale(Vec3::new(0.75,0.75,3.0*0.75)),
            ),
        },
    ];
    (camera, light, shapes)
}

pub fn make_axes_scene() -> (Camera, Vec<Light>, Vec<Shape>) {
    let camera = Camera::new(
        Vec3::new(1.5, 4.0, 1.35),
        Vec3::new(0.5, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.1,
    );
    let light = vec![Light {
        position: Vec3::new(2.0, 4.0, 3.0),
        color: Vec3::new(1.0, 1.0, 1.0),
    }];

    let red = Material {
        color: Vec3::new(1.0, 0.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };
    let green = Material {
        color: Vec3::new(0.0, 1.0, 0.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };
    let blue = Material {
        color: Vec3::new(0.0, 0.0, 1.0),
        ambient: 0.3,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.3,
    };
    let white = Material {
        color: Vec3::splat(0.9),
        ambient: 0.2,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
        transparency: 0.0,
        ior: 1.0,
        emission: 0.0,
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
        shape: Box::new(Shape::Cylinder { texture: Texture::Constant(red) }),
        transform: Transform::new(r_x * s_shaft),
    });
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cone { texture: Texture::Constant(red) }),
        transform: Transform::new(r_x * t_tip * s_tip),
    });

    // Y axis (green)
    let r_y = Mat4::from_rotation_x(-std::f32::consts::FRAC_PI_2);
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cylinder { texture: Texture::Constant(green) }),
        transform: Transform::new(r_y * s_shaft),
    });
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cone { texture: Texture::Constant(green) }),
        transform: Transform::new(r_y * t_tip * s_tip),
    });

    // Z axis (blue)
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cylinder { texture: Texture::Constant(blue) }),
        transform: Transform::new(s_shaft),
    });
    shapes.push(Shape::TransformedShape {
        shape: Box::new(Shape::Cone { texture: Texture::Constant(blue) }),
        transform: Transform::new(t_tip * s_tip),
    });

    shapes.push(Shape::Plane {
        normal: Vec3::new(0.0, 0.0, 1.0),
        d: -2.0,
        texture: Texture::Constant(white),
    });

    (camera, light, shapes)
}
