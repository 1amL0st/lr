use crate::objects::sphere::Sphere;
use crate::objects::plane::Plane;

use crate::objects::geometry::Geometry;

use math::Vec3;

pub fn create_world() -> Vec<Box<Geometry>> {
    let color = Vec3::new_color(0, 0, 0);

    let sphere = Sphere::new(Vec3::new(0., 0., 1.), 0.5, Vec3::new_color(32, 32, 64));
    let sphere_1 = Sphere::new(Vec3::new(0.0, 0., 0.), 1., Vec3::new_color(0, 32, 0));
    let sphere_2 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.25, Vec3::new_color(0, 0, 255));
    let sphere_3 = Sphere::new(Vec3::new(-1.3, 0., 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_4 = Sphere::new(Vec3::new(1.3, 0., 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_5 = Sphere::new(Vec3::new(0.0, 0.5, 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_7 = Sphere::new(Vec3::new(0.0, 0.0, 1.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_6 = Sphere::new(Vec3::new(0.0, -0.5, 0.), 0.1, Vec3::new_color(0, 255, 255));
    let plane = Plane::new(Vec3::new(0., 0., -0.5), Vec3::new(0., 0.5, 0.5), color);
    let plane_1 = Plane::new(Vec3::new(0., 0., -1.0), Vec3::new(0., 0., 1.), Vec3::new_color(64, 0, 0));

    let mut geometry: Vec<Box<Geometry>> = Vec::new();
    geometry.push(Box::new(sphere));
    geometry.push(Box::new(plane));
    geometry.push(Box::new(plane_1));
    // geometry.push(Box::new(sphere_1));
    // geometry.push(Box::new(sphere_2));
    // geometry.push(Box::new(sphere_3));
    // geometry.push(Box::new(sphere_4));
    // geometry.push(Box::new(sphere_5));
    // geometry.push(Box::new(sphere_6));
    // geometry.push(Box::new(sphere_7));

    geometry
}