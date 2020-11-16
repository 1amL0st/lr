extern crate nalgebra_glm as nlm;
extern crate image;

mod utils;
use utils::NlmVec3Ext;

mod objects;
mod cameras;
mod world;
mod constants;
mod renderer;
use renderer::renderer::{ RenderSettings, render };

fn main() {
    

    const SUBRAYS_LIMIT: u32 = 2u32;
    const SAMPLING_COUNT: u32 = 2u32;
    const WORK_PER_THREAD: u32 = 16;
    const IMAGE_WIDTH: u32 = 720;
    const IMAGE_HEIGHT: u32 = 360;
    let world_color: nlm::Vec3 = nlm::Vec3::new_color(128, 128, 128);

    let render_settings = RenderSettings::new(
        IMAGE_WIDTH, IMAGE_HEIGHT,
        SAMPLING_COUNT, world_color,
         SUBRAYS_LIMIT, WORK_PER_THREAD
    );

    let generation_start_time = std::time::Instant::now();
    let file_name = format!("./rendered/{}samples.png", render_settings.sampling_count).to_string();
    render(&file_name, &render_settings);

    let ms = generation_start_time.elapsed().as_millis();
    println!("Image generation time {}ms", ms);
}
