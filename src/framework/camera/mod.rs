use glam::{Mat3, Mat4, Vec2, Vec3};
use crate::framework::geometry::bounds::{Bounds, Bounds2, Bounds3};
use crate::framework::util::math::is_close_to_zero;
use crate::framework::util::window::Resize;

pub trait Camera {
    fn view(&self) -> Mat4;
    fn inverse_view(&self) -> Mat4 {
        self.view().inverse()
    }
    fn projection(&self) -> Mat4;
    fn inverse_projection(&self) -> Mat4 {
        self.projection().inverse()
    }
}

// todo: refactor - this is just basic transform stuff
#[derive(Copy, Clone, Debug)]
pub struct CameraView {
    position: Vec3,
    center_of_projection: Vec3,
    up: Vec3,
}

impl CameraView {
    pub fn new(position: Vec3, center_of_projection: Vec3, up: Vec3) -> Self {
        Self {
            position,
            center_of_projection,
            up: up.normalize(),
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn center_of_projection(&self) -> Vec3 {
        self.center_of_projection
    }

    pub fn set_center_of_projection(&mut self, center_of_projection: Vec3) {
        self.center_of_projection = center_of_projection;
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn set_up(&mut self, up: Vec3) {
        self.up = up.normalize();
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.center_of_projection, self.up)
    }

    pub fn forward(&self) -> Vec3 {
        (self.center_of_projection - self.position).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up)
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
        self.center_of_projection += translation;
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.translate(self.forward() * delta);
    }

    pub fn move_backward(&mut self, delta: f32) {
        self.move_forward(-delta);
    }

    pub fn move_right(&mut self, delta: f32) {
        self.translate(self.right() * delta);
    }

    pub fn move_left(&mut self, delta: f32) {
        self.move_right(-delta);
    }

    pub fn move_up(&mut self, delta: f32) {
        self.translate(self.up * delta);
    }

    pub fn move_down(&mut self, delta: f32) {
        self.move_up(-delta);
    }

    pub fn zoom_in(&mut self, delta: f32) {
        let distance = self.position.distance(self.center_of_projection);
        let movement = self.forward() *
            if distance <= delta {
                distance - f32::EPSILON
            } else {
                delta
            };
        self.position += movement;
    }

    pub fn zoom_out(&mut self, delta: f32) {
        self.zoom_in(-delta);
    }

    // todo: refactor into extra struct / trait
    pub fn orbit(&mut self, delta: Vec2, invert: bool) {
        if !(is_close_to_zero(delta.x) && is_close_to_zero(delta.y)) {
            let delta_scaled = delta * (std::f32::consts::PI * 2.);

            // choose origin to orbit around
            let origin = if invert {
                self.position
            } else {
                self.center_of_projection
            };

            // choose point that is being orbited
            let position = if invert {
                self.center_of_projection
            } else {
                self.position
            };

            let center_to_eye = position - origin;
            let radius = center_to_eye.length();

            let z = center_to_eye.normalize();
            let y = self.up;
            let x = y.cross(z).normalize();

            let y_rotation = Mat3::from_axis_angle(y, -delta_scaled.x);
            let x_rotation = Mat3::from_axis_angle(x, -delta_scaled.y);

            let rotated_y = y_rotation.mul_vec3(z);
            let rotated_x = x_rotation.mul_vec3(rotated_y);

            let new_position = origin
                + (if rotated_x.x.signum() == rotated_y.x.signum() {
                rotated_x
            } else {
                rotated_y
            } * radius);
            if invert {
                self.center_of_projection = new_position;
            } else {
                self.position = new_position;
            }
        }
    }
}

impl Default for CameraView {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OrthographicProjection {
    projection: Mat4,
    frustum: Bounds3,
}

impl OrthographicProjection {
    pub fn new(frustum: Bounds3) -> Self {
        let projection = Mat4::orthographic_rh(
            frustum.min().x,
            frustum.max().x,
            frustum.min().y,
            frustum.max().y,
            frustum.min().z,
            frustum.max().z,
        );
        Self { projection, frustum }
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }
    pub fn frustum(&self) -> &Bounds3 {
        &self.frustum
    }
}

impl Resize for OrthographicProjection {
    fn resize(&mut self, width: u32, height: u32) {
        let width_half = (width / 2) as f32;
        let height_half = (height / 2) as f32;
        let xy_bounds = Bounds2::new(
            Vec2::new(-width_half, -height_half),
            Vec2::new(width_half, height_half),
        );
        self.frustum.adjust_xy(xy_bounds);
        self.projection = Mat4::orthographic_rh(
            self.frustum.min().x,
            self.frustum.max().x,
            self.frustum.min().y,
            self.frustum.max().y,
            self.frustum.min().z,
            self.frustum.max().z,
        );
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PerspectiveProjection {
    projection: Mat4,
    fov_y: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
}

impl PerspectiveProjection {
    pub fn new(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let projection = Mat4::perspective_rh(fov_y, aspect_ratio, z_near, z_far);
        Self { projection, fov_y, aspect_ratio, z_near, z_far }
    }
    fn update_projection(&mut self) {
        self.projection = Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.z_near, self.z_far);
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }
    pub fn fov_y(&self) -> f32 {
        self.fov_y
    }
    pub fn fov_y_degrees(&self) -> f32 {
        self.fov_y.to_degrees()
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    pub fn z_near(&self) -> f32 {
        self.z_near
    }
    pub fn z_far(&self) -> f32 {
        self.z_far
    }
    pub fn set_fov_y(&mut self, fov_y: f32) {
        self.fov_y = fov_y;
        self.update_projection();
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.update_projection();
    }
    pub fn set_z_near(&mut self, z_near: f32) {
        self.z_near = z_near;
        self.update_projection();
    }
    pub fn set_z_far(&mut self, z_far: f32) {
        self.z_far = z_far;
        self.update_projection();
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Projection {
    Orthographic(OrthographicProjection),
    Perspective(PerspectiveProjection),
}

impl Projection {
    pub fn new_orthographic(frustum: Bounds3) -> Self {
        Self::Orthographic(OrthographicProjection::new(frustum))
    }
    pub fn new_perspective(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self::Perspective(PerspectiveProjection::new(fov_y, aspect_ratio, z_near, z_far))
    }
    pub fn projection(&self) -> Mat4 {
        match self {
            Self::Orthographic(o) => o.projection(),
            Self::Perspective(p) => p.projection()
        }
    }
}

impl Resize for Projection {
    fn resize(&mut self, width: u32, height: u32) {
        match self {
            Self::Orthographic(o) => o.resize(width, height),
            Self::Perspective(p) => p.set_aspect_ratio(width as f32 / height as f32)
        }
    }
}
