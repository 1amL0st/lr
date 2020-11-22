use crate::objects::{ sphere::Sphere, plane::Plane, Geometry };
use crate::cameras::{ Camera };

use nlm;
use crate::utils::NlmVec3Ext;

pub struct World {
    pub objects: Vec<Box<Geometry>>,
    pub camera: Camera,
    pub color: nlm::Vec3
}


impl World {
    pub fn new(objects: Vec<Box<Geometry>>, camera: Camera, color: nlm::Vec3) -> World {
        World {
            objects,
            camera,
            color
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    fn random_color(rng: &mut rand::rngs::ThreadRng) -> nlm::Vec3 {
        return nlm::Vec3::new_color(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255));
    }

    fn random_spheres(objects: &mut Vec<Box<Geometry>>) {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let sphere_size = rng.gen_range(0.25, 1.);
            let sphere_pos = nlm::vec3(rng.gen_range(-5., 5.), sphere_size, rng.gen_range(-10., 5.));
            let color = World::random_color(&mut rng);
            let sphere = Sphere::new(sphere_pos, sphere_size, color);
            objects.push(Box::new(sphere));
        }

        let plane_color = nlm::Vec3::new_color(64, 64, 64);
        let plane = Plane::new(nlm::vec3(0., 0., 0.), nlm::vec3(0., 1., 0.), plane_color);
        objects.push(Box::new(plane));
    }

    pub fn default_for_test(image_width: f32, image_height: f32) -> World {
        let camera_pos = nlm::Vec3::new(-10., 0.5, 10.);
        let look_at_point = nlm::Vec3::new(0., 0., 0.);
        let camera = Camera::new(camera_pos, look_at_point, 30.0, image_width, image_height);

        let mut objects: Vec<Box<Geometry>> = Vec::new();
        World::random_spheres(&mut objects);

        let world_color = nlm::Vec3::new_color(255, 255, 255);
        World::new(objects, camera, world_color)
    }
}

use rand::Rng;