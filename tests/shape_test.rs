use rustcast::types::{Ray, Material};
use rustcast::shape::Shape;
use glam::Vec3;

#[test]
fn test_square_intersection() {
    let material = Material {
        color: Vec3::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        reflection: 0.0,
        shininess: 0.0,
        specular_coef: 0.0,
    };
    let square = Shape::Square { material };

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
    // Depending on implementation, normal might be fixed (0,0,1) or depend on incident ray.
    // Usually squares are double sided or have a fixed normal. 
    // Given x,y in [-0.5, 0.5] in XY plane, normal is typically (0,0,1).
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
