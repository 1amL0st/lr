pub mod vec3 {
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

    #[derive(Debug)]
    pub struct Vec3
    {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }

    impl Vec3 {
        pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
            Vec3 {x, y, z}
        }

        pub fn empty() -> Vec3 {
            Vec3 {x: 0., y: 0., z: 0.}
        }

        pub fn copy(v: &Vec3) -> Vec3 {
            Vec3 {x: v.x, y: v.y, z: v.z}
        }

        pub fn add(&self, v: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x + v.x,
                y: self.y + v.y,
                z: self.z + v.z
            }
        }

        pub fn sub(&self, v: &Vec3) -> Vec3 {
            Vec3 {
                x: self.x - v.x,
                y: self.y - v.y,
                z: self.z - v.z }
        }

        pub fn len(&self) -> f32 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

        pub fn norm(&self) -> Vec3 {
            let len = self.len();
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len
            }
        }

        pub fn dot(&self, v: &Vec3) -> f32 {
            self.x * v.x + self.y * v.y + self.z * v.z
        }

        pub fn scale(&self, s: f32) -> Vec3 {
            Vec3::new(self.x * s, self.y * s, self.z * s)
        }

        pub fn negate(&self) -> Vec3 {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z
            }
        }

        pub fn norm_color(&self) -> Vec3 {
            Vec3 {
                x: f32_max(self.x, 0.),
                y: f32_max(self.y, 0.),
                z: f32_max(self.z, 0.)
            }
        }
    
        pub fn new_color(r: u8, g: u8, b: u8) -> Vec3 {
            let factor = 1. / 255.;
            Vec3 {
                x: (r as f32) * factor,
                y: (g as f32) * factor,
                z: (b as f32) * factor
            }
        }

        pub fn print(v: &Vec3) {
            println!{"{:?}", v};
        }
    }
}