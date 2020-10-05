use math::Vec3;
use crate::objects::geometry::Ray;

pub struct Camera
{
    pos: Vec3,
    dir: Vec3,
    image_width: f32,
    image_height: f32,
    norm_width: f32,
    norm_height: f32,
    aspect: f32,
    field_of_view: f32 //In radians
}

impl Camera {
    pub fn new(pos: Vec3, dir: Vec3, field_of_view_degrees: f32, image_width: f32, image_height: f32) -> Camera {
        Camera {
            pos: pos,
            dir: dir,
            image_width,
            image_height,
            norm_width: 2. / image_width,
            norm_height: 2. / image_height,
            aspect: image_width / image_height,
            field_of_view: ((field_of_view_degrees).to_radians() * 0.5).tan()
    }
    }

    pub fn get_ray(&self, mut pixel_x: f32, mut pixel_y: f32) -> Ray {
        pixel_x = ((pixel_x) * self.norm_width - 1.) * self.aspect * self.field_of_view;
        pixel_y = (1. - (pixel_y) * self.norm_height) * self.field_of_view;

        Ray::new(Vec3::new(pixel_x, pixel_y, self.pos.z), Vec3::new(pixel_x, pixel_y, self.dir.z).norm())
    }
}