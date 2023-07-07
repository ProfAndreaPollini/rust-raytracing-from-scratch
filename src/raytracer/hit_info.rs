use std::rc::Rc;

use macroquad::prelude::{Color, Vec3};

use super::{
    material::{Material, MaterialKind},
    ray::Ray,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>;
}

#[derive(Debug)]
pub struct HitInfo {
    pub hit: bool,
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: MaterialKind,
}

// trait AA {}
// struct BB;
// impl AA for BB {}
// struct CC<'a> {
//     aa: Box<&'a dyn AA>,
// }
// impl<'a> CC<'a> {
//     fn new(aa: Box<&'a dyn AA>) -> Self {
//         Self { aa }
//     }
// }

impl HitInfo {
    pub fn new(hit: bool, t: f32, p: Vec3, normal: Vec3, material: MaterialKind) -> Self {
        Self {
            hit,
            t,
            p,
            normal,
            front_face: false,
            material,
        }
    }

    pub fn set_face_normal(&mut self, direction: Vec3, outward_normal: Vec3) {
        self.front_face = direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
