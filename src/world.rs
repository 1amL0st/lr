use crate::{materials::Dielectric, objects::{ sphere::Sphere, plane::Plane, Geometry }};
use crate::cameras::{ Camera };
use crate::materials:: { Lambertian, Metal };

use nlm;
use crate::utils::NlmVec3Ext;

use std::sync::Arc;
// use std::rc::Arc;

pub struct World {
    pub objects: Vec<Box<dyn Geometry>>,
    pub camera: Camera,
    pub color: nlm::Vec3
}

impl World {
    pub fn new(objects: Vec<Box<dyn Geometry>>, camera: Camera, color: nlm::Vec3) -> World {
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

    fn random_spheres(objects: &mut Vec<Box<dyn Geometry>>) {
        let lambertian = Arc::new(Lambertian::new(nlm::Vec3::new_color(128, 128, 128)));

        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let sphere_size = rng.gen_range(0.25, 1.);
            let sphere_pos = nlm::vec3(rng.gen_range(-5., 5.), sphere_size, rng.gen_range(-10., 5.));
            let color = World::random_color(&mut rng);
            let sphere = Sphere::new(sphere_pos, sphere_size, lambertian.clone());
            objects.push(Box::new(sphere));
        }

        let plane_color = nlm::Vec3::new_color(64, 64, 64);
        let plane = Plane::new(nlm::vec3(0., 0., 0.), nlm::vec3(0., 1., 0.), lambertian.clone());
        objects.push(Box::new(plane));
    }

    pub fn randoom_for_test(image_width: f32, image_height: f32) -> World {
        let camera_pos = nlm::Vec3::new(-10., 0.5, 10.);
        let look_at_point = nlm::Vec3::new(0., 0., 0.);
        let camera = Camera::new(camera_pos, look_at_point, 30.0, image_width, image_height);

        let mut objects: Vec<Box<dyn Geometry>> = Vec::new();
        World::random_spheres(&mut objects);

        let world_color = nlm::Vec3::new_color(255, 255, 255);
        World::new(objects, camera, world_color)
    }

    fn sphere_with_color(pos: nlm::Vec3, radius: f32, color: nlm::Vec3) -> Box<Sphere> {
        let lambertian = Arc::new(Lambertian::new(color));
        Box::new(Sphere::new(pos, radius, lambertian))
    }

    pub fn default_test_spheres(objects: &mut Vec<Box<dyn Geometry>>) {
        objects.push(Sphere::matte_with_color(nlm::vec3(0., 1., 0.0), 1., nlm::Vec3::new_color(0, 0, 128)));

        objects.push(Sphere::matte_with_color(nlm::vec3(3., 0.25, 4.0), 0.25, nlm::Vec3::new_color(128, 0, 128)));
        objects.push(Sphere::matte_with_color(nlm::vec3(-3., 0.25, 4.0), 0.25, nlm::Vec3::new_color(32, 32, 128)));
        objects.push(Sphere::matte_with_color(nlm::vec3(1., 0.25, 7.0), 0.25, nlm::Vec3::new_color(0, 128, 128)));

        objects.push(Box::new(Sphere::new(
            nlm::vec3(3.0, 1., 0.0),
            1.,
            std::sync::Arc::new(Dielectric::new(1.25))
        )));
        objects.push(Box::new(Sphere::new(
            nlm::vec3(-3.0, 1., 0.0),
            1.0,
            std::sync::Arc::new(Dielectric::new(1.85))
        )));

        objects.push(Box::new(Sphere::new(
            nlm::vec3(-3.0, 1., 0.0),
            0.5,
            std::sync::Arc::new(Dielectric::new(1.45))
        )));

        objects.push(
            Plane::matte_with_color(nlm::vec3(0., 0., 0.), nlm::vec3(0., 1., 0.), nlm::Vec3::new_color(128, 128, 128))
        );
    }

    pub fn default_for_test(image_width: f32, image_height: f32) -> World {
        let camera_pos = nlm::Vec3::new(0., 0.5, 10.);
        let look_at_point = nlm::Vec3::new(0., 0.5, 0.);
        let camera = Camera::new(camera_pos, look_at_point, 30.0, image_width, image_height);

        let mut objects: Vec<Box<dyn Geometry>> = Vec::new();
        World::default_test_spheres(&mut objects);

        // let world_color = nlm::Vec3::new_color(255, 255, 255);
        let world_color = nlm::Vec3::new_color(255, 255, 255);
        World::new(objects, camera, world_color)
    }
}

use rand::Rng;