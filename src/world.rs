use crate::objects::{ sphere::Sphere, plane::Plane, Geometry };
use crate::cameras::{ Camera };

use nlm;
use crate::utils::NlmVec3Ext;

pub struct World {
    pub objects: Vec<Box<Geometry>>,
    pub camera: Camera
}

impl World {
    pub fn new(objects: Vec<Box<Geometry>>, camera: Camera) -> World {
        World {
            objects,
            camera
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn default_for_test(image_width: f32, image_height: f32) -> World {
        let objects = create_world();

        let camera_dir = nlm::Vec3::new(0., 0., -1.);
        let camera_pos = nlm::Vec3::new(0., 0., 10.);
        let camera = Camera::new(camera_pos, camera_dir, 30.0, image_width, image_height);

        World::new(objects, camera)
    }
}

pub fn create_world() -> Vec<Box<Geometry>> {
    let color = nlm::Vec3::new_color(0, 0, 0);

    let sphere = Sphere::new(nlm::Vec3::new(0., 0., 0.5), 0.5, nlm::Vec3::new_color(32, 32, 64));
    let sphere_1 = Sphere::new(nlm::Vec3::new(1.0, 0., 0.), 1., nlm::Vec3::new_color(0, 32, 0));
    let sphere_2 = Sphere::new(nlm::Vec3::new(-1.0, 0., 0.), 0.25, nlm::Vec3::new_color(0, 0, 255));
    let sphere_3 = Sphere::new(nlm::Vec3::new(-1.3, 0., 0.), 0.1, nlm::Vec3::new_color(0, 255, 255));
    let sphere_4 = Sphere::new(nlm::Vec3::new(1.3, 0., 0.), 0.1, nlm::Vec3::new_color(0, 255, 255));
    let sphere_5 = Sphere::new(nlm::Vec3::new(0.0, 0.5, 0.), 0.1, nlm::Vec3::new_color(0, 255, 255));
    let sphere_7 = Sphere::new(nlm::Vec3::new(0.0, 0.0, 1.), 0.1, nlm::Vec3::new_color(0, 255, 255));
    let sphere_6 = Sphere::new(nlm::Vec3::new(0.0, -0.5, 0.), 0.1, nlm::Vec3::new_color(0, 255, 255));
    let plane = Plane::new(nlm::Vec3::new(0., 0., -0.5), nlm::Vec3::new(0., 0.5, 0.5), color);
    let plane_1 = Plane::new(nlm::Vec3::new(0., 0., -1.0), nlm::Vec3::new(0., 0., 1.), nlm::Vec3::new_color(64, 0, 0));

    let mut geometry: Vec<Box<Geometry>> = Vec::new();
    geometry.push(Box::new(sphere));
    geometry.push(Box::new(plane));
    geometry.push(Box::new(plane_1));
    geometry.push(Box::new(sphere_1));
    geometry.push(Box::new(sphere_2));
    // geometry.push(Box::new(sphere_3));
    // geometry.push(Box::new(sphere_4));
    // geometry.push(Box::new(sphere_5));
    // geometry.push(Box::new(sphere_6));
    // geometry.push(Box::new(sphere_7));

    geometry
}