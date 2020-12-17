extern crate nalgebra_glm as nlm;
extern crate image;

mod utils;
use utils::NlmVec3Ext;

mod objects;
mod cameras;
mod world;
mod constants;
mod materials;
mod renderer;
use renderer::renderer::{ RenderSettings, render };

use std::{num::ParseIntError, path};
use std::env;
use std::fs;

fn select_render_img_path(file_name: &String) -> Option<String> {
    let current_path = env::current_dir().unwrap();
    let absolute_file_path = format!("{}{}", current_path.to_str().unwrap(), file_name.as_str());
    let path = path::Path::new(&absolute_file_path);

    if path.exists() && path.is_file() {
        let parent_folder = path.parent().unwrap();

        let name = path.file_stem()?.to_str()?;
        let extension = path.extension()?.to_str()?;
        path.extension()?;

        let mut match_count: u32 = 0;
        for item in fs::read_dir(parent_folder).unwrap() {
            let entry = item.unwrap();
            if entry.file_name().to_str().unwrap().contains(name) {
                match_count += 1;
            }
        }
        let new_file_name = format!("{}/{}_{}.{}", parent_folder.to_str().unwrap(), name, match_count.to_string(), extension);
        Some(String::from(new_file_name))
    } else {
        Some(file_name.clone())
    }
}

fn format_mseconds_time(ms: u128) -> String {
    // TODO: This function mithgt be dangerous (possible panic!)
    let hours = ms / (3600 * 1000);
    let minutes = (ms - hours * 3600 * 1000) / (1000 * 60);
    let seconds = (ms - (hours * 60 + minutes) * 60 * 1000) / 1000;
    let ms = ms - (hours * 3600 + minutes * 60 + seconds) * 1000;
    format!("{:02}:{:02}:{:02}:{:02}", hours, minutes, seconds, ms)
}

fn main() {
    // println!("{}", nlm::reflect_vec(&nlm::Vec3::new(1., 1., 0.), &nlm::Vec3::new(1.0, 0., 0.)));
    let v = nlm::Vec3::new(1.0, 1.0, 0.);
    let n = nlm::Vec3::new(1., 0., 0.);

    let reflected = -(v - ((2.0 * v.dot(&n)) * &n)).normalize();
    println!("{}", reflected);
    // panic!();

    let render_settings = RenderSettings::new(
        720, 360, // Image width and height
        2, 4, // Sampling and subrays limit
        16, // Work per thread
    );

    let generation_start_time = std::time::Instant::now();
    let mut file_name = format!("./rendered/{}samples.png", render_settings.sampling_count).to_string();
    // file_name = select_render_img_path(&file_name).unwrap();

    let result_img = render(&render_settings);
    result_img.save(file_name).unwrap();

    let ms = generation_start_time.elapsed().as_millis();
    // println!("Image generation time in ms = {}", ms);
    println!("Image generation time {}", format_mseconds_time(ms));
}
