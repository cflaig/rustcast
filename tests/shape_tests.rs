use rustcast::shape::{Shape, solve_quadratic};
use rustcast::types::{Ray, Material, Texture};
use glam::Vec3;

#[test]
fn test_square_intersection() {
    let material = Material {
        color: Vec3::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        reflection: 0.0,
        roughness: 0.0,
        transparency: 0.0,
        ior: 0.0,
        emission: 0.0,
    };
    let square = Shape::Square { texture: Texture::Constant(material) };
    
    // Ray hitting the square from the front (positive Z)
    let ray1 = Ray {
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };
    let hit1 = square.intersect(&ray1);
    assert!(hit1.is_some());
    let hit1 = hit1.unwrap();
    assert_eq!(hit1.t, 1.0);
    assert_eq!(hit1.normal, Vec3::new(0.0, 0.0, 1.0));

    // Ray hitting the square from the back (negative Z)
    let ray2 = Ray {
        origin: Vec3::new(0.0, 0.0, -1.0),
        direction: Vec3::new(0.0, 0.0, 1.0),
    };
    let hit2 = square.intersect(&ray2);
    assert!(hit2.is_some());
    let hit2 = hit2.unwrap();
    assert_eq!(hit2.t, 1.0);
    assert_eq!(hit2.normal, Vec3::new(0.0, 0.0, 1.0));

    // Ray hitting the edge
    let ray3 = Ray {
        origin: Vec3::new(0.4999, 0.4999, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };
    let hit3 = square.intersect(&ray3);
    assert!(hit3.is_some());

    // Ray missing the square (outside X)
    let ray4 = Ray {
        origin: Vec3::new(0.6, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };
    let hit4 = square.intersect(&ray4);
    assert!(hit4.is_none());

    // Ray missing the square (outside Y)
    let ray5 = Ray {
        origin: Vec3::new(0.0, 0.6, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };
    let hit5 = square.intersect(&ray5);
    assert!(hit5.is_none());

    // Ray parallel to the square
    let ray6 = Ray {
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
    };
    let hit6 = square.intersect(&ray6);
    assert!(hit6.is_none());
}

#[test]
fn test_solve_quadratic() {
    assert_eq!(solve_quadratic(1.0, 1.0, 1.0), None);
    assert_eq!(solve_quadratic(1.0, -3.0, 2.0), Some(1.0));
    //x1 = 1 , x2 = 2
    assert_eq!(solve_quadratic(1.0, -3.0, 2.0), Some(1.0));
    //x1 = 1 , x2 = -2
    assert_eq!(solve_quadratic(1.0, 1.0, -2.0), Some(1.0));
    //x1 = -1 , x2 = 2
    assert_eq!(solve_quadratic(1.0, -1.0, -2.0), Some(2.0));
    //x1 = -1 , x2 = -2
    assert_eq!(solve_quadratic(1.0, 3.0, 2.0), None);
}

#[test]
fn test_box_intersection() {
    let material = Material {
        color: Vec3::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        reflection: 0.0,
        roughness: 0.0,
        transparency: 0.0,
        ior: 0.0,
        emission: 0.0,
    };
    let unit_box = Shape::UnitBox { texture: Texture::Constant(material) };

    // Outside hitting front face (Z+)
    let ray_outside_front = Ray {
        origin: Vec3::new(0.0, 0.0, 2.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };
    let hit = unit_box.intersect(&ray_outside_front).expect("Should hit box");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));

    // Outside hitting back face (Z-)
    let ray_outside_back = Ray {
        origin: Vec3::new(0.0, 0.0, -2.0),
        direction: Vec3::new(0.0, 0.0, 1.0),
    };
    let hit = unit_box.intersect(&ray_outside_back).expect("Should hit box");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));

    // Inside hitting front face (Z+)
    let ray_inside_front = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(0.0, 0.0, 1.0),
    };
    let hit = unit_box.intersect(&ray_inside_front).expect("Should hit box from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));

    // Inside hitting back face (Z-)
    let ray_inside_back = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };
    let hit = unit_box.intersect(&ray_inside_back).expect("Should hit box from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));

    // Inside hitting right face (X+)
    let ray_inside_right = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 0.0, 0.0),
    };
    let hit = unit_box.intersect(&ray_inside_right).expect("Should hit box from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));

    // Inside hitting left face (X-)
    let ray_inside_left = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(-1.0, 0.0, 0.0),
    };
    let hit = unit_box.intersect(&ray_inside_left).expect("Should hit box from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(-1.0, 0.0, 0.0));

    // Inside hitting top face (Y+)
    let ray_inside_top = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(0.0, 1.0, 0.0),
    };
    let hit = unit_box.intersect(&ray_inside_top).expect("Should hit box from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));

    // Inside hitting bottom face (Y-)
    let ray_inside_bottom = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(0.0, -1.0, 0.0),
    };
    let hit = unit_box.intersect(&ray_inside_bottom).expect("Should hit box from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, -1.0, 0.0));

    // Test all other faces from outside
    // X+
    let hit = unit_box.intersect(&Ray { origin: Vec3::new(2.0, 0.0, 0.0), direction: Vec3::new(-1.0, 0.0, 0.0) }).unwrap();
    assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));
    // X-
    let hit = unit_box.intersect(&Ray { origin: Vec3::new(-2.0, 0.0, 0.0), direction: Vec3::new(1.0, 0.0, 0.0) }).unwrap();
    assert_eq!(hit.normal, Vec3::new(-1.0, 0.0, 0.0));
    // Y+
    let hit = unit_box.intersect(&Ray { origin: Vec3::new(0.0, 2.0, 0.0), direction: Vec3::new(0.0, -1.0, 0.0) }).unwrap();
    assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
    // Y-
    let hit = unit_box.intersect(&Ray { origin: Vec3::new(0.0, -2.0, 0.0), direction: Vec3::new(0.0, 1.0, 0.0) }).unwrap();
    assert_eq!(hit.normal, Vec3::new(0.0, -1.0, 0.0));
}

