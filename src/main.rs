#![allow(dead_code, unused_variables)]

use std::{f32::INFINITY, rc::Rc};

use egui_macroquad::egui;
use macroquad::{prelude::*, rand::RandomRange};
mod raytracer;

use raytracer::{
    camera::Camera,
    geometry::reflect,
    hit_info::{HitInfo, Hittable},
    material::{LambertianMaterial, MaterialKind},
    ray::Ray,
    sphere::Sphere,
    world::World,
};

fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    let rr = a.r * (1. - t) + b.r * t;
    let gg = a.g * (1. - t) + b.g * t;
    let bb = a.b * (1. - t) + b.b * t;
    let aa = a.a * (1. - t) + b.a * t;
    Color::new(rr, gg, bb, aa)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "3D Engine".to_owned(),
        // fullscreen: true,
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

fn ray_color(ray: &raytracer::ray::Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color::new(1.0, 1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0, 1.0);

    color_lerp(white, blue, t)
}

struct NoIntersectionError;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> Result<f32, NoIntersectionError> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0. {
        Err(NoIntersectionError)
    } else {
        Ok((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn random() -> f32 {
    RandomRange::gen_range(0., 1.)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(0., 0., 0.);
    loop {
        p = 2. * Vec3::new(random(), random(), random()) - Vec3::new(1., 1., 1.);
        if p.length_squared() < 1. {
            break;
        }
    }
    p
}

fn get_ray_color(ray: &Ray, world: &World, depth: u8) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0., 1.);
    }

    if let Some(hit) = world.hit(ray, 0.001, INFINITY) {
        let (attenuation, ray, ok) = hit.material.scatter(ray, &hit);
        if ok {
            let c = get_ray_color(&ray, world, depth - 1);
            return Color::new(
                c.r * attenuation.r,
                c.g * attenuation.g,
                c.b * attenuation.b,
                1.,
            );
        } else {
            return Color::new(0., 0., 0., 1.);
        }
        // match hit.material {
        //     MaterialKind::Lambertian { albedo } => {
        //         let reflected = reflect(ray.direction.normalize(), hit.normal);
        //         let scattered = Ray::new(hit.p, reflected);
        //         let attenuation = albedo;
        //         let col = get_ray_color(&scattered, world, depth - 1);
        //         return Color::new(
        //             col.r * attenuation.r,
        //             col.g * attenuation.g,
        //             col.b * attenuation.b,
        //             1.,
        //         );
        //     }
        //     MaterialKind::Metal { albedo } => {
        //         let reflected = reflect(ray.direction.normalize(), hit.normal);
        //         let scattered = Ray::new(hit.p, reflected);
        //         let attenuation = albedo;
        //         let col = get_ray_color(&scattered, world, depth - 1);
        //         return Color::new(
        //             col.r * attenuation.r,
        //             col.g * attenuation.g,
        //             col.b * attenuation.b,
        //             1.,
        //         );
        //     }
        // }

        // let col = get_ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);
        // return Color::new(col.r * 0.5, col.g * 0.5, col.b * 0.5, 1.);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color::new(1.0, 1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0, 1.0);

    color_lerp(white, blue, t)
}

fn render_update(camera: &Camera, world: &World, screen_image: &mut Image) {
    for x in 0..1600 {
        for y in 0..900 {
            let u = x as f32 / 1599.;
            let v = y as f32 / 899.;
            let dir = camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
                - camera.origin;

            let ray = Ray::from_camera(camera, u, v);

            let hit = world.hit(&ray, 0.001, 1000.);

            if let Some(hit) = hit {
                // println!("hit: {:?}", hit);
                // let normal = hit.normal; //(hit_point - Point::new(0., 0., -1.)).normalize();
                // let c = Color::new(
                //     0.5 * (normal.x + 1.),
                //     0.5 * (normal.y + 1.),
                //     0.5 * (normal.z + 1.),
                //     1.,
                // );
                let mut c = Color::new(0., 0., 0., 1.);
                for i in 0..10 {
                    let u = (x as f32 + random()) / 1599.;
                    let v = (y as f32 + random()) / 899.;
                    let ray = Ray::from_camera(camera, u, v);
                    let col = get_ray_color(&ray, world, 50);
                    c.r += col.r;
                    c.g += col.g;
                    c.b += col.b;
                }
                c.r /= 10.;
                c.g /= 10.;
                c.b /= 10.;
                // let c = get_ray_color(&ray, world, 50);
                // screen_image.set_pixel(x, y, c);
                /* !TODO: ANTIALIASING
                let c = Color::default();

                for i in 0..3 {
                    let u = u + rand::gen_range(0., 1.) as f32 / 1599.;
                    let v = v + rand::gen_range(0., 1.) as f32 / 899.;
                    let ray = Ray::from_camera(&camera, u, v);
                    Color::new(
                        0.5 * (normal.x + 1.),
                        0.5 * (normal.y + 1.),
                        0.5 * (normal.z + 1.),
                        1.,
                    );
                } */
                screen_image.set_pixel(x, y, c);
            } else {
                let c = ray_color(&ray);
                screen_image.set_pixel(x, y, c);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera::default();

    let mut screen_image = Image::gen_image_color(1600, 900, RED);
    let screen_texture = Texture2D::from_image(&screen_image);

    // let material1 = MaterialKind::Lambertian {
    //     albedo: Color::new(0.8, 0.8, 0.0, 1.0),
    // };

    // let sphere1 = Sphere {
    //     center: Vec3::new(0.5, 0., -1.),
    //     radius: 0.25,
    //     material: material1,
    // };
    let sphere2 = Sphere {
        center: Vec3::new(0.0, 100.5, -1.),
        radius: 100.,
        material: MaterialKind::Lambertian {
            albedo: Color::new(0., 0., 1.0, 1.0),
        },
    };

    let sphere3 = Sphere {
        center: Vec3::new(-0.5, 0., -1.),
        radius: 0.25,
        material: MaterialKind::Metal {
            albedo: Color::new(0.8, 0.8, 0.8, 1.0),
        },
    };

    let sphere4 = Sphere {
        center: Vec3::new(0.0, 0., -1.),
        radius: -0.25,
        material: MaterialKind::Lambertian {
            albedo: Color::new(0.7, 0.3, 0.3, 1.0),
        },
    };
    let sphere5 = Sphere {
        center: Vec3::new(0.5, 0., -1.),
        radius: 0.25,
        material: MaterialKind::Metal {
            albedo: Color::new(0.8, 0.6, 0.2, 1.0),
        },
    };

    let mut world = World::new();
    // world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));
    world.add(Box::new(sphere4));
    world.add(Box::new(sphere5));

    // for x in 0..1600 {
    //     for y in 0..900 {
    //         let u = x as f32 / 1599.;
    //         let v = y as f32 / 899.;
    //         let dir = camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
    //             - camera.origin;

    //         let ray = Ray::from_camera(&camera, u, v);

    //         let mut hit = world.hit(&ray, 0.1, 1000.);

    //         if let Some(hit) = hit {
    //             // println!("hit: {:?}", hit);
    //             let normal = hit.normal; //(hit_point - Point::new(0., 0., -1.)).normalize();
    //             let c = Color::new(
    //                 0.5 * (normal.x + 1.),
    //                 0.5 * (normal.y + 1.),
    //                 0.5 * (normal.z + 1.),
    //                 1.,
    //             );

    //             /* !TODO: ANTIALIASING
    //             let c = Color::default();

    //             for i in 0..3 {
    //                 let u = u + rand::gen_range(0., 1.) as f32 / 1599.;
    //                 let v = v + rand::gen_range(0., 1.) as f32 / 899.;
    //                 let ray = Ray::from_camera(&camera, u, v);
    //                 Color::new(
    //                     0.5 * (normal.x + 1.),
    //                     0.5 * (normal.y + 1.),
    //                     0.5 * (normal.z + 1.),
    //                     1.,
    //                 );
    //             } */
    //             screen_image.set_pixel(x, y, c);
    //         } else {
    //             let c = ray_color(&ray);
    //             screen_image.set_pixel(x, y, c);
    //         }
    //     }
    // }

    render_update(&camera, &world, &mut screen_image);

    loop {
        clear_background(LIGHTGRAY);

        screen_texture.update(&screen_image);
        draw_texture(screen_texture, 0., 0., WHITE);

        //draw fps
        let fps = get_fps().to_string();
        draw_text(&fps, 10., 10., 30., BLACK);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Camera");
                // camera origin
                ui.label(format!("origin: {:?}", camera.origin));
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        let mut needs_update = false;

        // check AWSD key for camera movement
        if is_key_down(KeyCode::W) {
            camera.origin.y -= 0.1;
            needs_update = true;
        }

        if is_key_down(KeyCode::S) {
            camera.origin.y += 0.1;
            needs_update = true;
        }

        if is_key_down(KeyCode::A) {
            camera.origin.x -= 0.1;
            needs_update = true;
        }

        if is_key_down(KeyCode::D) {
            camera.origin.x += 0.1;
            needs_update = true;
        }

        // zoom

        if is_key_down(KeyCode::Q) {
            let f = camera.focal_length();
            camera.set_focal_length(f + 0.1);
            needs_update = true;
        }

        if is_key_down(KeyCode::E) {
            let f = camera.focal_length();
            camera.set_focal_length(f - 0.1);
            needs_update = true;
        }

        if needs_update {
            render_update(&camera, &world, &mut screen_image);
            screen_texture.update(&screen_image);
            draw_texture(screen_texture, 0., 0., WHITE);
        }

        next_frame().await
    }
}
