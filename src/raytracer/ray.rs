use macroquad::prelude::Vec3;

#[allow(dead_code, unused_variables)]

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn from_camera(camera: &super::camera::Camera, u: f32, v: f32) -> Self {
        Ray::new(
            camera.origin,
            camera.lower_left_corner + camera.horizontal * u + camera.vertical * v - camera.origin,
        )
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use macroquad::prelude::Vec3;

    use crate::raytracer::ray::Ray;

    #[test]
    fn test_ray_creation() {
        let origin = Vec3::new(1., 2., 3.);
        let direction = Vec3::new(4., 5., 6.);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin.x, 1.);
        assert_eq!(ray.origin.y, 2.);
        assert_eq!(ray.origin.z, 3.);

        assert_eq!(ray.direction.x, 4.);
        assert_eq!(ray.direction.y, 5.);
        assert_eq!(ray.direction.z, 6.);
    }

    #[test]
    fn test_ray_values() {
        let origin = Vec3::new(2., 3., 4.);
        let direction = Vec3::new(1., 0., 0.);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.at(0.), Vec3::new(2., 3., 4.));
        assert_eq!(ray.at(1.), Vec3::new(3., 3., 4.));
        assert_eq!(ray.at(-1.), Vec3::new(1., 3., 4.));
        assert_eq!(ray.at(2.5), Vec3::new(4.5, 3., 4.));
    }
}
