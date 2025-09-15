mod camera;
mod color;
mod hittable;
mod hittable_list;
mod inretval;
mod ray;
mod sphere;
mod vec3;
mod util;

use vec3::Point3;

use crate::{
    camera::Camera, hittable_list::HittableList,
    sphere::Sphere,
};

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.render(&world);
}
