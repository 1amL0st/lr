use crate::objects::Ray;
use nlm;
pub struct Camera
{
    pub matrix: nlm::Mat4x4,
    pub image_width: f32,
    pub image_height: f32,
    pub norm_width: f32,
    pub norm_height: f32,
    pub aspect: f32,
    pub field_of_view: f32 //In radians
}

impl Camera {
    pub fn new(camera_pos: nlm::Vec3, look_at_point: nlm::Vec3, field_of_view_degrees: f32, image_width: f32, image_height: f32) -> Camera {
        let mut camera_up: nlm::Vec3;
        // Это какой-то костыль
        if (camera_pos.x != 0.0) {
            camera_up = nlm::vec3(0., 1., 0.);
        } else {
            camera_up = nlm::vec3(1., 0., 0.);
        }
        camera_up = nlm::vec3(0., 1., 0.);
        
        let matrix = nlm::look_at(&camera_pos, &look_at_point, &camera_up).try_inverse().unwrap();
        Camera {
            matrix,
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

        let mut dir = nlm::Vec4::new(pixel_x, pixel_y, -1., 0.);
        let mut pos = nlm::Vec4::new(pixel_x, pixel_y, 0., 1.);

        dir = (self.matrix * dir).normalize();
        pos = self.matrix * pos;

        Ray::new(
            nlm::vec4_to_vec3(&pos),
            nlm::vec4_to_vec3(&dir)
        )
    }
}