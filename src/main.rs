extern crate image;
use image::{ RgbImage };

use math::Vec3;

mod objects;
use objects::geometry:: {Geometry, Ray, HitData };
use objects::sphere::Sphere;
use objects::plane::Plane;

mod cameras;
use cameras::camera::Camera;

fn trace(x: f32, y: f32, geometry: &Vec<Box<Geometry>>, camera: &Camera, world_color: Vec3) -> Vec3 {
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

use std::thread;
use std::sync::mpsc::{
    channel, Sender, Receiver
};

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
    geometry.push(Box::new(plane));
    geometry.push(Box::new(sphere_1));
    geometry.push(Box::new(sphere_2));
    geometry.push(Box::new(sphere_3));
    geometry.push(Box::new(sphere_4));
    geometry.push(Box::new(sphere_5));
    geometry.push(Box::new(sphere_6));
    geometry.push(Box::new(sphere_7));
    geometry.push(Box::new(Sphere::new(Vec3::new(-1., 0., 0.), 0.5, Vec3::new_color(0, 128, 0))));
    geometry.push(Box::new(Sphere::new(Vec3::new(1., 0., 0.), 0.5, Vec3::new_color(0, 128, 0))));

    geometry
}

/*
    This function generates result image and save it in file 
*/

use rayon::prelude::*;

fn render(file_name: &String, image_width: u32, image_height: u32) {
    let mut img: RgbImage = image::ImageBuffer::new(image_width, image_height);

    let camera_dir = Vec3::new(0., 0., -1.);
    let camera_pos = Vec3::new(0., 0., 2.);
    let camera = Camera::new(camera_pos, camera_dir, 30.0, image_width as f32, image_height as f32);

    let geometry = create_world();

    let sampling_count = 4;
    //let world_color = Vec3::new_color(255, 255, 255);
    let world_color = Vec3::new_color(0, 0, 0);

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

    thread_works.par_iter_mut().for_each(|work| {
        for i in work.start..work.end {
            let color = trace(0., 0., &geometry, &camera, world_color);
            work.pixels.push(color);
        }
    });

    // let threads_count = 1;
    // let mut threads = vec![];

    // for i in 0..threads_count {
    //     threads.push(thread::spawn(move || {
    //         // println!("Thread = {}", i);
    //     }));
    // }

    // let pool: Vec<ThreadChannel> = Vec::new();
    // for i in 0..threads_count {
    //     pool.push(ThreadChannel::new());
    // }

    // let a = Arc::new(geometry);

    // for i in 0..threads_count {
    //     let thread_proc = |geometry: &Vec<Box<Geometry>>| {

    //     };
    //     let thread = thread::spawn(|| {
    //         thread_proc(&geometry);
    //     });
    // }

    // let first_thread = thread::spawn(move || {
    //     while true {
    //         let result = rrx.try_recv();
    //         if result.is_ok() {
    //             println!("I've got data from out thread!");
    //             break;
    //         }
    //     }
    // });

    // let works_done = 0u32;
    // while works_done < works_count {
    //     for i in 0..threads.len() {
    //         let value = threads[i].join();
    //     }
    // }

    // let (tx, rx) = channel();
    // thread::spawn(move|| {
    //     tx.send(10).unwrap();
    // });
    // let first_thread;

    // let string: String = String::from("Hey! ");
    // let func = |v: &String| {
    //     let mut value = v.clone();
    //     return value + "hello!";
    // };

    // println!("{}", func(&string));
    // println!("{}", func(&string));

    /*
    let work_per_thread = 32; //Pixels
    let total_pixels = (camera.image_width * camera.image_height) as u32;
    let mut works_count: u32 = (total_pixels / work_per_thread) + 1;
    if total_pixels % work_per_thread == 0 {
        works_count = works_count - 1;
    }

    let test = vec![1, 2, 3];
    test.clone();

    let mut children = vec![];
    let threads_count = 16;

    for i in 0..threads_count {
        let arc = std::sync::Arc::new(thread_works.clone());
        children.push(thread::spawn(move || {
            while (!thread_works[(works_count - 1) as usize].status.is_done()) {
                println!("this is thread number {}", i);
            }
        }));
    }
    
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    */
    // for y in 0..image_height {
    //     for x in 0..image_width {
    //         let mut pixel = *img.get_pixel_mut(x, y);

    //         let pixel_x = x as f32 + 0.5;
    //         let pixel_y = y as f32 + 0.5;
    //         let mut pixel_color = Vec3::empty();

    //         for count in 0..sampling_count {
    //             let x = pixel_x + rand::random::<f32>() - 0.5;
    //             let y = pixel_y + rand::random::<f32>() - 0.5;
    //             let color = trace(x, y, &geometry, &camera, world_color.clone());
    //             pixel_color = pixel_color.add(&color);
    //         }

    //         pixel_color = pixel_color.scale(1. / (sampling_count as f32));

    //         pixel[0] = (pixel_color.x * 255.) as u8;
    //         pixel[1] = (pixel_color.y * 255.) as u8;
    //         pixel[2] = (pixel_color.z * 255.) as u8;
    //         img.put_pixel(x, y, pixel);
    //     }
    // }

    // thread::spawn(|| {
    //     println!("Hello my new thread! {}", var);
    // });

    img.save(file_name).unwrap();
}

fn main() {
    render(&String::from("./rendered/output.png"), 1280, 720);
}
