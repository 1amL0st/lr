/*
 * Use alt + -> / <- go to previous/next cursor position
*/

extern crate image;
use image::{ RgbImage };

extern crate math;
use math::vec3::Vec3;
use Vec3 as RgbColor;

struct Normal
{
    pos: Vec3,
    dir: Vec3
}

impl Normal {
    fn new(pos: Vec3, dir: Vec3) -> Normal {
        Normal {
            pos,
            dir
        }
    }

    fn empty() -> Normal {
        Normal {
            pos: Vec3::empty(),
            dir: Vec3::empty()
        }
    }
}

struct HitData {
    is_hit: bool,
    t: f32,
    normal: Normal
}

impl HitData {
    fn empty() -> HitData {
        let is_hit = false;
        return HitData {
            is_hit,
            t: 0.,
            normal: Normal::empty()
        }
    }
}

trait Geometry {
    fn hit(&self, ray: &Ray) -> HitData;
    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal;
    fn get_color(&self, hit: &HitData) -> RgbColor;
}

struct Plane {
    pos: Vec3,
    normal: Vec3
}

impl Plane {
    fn new(pos: Vec3, normal: Vec3) -> Plane {
        Plane {
            pos,
            normal
        }
    }
}

impl Geometry for Plane {
    fn hit(&self, ray: &Ray) -> HitData {
        let point = Vec3::new(0., 0., 0.);
        let normal = Vec3::new(0., 0., 0.);

        HitData {
            is_hit: false,
            t: 0.,
            normal: Normal::empty()
        }
    }

    fn get_color(&self, hit: &HitData) -> RgbColor {
        RgbColor::new(1., 1., 1.)
    }

    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal {
        Normal::empty()
    }
}

struct Sphere {
    pos: Vec3,
    radius: f32,
    color: RgbColor
}

impl Sphere {
    fn new(pos: Vec3, radius: f32, color: RgbColor) -> Sphere {
        Sphere {
            pos,
            radius,
            color
        }
    }
}

fn f32_max(first: f32, second: f32) -> f32 {
    if first > second {
        return first
    }
    second
}

fn f32_min(first: f32, second: f32) -> f32 {
    if first < second {
        return first
    }
    second
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray) -> HitData {
        let len = ray.pos.sub(&self.pos);
        let a = ray.dir.dot(&ray.dir);
        let b = 2. * ray.dir.dot(&len);
        let c = len.dot(&len) - self.radius * self.radius;
        
        let d = b * b - 4. * a * c;

        let mut is_hit = d >= 0.0;
        let mut result_t = 0.;

        if is_hit {
            let sqr = d.sqrt();
            let t1 = (-b + sqr) / 2. * a;
            let t2 = (-b - sqr) / 2. * a;
            if t1 < 0. && t2 < 0. {
                is_hit = false;
            } else {
                result_t = f32_min(t1, t2);
            }
        }

        HitData {
            is_hit: is_hit,
            t: result_t,
            normal: Normal::empty()
        }
    }

    fn get_color(&self, hit_data: &HitData) -> RgbColor {
        RgbColor::copy(&self.color)
    }

    fn get_normal(&self, ray: &Ray, hit_data: &HitData) -> Normal {
        let pos = ray.pos.add(&ray.dir.scale(hit_data.t));
        let mut dir = pos.sub(&self.pos);
        dir = dir.norm();
        Normal {
            pos: pos,
            dir: dir
        }
    }
}

struct Ray
{
    pos: Vec3,
    dir: Vec3
}

impl Ray
{
    fn new(pos: Vec3, dir: Vec3) -> Ray {
        Ray {
            pos: pos,
            dir: dir
        }
    }

    fn empty() -> Ray {
        Ray {
            pos: Vec3::empty(),
            dir: Vec3::empty()
        }
    }
}

struct Camera
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
    fn new(pos: Vec3, dir: Vec3, field_of_view_degrees: f32, image_width: f32, image_height: f32) -> Camera {
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

    fn get_ray(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let mut x = pixel_x as f32;
        let mut y = pixel_y as f32;

        x = ((x + 0.5) * self.norm_width - 1.) * self.aspect * self.field_of_view;
        y = (1. - (y + 0.5) * self.norm_height) * self.field_of_view;

        Ray::new(Vec3::new(x, y, self.pos.z), Vec3::new(x, y, self.dir.z).norm())
    }
}

fn trace(x: u32, y: u32, geometry: &Vec<Box<Geometry>>, camera: &Camera, world_color: RgbColor) -> RgbColor
{
    let mut result_color = world_color;
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

    if hit_data.is_hit {
        hit_data.normal = hit_obj.get_normal(&ray, &hit_data);
        let angle = hit_data.normal.dir.dot(&ray.dir);
        result_color = hit_obj.get_color(&mut hit_data);

        result_color = result_color.sub(&result_color.scale(angle)).norm_color();
    }

    result_color
}

/*
    This function generates result image and save it in file 
*/
fn render(file_name: &String, image_width: u32, image_height: u32)
{
    let mut img: RgbImage = image::ImageBuffer::new(image_width, image_height);

    let sphere = Sphere::new(Vec3::new(0., 0., 0.), 0.5, RgbColor::new_color(255, 0, 0));
    let sphere_1 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.35, RgbColor::new_color(0, 255, 0));
    let sphere_2 = Sphere::new(Vec3::new(0.0, 0., 0.), 0.25, RgbColor::new_color(0, 0, 255));
    let sphere_3 = Sphere::new(Vec3::new(-1.3, 0., 0.), 0.1, RgbColor::new_color(0, 255, 255));
    let sphere_4 = Sphere::new(Vec3::new(1.3, 0., 0.), 0.1, RgbColor::new_color(0, 255, 255));
    let sphere_5 = Sphere::new(Vec3::new(0.0, 0.5, 0.), 0.1, RgbColor::new_color(0, 255, 255));
    let sphere_6 = Sphere::new(Vec3::new(0.0, -0.5, 0.), 0.1, RgbColor::new_color(0, 255, 255));
    let plane = Plane::new(Vec3::new(0., 0., -1.), Vec3::new(0., 0., 1.));

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

    for y in 0..image_height {
        for x in 0..image_width {
            let mut pixel = *img.get_pixel_mut(x, y);
            let color = trace(x, y, &geometry, &camera, RgbColor::new_color(0, 0, 0));

            pixel[0] = (color.x * 255.) as u8;
            pixel[1] = (color.y * 255.) as u8;
            pixel[2] = (color.z * 255.) as u8;
            img.put_pixel(x, y, pixel);
        }
    }

    img.save(file_name).unwrap();
}

fn main() {
    render(&String::from("./rendered/output.png"), 1280, 720);
}
