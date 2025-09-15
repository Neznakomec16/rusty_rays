use crate::{
    hittable::{HitRecord, Hittable},
    inretval::Interval,
    ray::Ray,
    vec3::{Point3, dot},
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
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
        let mut rec = HitRecord::new(t, p, normal);
        rec.set_face_normal(r);
        Some(rec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn test_sphere_hit_basic() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());

        let hit_record = hit.unwrap();
        assert!((hit_record.t - 4.0).abs() < 1e-10);
        assert!((hit_record.p.z + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_sphere_hit_miss() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 2.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_none());
    }

    #[test]
    fn test_sphere_hit_inside() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        let hit = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());

        let hit_record = hit.unwrap();
        assert!((hit_record.t - 2.0).abs() < 1e-10);
        assert!((hit_record.p.x - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_sphere_hit_ray_bounds() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        // Hit is at t=4, but we limit tmax to 3
        let hit = sphere.hit(&ray, Interval::new(0.001, 3.0));
        assert!(hit.is_none());

        // Hit is at t=4, tmax allows it
        let hit = sphere.hit(&ray, Interval::new(0.001, 5.0));
        assert!(hit.is_some());
    }

    #[test]
    fn test_sphere_hit_normal_direction() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(2.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));

        let hit = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());

        let hit_record = hit.unwrap();
        // Normal should point outward from sphere center
        assert!((hit_record.normal.x - 1.0).abs() < 1e-10);
        assert!(hit_record.normal.y.abs() < 1e-10);
        assert!(hit_record.normal.z.abs() < 1e-10);
    }

    #[test]
    fn test_sphere_hit_tangent() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 1.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.hit(&ray, Interval::new(0.001, f64::INFINITY));
        assert!(hit.is_some());

        let hit_record = hit.unwrap();
        assert!((hit_record.p.y - 1.0).abs() < 1e-10);
        assert!(hit_record.p.z.abs() < 1e-10);
    }
}
