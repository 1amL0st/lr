use rand::Rng;

use crate::Vec3;

pub fn point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut point = Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    while point.len() >= 1.0 {
        point = Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    }
    point
}