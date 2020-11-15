use rand::Rng;

use nlm;

pub trait NlmVec3Ext {
    fn zeros() -> nlm::Vec3;
    fn new_color(r: u8, g: u8, b: u8) -> nlm::Vec3;
}

impl NlmVec3Ext for nlm::Vec3 {
    fn zeros() -> nlm::Vec3 {
        nlm::Vec3::new(0., 0., 0.)
    }

    fn new_color(r: u8, g: u8, b: u8) -> nlm::Vec3 {
        let factor = 1. / 255.;
        nlm::Vec3::new(
            (r as f32) * factor,
            (g as f32) * factor,
            (b as f32) * factor
        )
    }
}

pub fn point_in_unit_sphere() -> nlm::Vec3 {
    let mut rng = rand::thread_rng();
    let mut point = nlm::Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    while point.magnitude() >= 1.0 {
        point = nlm::Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    }
    point
}