use rustcast::renderer::sample_random_on_sphere;
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
