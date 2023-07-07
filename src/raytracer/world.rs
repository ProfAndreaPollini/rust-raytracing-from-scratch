#![allow(dead_code, unused_variables)]

use macroquad::prelude::Color;

use super::{
    hit_info::{HitInfo, Hittable},
    material::MaterialKind,
    ray::Ray,
};

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let mut closest_so_far = t_max;
        let material = MaterialKind::Lambertian {
            albedo: Color::new(0., 0., 0., 1.),
        };
        let mut hit_info =
            HitInfo::new(false, 0., Default::default(), Default::default(), material);

        for object in &self.objects {
            if let Some(info) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = info.t;
                hit_info = info;
            }
        }

        if hit_info.hit {
            return Some(hit_info);
        }

        None
    }
}
