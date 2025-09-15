use crate::color::write_color;
use crate::util::random_f64;
use crate::vec3::{Point3, Vec3};
use crate::{color::Color, hittable::Hittable, inretval::Interval, ray::Ray};

pub struct Camera {
    pub aspect_ratio: f64,  // = 1.0
    pub image_width: usize, // = 100
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    image_height: usize,
    center: Point3,
    pixel100_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            // Private
            image_height: 0,
            center: Vec3::zero(),
            pixel100_loc: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 0.0,
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();
        let out = std::io::stdout();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaning: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&out, self.pixel_samples_scale * pixel_color);
            }
        }
        eprintln!("\rDone                           ");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewpoint_height = 2.0;
        let viewport_width =
            viewpoint_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewpoint_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel100_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel100_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn ray_color(&self, r: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth-1, world);
        }
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
