use crate::{
    hittable::{HitRecord, Hittable}, inretval::Interval, material::Material, ray::Ray, vec3::{Vec3, Point3}
};

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub mat: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, mat: impl Material + 'a) -> Self {
        Self { center, radius, mat: Box::new(mat) }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let h = Vec3::dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let mut rec = HitRecord::new(t, p, normal, &*self.mat);
        rec.set_face_normal(r);
        Some(rec)
    }
}