#[test]
fn test_cylinder_intersection() {
    let material = Material {
        color: Vec3::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        reflection: 0.0,
        roughness: 0.0,
        transparency: 0.0,
        ior: 0.0,
        emission: 0.0,
    };
    let cylinder = Shape::Cylinder { texture: Texture::Constant(material) };

    // Outside hitting side
    let ray_side = Ray { origin: Vec3::new(2.0, 0.0, 0.5), direction: Vec3::new(-1.0, 0.0, 0.0) };
    let hit = cylinder.intersect(&ray_side).expect("Should hit side");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));

    // Outside hitting top cap (z=1)
    let ray_top = Ray { origin: Vec3::new(0.0, 0.0, 2.0), direction: Vec3::new(0.0, 0.0, -1.0) };
    let hit = cylinder.intersect(&ray_top).expect("Should hit top cap");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));

    // Outside hitting bottom cap (z=0)
    let ray_bottom = Ray { origin: Vec3::new(0.0, 0.0, -1.0), direction: Vec3::new(0.0, 0.0, 1.0) };
    let hit = cylinder.intersect(&ray_bottom).expect("Should hit bottom cap");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));

    // Inside hitting side
    let ray_inside_side = Ray { origin: Vec3::new(0.0, 0.0, 0.5), direction: Vec3::new(1.0, 0.0, 0.0) };
    let hit = cylinder.intersect(&ray_inside_side).expect("Should hit side from inside");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));

    // Inside hitting top cap
    let ray_inside_top = Ray { origin: Vec3::new(0.0, 0.0, 0.5), direction: Vec3::new(0.0, 0.0, 1.0) };
    let hit = cylinder.intersect(&ray_inside_top).expect("Should hit top cap from inside");
    assert_eq!(hit.t, 0.5);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));

    // Inside hitting bottom cap
    let ray_inside_bottom = Ray { origin: Vec3::new(0.0, 0.0, 0.5), direction: Vec3::new(0.0, 0.0, -1.0) };
    let hit = cylinder.intersect(&ray_inside_bottom).expect("Should hit bottom cap from inside");
    assert_eq!(hit.t, 0.5);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
}

#[test]
fn test_cone_intersection() {
    let material = Material {
        color: Vec3::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        reflection: 0.0,
        roughness: 0.0,
        transparency: 0.0,
        ior: 0.0,
        emission: 0.0,
    };
    let cone = Shape::Cone { texture: Texture::Constant(material) };

    // Outside hitting side
    let ray_side = Ray { origin: Vec3::new(1.0, 0.0, 0.5), direction: Vec3::new(-1.0, 0.0, 0.0) };
    let hit = cone.intersect(&ray_side).expect("Should hit side");
    assert_eq!(hit.t, 0.5);
    let expected_normal = Vec3::new(0.5, 0.0, 0.5).normalize();
    assert!((hit.normal - expected_normal).length() < 1e-6);

    // Outside hitting bottom cap (z=0)
    let ray_bottom = Ray { origin: Vec3::new(0.0, 0.0, -1.0), direction: Vec3::new(0.0, 0.0, 1.0) };
    let hit = cone.intersect(&ray_bottom).expect("Should hit bottom cap");
    assert_eq!(hit.t, 1.0);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));

    // Inside hitting side
    let ray_inside_side = Ray { origin: Vec3::new(0.0, 0.0, 0.5), direction: Vec3::new(1.0, 0.0, 0.0) };
    let hit = cone.intersect(&ray_inside_side).expect("Should hit side from inside");
    assert_eq!(hit.t, 0.5);

    // Inside hitting bottom cap
    let ray_inside_bottom = Ray { origin: Vec3::new(0.0, 0.0, 0.5), direction: Vec3::new(0.0, 0.0, -1.0) };
    let hit = cone.intersect(&ray_inside_bottom).expect("Should hit bottom cap from inside");
    assert_eq!(hit.t, 0.5);
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
}
