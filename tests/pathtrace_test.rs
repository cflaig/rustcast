use rustcast::renderer::{sample_random_on_sphere, sample_cosine_weighted_hemisphere};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use glam::Vec3;

#[test]
fn test_sample_random_on_sphere_uniformity() {
    let mut rng = SmallRng::seed_from_u64(42);
    let mut sum = Vec3::ZERO;
    let n = 100000;
    for _ in 0..n {
        sum += sample_random_on_sphere(&mut rng);
    }
    let avg = sum / n as f32;
    assert!(avg.length() < 0.05);
}

#[test]
fn test_sample_cosine_weighted_hemisphere() {
    let mut rng = SmallRng::seed_from_u64(42);
    let normal = Vec3::Z;
    let mut sum = Vec3::ZERO;
    let n = 100000;
    for _ in 0..n {
        sum += sample_cosine_weighted_hemisphere(&mut rng, normal);
    }
    let avg = sum / n as f32;
    // For cosine weighted sampling with normal (0,0,1), the average direction should be (0,0,2/3)
    assert!((avg.z - 2.0/3.0).abs() < 0.01);
    assert!(avg.x.abs() < 0.01);
    assert!(avg.y.abs() < 0.01);
}
