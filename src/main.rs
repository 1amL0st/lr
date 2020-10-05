extern crate image;
use image::{ RgbImage };

use math::Vec3;

mod objects;
use objects::geometry:: {Geometry, Ray, HitData };
use objects::sphere::Sphere;
use objects::plane::Plane;

mod cameras;
use cameras::camera::Camera;

fn trace(x: f32, y: f32, geometry: &Vec<Box<Geometry>>, camera: &Camera, world_color: Vec3) -> Vec3
{
    let ray = camera.get_ray(x, y);

    let mut hit_data = HitData::empty();
    let mut hit_obj = &geometry[0];
    let mut shortest = std::f32::INFINITY;

    for obj in geometry {
        let data = obj.hit(&ray);
        if data.is_hit {
            if data.t < shortest {
                shortest = data.t;
                hit_obj = obj;
                hit_data = data;
            }
        }
    }

    let mut result_color = world_color;
    if hit_data.is_hit {
        hit_data.normal = hit_obj.get_normal(&ray, &hit_data);
        let angle = -hit_data.normal.dir.dot(&ray.dir);
        result_color = hit_obj.get_color(&mut hit_data).scale(angle);
    }

    result_color
}

/*
    This function generates result image and save it in file 
*/
fn render(file_name: &String, image_width: u32, image_height: u32)
{
    let mut img: RgbImage = image::ImageBuffer::new(image_width, image_height);

    let sphere = Sphere::new(Vec3::new(0., 0., 0.), 0.5, Vec3::new_color(255, 0, 0));
    let sphere_1 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.35, Vec3::new_color(0, 255, 0));
    let sphere_2 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.25, Vec3::new_color(0, 0, 255));
    let sphere_3 = Sphere::new(Vec3::new(-1.3, 0., 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_4 = Sphere::new(Vec3::new(1.3, 0., 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_5 = Sphere::new(Vec3::new(0.0, 0.5, 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_6 = Sphere::new(Vec3::new(0.0, -0.5, 0.), 0.1, Vec3::new_color(0, 255, 255));
    let plane = Plane::new(Vec3::new(0., 0., 0.2), Vec3::new(0., 0.5, 0.5), Vec3::new(0.0, 0., 0.32));

    let mut geometry: Vec<Box<Geometry>> = Vec::new();
    geometry.push(Box::new(sphere));
    geometry.push(Box::new(plane));
    geometry.push(Box::new(sphere_1));
    geometry.push(Box::new(sphere_2));
    geometry.push(Box::new(sphere_3));
    geometry.push(Box::new(sphere_4));
    geometry.push(Box::new(sphere_5));
    geometry.push(Box::new(sphere_6));

    let camera_dir = Vec3::new(0., 0., -1.);
    let camera_pos = Vec3::new(0., 0., 2.);
    let camera = Camera::new(camera_pos, camera_dir, 30.0, image_width as f32, image_height as f32);

    let sampling_count = 4;
    //let world_color = Vec3::new_color(255, 255, 255);
    let world_color = Vec3::new_color(0, 0, 0);

    for y in 0..image_height {
        for x in 0..image_width {
            let mut pixel = *img.get_pixel_mut(x, y);

            let pixel_x = x as f32 + 0.5;
            let pixel_y = y as f32 + 0.5;
            let mut pixel_color = Vec3::empty();

            for count in 0..sampling_count {
                let x = pixel_x + rand::random::<f32>() - 0.5;
                let y = pixel_y + rand::random::<f32>() - 0.5;
                let color = trace(x, y, &geometry, &camera, world_color.clone());
                pixel_color = pixel_color.add(&color);
            }

            pixel_color = pixel_color.scale(1. / (sampling_count as f32));

            pixel[0] = (pixel_color.x * 255.) as u8;
            pixel[1] = (pixel_color.y * 255.) as u8;
            pixel[2] = (pixel_color.z * 255.) as u8;
            img.put_pixel(x, y, pixel);
        }
    }

    img.save(file_name).unwrap();
}

fn main() {
    render(&String::from("./rendered/output.png"), 1280, 720);
}
