use macroquad::prelude::Vec3;

use super::{
    hit_info::Hittable,
    material::{Material, MaterialKind},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: MaterialKind,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialKind) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &super::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<super::hit_info::HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2. * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return None;
        }

        let t = (-b - discriminant.sqrt()) / (2. * a);
        if t < t_max && t > t_min {
            let p = ray.at(t);

            let normal = (p - self.center) / self.radius;

            let mut hit_info = super::hit_info::HitInfo::new(true, t, p, normal, self.material);
            hit_info.set_face_normal(ray.direction, normal);
            // normal.y = -normal.y;
            return Some(hit_info);
        }

        let t = (-b + discriminant.sqrt()) / (2. * a);
        if t < t_max && t > t_min {
            let p = ray.at(t);
            let normal = (p - self.center) / self.radius;

            let mut hit_info = super::hit_info::HitInfo::new(true, t, p, normal, self.material);
            hit_info.set_face_normal(ray.direction, normal);

            return Some(hit_info);
        }

        None
    }
}
