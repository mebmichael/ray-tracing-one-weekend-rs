use vector_math::random_methods::random_in_unit_disk;
use vector_math::ray::Ray;
use vector_math::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    up: Vec3,
    right: Vec3,
}

impl Camera {
    /// `fov` is vertical field-of-view.
    pub fn new(origin: Vec3, look_at: Vec3, up: Vec3, fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let forward = (origin - look_at).normalized();
        let right = up.cross(forward);
        let up_orthonormal = forward.cross(right);

        Camera {
            origin,
            lower_left_corner: origin - (half_width * right + half_height * up_orthonormal + forward) * focus_dist,
            horizontal: 2.0 * half_width * focus_dist * right,
            vertical: 2.0 * half_height * focus_dist * up_orthonormal,
            lens_radius: aperture / 2.0,
            right,
            up: up_orthonormal,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.right * rd.x + self.up * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
