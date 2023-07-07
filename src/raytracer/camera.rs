use macroquad::prelude::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    focal_length: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(Vec3::default(), 16. / 9., 1.)
    }
}

impl Camera {
    pub fn new(origin: Vec3, aspect_ratio: f32, focal_length: f32) -> Self {
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = focal_length;

        let origin = origin;
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            focal_length,
        }
    }

    pub fn set_focal_length(&mut self, focal_length: f32) {
        self.focal_length = focal_length;
    }

    pub fn focal_length_mut(&mut self) -> &mut f32 {
        &mut self.focal_length
    }

    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }
}
