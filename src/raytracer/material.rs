use std::fmt::Debug;

use macroquad::prelude::{Color, Vec3};

use super::{
    geometry::{near_zero, random_in_hemisphere, random_unit_vector, reflect},
    hit_info::HitInfo,
    ray::Ray,
};

pub trait Material: Debug {
    fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> (Color, Ray);
}

#[derive(Debug)]
pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, hit_info: &HitInfo) -> (Color, Ray) {
        let mut scatter_direction = hit_info.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = hit_info.normal;
        }

        let scattered = Ray::new(hit_info.p, scatter_direction);

        (self.albedo, scattered)
    }
}

// #[derive(Debug)]
// struct MetalMaterial {
//     albedo: Color,
// }

// impl Material for MetalMaterial {
//     fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> Option<(Color, Ray)> {
//         let reflected = ray_in.direction.normalize().reflect(hit_info.normal);
//         let scattered = Ray::new(hit_info.p, reflected);

//         if scattered.direction.dot(hit_info.normal) > 0. {
//             return Some((self.albedo, scattered));
//         }

//         None
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum MaterialKind {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
    // Metal(MetalMaterial),
}

impl MaterialKind {
    pub fn scatter(&self, ray_in: &Ray, hit_info: &HitInfo) -> (Color, Ray, bool) {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = hit_info.normal + random_in_hemisphere(hit_info.normal);

                if near_zero(scatter_direction) {
                    scatter_direction = hit_info.normal;
                }

                let scattered = Ray::new(hit_info.p, scatter_direction);

                (*albedo, scattered, true)
            } // Self::Metal(material) => material.scatter(ray_in, hit_info),
            Self::Metal { albedo } => {
                let reflected = reflect(ray_in.direction.normalize(), hit_info.normal);
                let scattered = Ray::new(hit_info.p, reflected);
                let ok = scattered.direction.dot(hit_info.normal) > 0.;
                (*albedo, scattered, ok)
            }
        }
    }
}
