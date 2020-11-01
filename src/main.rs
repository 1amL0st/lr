extern crate image;
use image::{ RgbImage };

use math::Vec3;

mod objects;
use objects::geometry:: {Geometry, Ray, HitData };
use objects::sphere::Sphere;
use objects::plane::Plane;

mod cameras;
use cameras::camera::Camera;

use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use rand::Rng;

fn point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut point = Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    while point.len() >= 1.0 {
        point = Vec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), rng.gen_range(-1., 1.));
    }
    point
}

const SUBRAYS_LIMIT: u32 = 32u32;

fn trace(ray: &Ray, geometry: &Vec<Box<Geometry>>, camera: &Camera, world_color: Vec3, depth: u32) -> Vec3 {
    if geometry.is_empty() {
        return world_color;
    }
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
        let target = point_in_unit_sphere().add(&hit_data.normal.pos).add(&hit_data.normal.dir);
        let new_ray = Ray::new(hit_data.normal.pos, target.sub(&hit_data.normal.pos).norm());
        result_color = hit_obj.get_color(&hit_data);
        if depth < SUBRAYS_LIMIT {
            result_color = result_color.add(&trace(&new_ray, geometry, camera, world_color, depth + 1).scale(0.5));
        }
    } else {
    }
    result_color
}

fn trace_camera_ray(x: f32, y: f32, geometry: &Vec<Box<Geometry>>, camera: &Camera, world_color: Vec3) -> Vec3 {
    let ray = camera.get_ray(x, y);
    let mut result_color = trace(&ray, geometry, camera, world_color, 0);
    result_color
}

struct ThreadWork {
    start: u32,
    end: u32,
    id: u32,
    pixels: Vec<Vec3>,
}

impl ThreadWork {
    fn new(start: u32, end: u32, id: u32, pixels: u32) -> ThreadWork {
        ThreadWork {
            start,
            end,
            id,
            pixels: Vec::<Vec3>::with_capacity(pixels as usize),
        }
    }
}

fn create_world() -> Vec<Box<Geometry>> {
    let sphere = Sphere::new(Vec3::new(0., 0., 0.), 0.5, Vec3::new_color(255, 0, 0));
    let sphere_1 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.35, Vec3::new_color(0, 255, 0));
    let sphere_2 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.25, Vec3::new_color(0, 0, 255));
    let sphere_3 = Sphere::new(Vec3::new(-1.3, 0., 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_4 = Sphere::new(Vec3::new(1.3, 0., 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_5 = Sphere::new(Vec3::new(0.0, 0.5, 0.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_7 = Sphere::new(Vec3::new(0.0, 0.0, 1.), 0.1, Vec3::new_color(0, 255, 255));
    let sphere_6 = Sphere::new(Vec3::new(0.0, -0.5, 0.), 0.1, Vec3::new_color(0, 255, 255));
    let plane = Plane::new(Vec3::new(0., 0., 0.2), Vec3::new(0., 0.5, 0.5), Vec3::new(0.0, 0., 0.32));

    let mut geometry: Vec<Box<Geometry>> = Vec::new();
    geometry.push(Box::new(sphere));
    // geometry.push(Box::new(plane));
    // geometry.push(Box::new(sphere_1));
    // geometry.push(Box::new(sphere_2));
    // geometry.push(Box::new(sphere_3));
    // geometry.push(Box::new(sphere_4));
    // geometry.push(Box::new(sphere_5));
    // geometry.push(Box::new(sphere_6));
    // geometry.push(Box::new(sphere_7));
    // geometry.push(Box::new(Sphere::new(Vec3::new(-1., 0., 0.), 0.5, Vec3::new_color(0, 128, 0))));
    // geometry.push(Box::new(Sphere::new(Vec3::new(1., 0., 0.), 0.5, Vec3::new_color(0, 128, 0))));

    geometry
}

/*
    This function generates result image and save it in file 
*/


fn render(file_name: &String, image_width: u32, image_height: u32) {
    let mut img: RgbImage = image::ImageBuffer::new(image_width, image_height);

    let camera_dir = Vec3::new(0., 0., -1.);
    let camera_pos = Vec3::new(0., 0., 5.);
    let camera = Camera::new(camera_pos, camera_dir, 30.0, image_width as f32, image_height as f32);

    let geometry = create_world();

    let sampling_count = 16;
    let world_color = Vec3::new_color(16, 16, 64);

    let work_per_thread = 32; //Pixels
    let total_pixels = (camera.image_width * camera.image_height) as u32;
    let mut works_count: u32 = (total_pixels / work_per_thread) + 1;
    if total_pixels % work_per_thread == 0 {
        works_count = works_count - 1;
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let mut thread_works: Vec<ThreadWork> = Vec::with_capacity(works_count as usize);
    for i in 0..works_count {
        let start = work_per_thread * i;
        let end = (work_per_thread * (i + 1)).min(total_pixels);
        thread_works.push(ThreadWork::new(start, end, i, work_per_thread));
    }

    let done_works = Arc::new(Mutex::new(0));
    let render_start_time = std::time::Instant::now();

    thread_works.par_iter_mut().for_each(|work| {
        for i in work.start..work.end {
            let x = i % image_width;
            let y = i / image_width;

            let pixel_x = x as f32 + 0.5;
            let pixel_y = y as f32 + 0.5;
            let mut pixel_color = Vec3::empty();

            for _ in 0..sampling_count {
                let x = pixel_x + rand::random::<f32>() - 0.5;
                let y = pixel_y + rand::random::<f32>() - 0.5;
                let color = trace_camera_ray(x, y, &geometry, &camera, world_color.clone());
                pixel_color = pixel_color.add(&color);
            }

            pixel_color = pixel_color.scale(1. / (sampling_count as f32));
            work.pixels.push(pixel_color);
        }

        let mut done = done_works.lock().unwrap();
        let p = *done * 100 / works_count;
        if p % 5 == 0 {
            println!("{}%", p);
        }
        *done += 1;
    });

    let ms = render_start_time.elapsed().as_millis();
    println!("This render took = {}ms", ms);

    thread_works.iter().for_each(|work| {
        for i in work.start..work.end {
            let x = i % image_width;
            let y = i / image_width;

            let mut pixel = *img.get_pixel_mut(x, y);
            let pixel_color = work.pixels[(i - work.start) as usize];
            pixel[0] = (pixel_color.x * 255.) as u8;
            pixel[1] = (pixel_color.y * 255.) as u8;
            pixel[2] = (pixel_color.z * 255.) as u8;
            img.put_pixel(x, y, pixel);
        }
    });

    img.save(file_name).unwrap();
}

fn main() {
    let generation_start_time = std::time::Instant::now();

    render(&String::from("./rendered/output.png"), 720, 360);

    let ms = generation_start_time.elapsed().as_millis();
    println!("Image generation time {}ms", ms);
}
