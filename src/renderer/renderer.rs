use rayon::prelude::*;
use std::sync::{Arc, Mutex};

extern crate nalgebra_glm as nlm;

extern crate image;
use image::{ RgbImage };

use crate::objects::{ Geometry, Ray, HitData };
use crate::cameras::Camera;
use crate::world:: { World };

pub struct RenderSettings {
    pub sampling_count: u32,
    pub work_per_thread: u32,
    pub image_width: u32,
    pub image_height: u32,
    pub max_subrays_count: u32,
}

impl RenderSettings {
    pub fn new(image_width: u32, image_height: u32, sampling_count: u32, max_subrays_count: u32, work_per_thread: u32) -> RenderSettings {
        RenderSettings {
            sampling_count,
            work_per_thread,
            image_width,
            image_height,
            max_subrays_count
        }
    }
}

fn trace(ray: &Ray, geometry: &Vec<Box<dyn Geometry>>, camera: &Camera, world_color: nlm::Vec3, depth: u32, max_depth: u32, rng: &mut rand::rngs::ThreadRng) -> nlm::Vec3 {
    if geometry.is_empty() {
        return world_color;
    }

    if depth > max_depth {
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
    let is_hit = hit_data.is_hit;
    if is_hit {
        hit_data.ray = ray.copy();
        hit_data.normal = hit_obj.get_normal(&ray, &hit_data);

        let material = hit_obj.get_mateial();
        let mut ray = Ray::zeros();
        material.scatter(&mut hit_data, &mut result_color, &mut ray);

        let new_color = &trace(&ray, geometry, camera, world_color, depth + 1, max_depth, rng);

        //This is odd formula
        result_color.x = result_color.x * new_color.x;
        result_color.y = result_color.y * new_color.y;
        result_color.z = result_color.z * new_color.z;

    } else {

    }
    result_color
}

fn trace_camera_ray(x: f32, y: f32, world: &World, render_settings: &RenderSettings, rng: &mut rand::rngs::ThreadRng) -> nlm::Vec3 {
    let ray = world.camera.get_ray(x, y);
    let result_color = trace(&ray, &world.objects, &world.camera, world.color.clone(), 0, render_settings.max_subrays_count, rng);
    result_color
}

struct ThreadWork {
    start: u32,
    end: u32,
    pixels: Vec<nlm::Vec3>,
}

impl ThreadWork {
    fn new(start: u32, end: u32, pixels: u32) -> ThreadWork {
        ThreadWork {
            start,
            end,
            pixels: Vec::<nlm::Vec3>::with_capacity(pixels as usize),
        }
    }
}

pub fn render(render_settings: &RenderSettings) -> RgbImage {
    let image_width = render_settings.image_width;
    let sampling_count = render_settings.sampling_count;
    let image_height = render_settings.image_height;

    let mut img: RgbImage = image::ImageBuffer::new(image_width, image_height);
    let world = World::default_for_test(image_width as f32, image_height as f32);

    let work_per_thread = render_settings.work_per_thread; //Pixels
    let total_pixels = (image_width * image_height) as u32;
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
        thread_works.push(ThreadWork::new(start, end, work_per_thread));
    }

    let done = Arc::new(Mutex::<u32>::new(0));

    let render_start_time = std::time::Instant::now();
    thread_works.par_iter_mut().for_each(|work| {
        let mut rng = rand::thread_rng();
        for i in work.start..work.end {
            let x = i % image_width;
            let y = i / image_width;

            let pixel_x = x as f32 + 0.5;
            let pixel_y = y as f32 + 0.5;
            let mut pixel_color = nlm::Vec3::zeros();

            for _ in 0..sampling_count {
                let x = pixel_x + rand::random::<f32>() - 0.5;
                let y = pixel_y + rand::random::<f32>() - 0.5;
                
                let color = trace_camera_ray(x, y, &world, render_settings, &mut rng);
                pixel_color = pixel_color + &color;
            }

            pixel_color = pixel_color * 1. / (sampling_count as f32);
            work.pixels.push(pixel_color);
        }

        let mut done_ref = done.lock().unwrap();
        let p = *done_ref * 100 / works_count;
        if p % 5 == 0 {
            println!("{}%", p);
        }
        *done_ref += 1;
    });

    let ms = render_start_time.elapsed().as_millis();
    println!("renderer::render time = {}ms", ms);

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

    img
}