use glam::{Affine3A, Mat3, Mat4, Quat, Vec3};
use serde::Deserialize;

pub trait Transformable {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
    fn translate(&mut self, translation: Vec3) {
        self.transform_mut().position += translation;
    }
    fn move_forward(&mut self, delta: f32) {
        self.translate(self.transform().forward() * delta)
    }
    fn move_backward(&mut self, delta: f32) {
        self.move_forward(-delta);
    }
    fn move_right(&mut self, delta: f32) {
        self.translate(self.transform().right() * delta);
    }
    fn move_left(&mut self, delta: f32) {
        self.move_right(-delta);
    }
    fn move_up(&mut self, delta: f32) {
        self.translate(self.transform().up() * delta);
    }
    fn move_down(&mut self, delta: f32) {
        self.move_up(-delta);
    }
    fn rotate(&mut self, rotation: Quat) {
        self.transform_mut().orientation.rotate(rotation);
    }
    fn yaw(&mut self, angle: f32) {
        self.transform_mut().orientation.yaw(angle);
    }
    fn pitch(&mut self, angle: f32) {
        self.transform_mut().orientation.pitch(angle);
    }
    fn roll(&mut self, angle: f32) {
        self.transform_mut().orientation.roll(angle);
    }
    fn yaw_deg(&mut self, angle: f32) {
        self.transform_mut().orientation.yaw_deg(angle);
    }
    fn pitch_deg(&mut self, angle: f32) {
        self.transform_mut().orientation.pitch_deg(angle);
    }
    fn roll_deg(&mut self, angle: f32) {
        self.transform_mut().orientation.roll_deg(angle);
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(from = "Mat3")]
pub struct Orientation {
    forward: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Orientation {
    pub fn new(forward: Vec3, up: Vec3) -> Self {
        let forward_unit = forward.normalize();
        let up_unit = up.normalize();
        let right = forward_unit.cross(up_unit).normalize();
        Self {
            forward: forward_unit,
            right,
            up: right.cross(forward_unit).normalize(),
        }
    }
    pub fn rotate(&mut self, rotation: Quat) {
        self.forward = rotation.mul_vec3(self.forward).normalize();
        self.right = rotation.mul_vec3(self.right).normalize();
        self.up = rotation.mul_vec3(self.up).normalize();
    }
    pub fn yaw(&mut self, angle: f32) {
        self.rotate(Quat::from_axis_angle(self.up, angle));
    }
    pub fn pitch(&mut self, angle: f32) {
        self.rotate(Quat::from_axis_angle(self.right, angle));
    }
    pub fn roll(&mut self, angle: f32) {
        self.rotate(Quat::from_axis_angle(self.forward, angle));
    }
    pub fn yaw_deg(&mut self, angle: f32) {
        self.yaw(angle.to_radians());
    }
    pub fn pitch_deg(&mut self, angle: f32) {
        self.pitch(angle.to_radians());
    }
    pub fn roll_deg(&mut self, angle: f32) {
        self.roll(angle.to_radians());
    }
    pub fn as_mat3(&self) -> Mat3 {
        Mat3::from_cols(self.right, self.up, -self.forward)
    }
    pub fn as_affine3a(&self) -> Affine3A {
        Affine3A::from_mat3(self.as_mat3())
    }
    pub fn as_quat(&self) -> Quat {
        Quat::from_mat3(&self.as_mat3())
    }
    pub fn forward(&self) -> Vec3 {
        self.forward
    }
    pub fn right(&self) -> Vec3 {
        self.right
    }
    pub fn up(&self) -> Vec3 {
        self.up
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::from(Mat3::IDENTITY)
    }
}

impl From<Mat3> for Orientation {
    fn from(m: Mat3) -> Self {
        Self::new(-m.z_axis, m.y_axis)
    }
}

impl From<Quat> for Orientation {
    fn from(q: Quat) -> Self {
        Self::from(Mat3::from_quat(q))
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(from = "Mat4")]
pub struct Transform {
    position: Vec3,
    orientation: Orientation,
    scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, orientation: Orientation, scale: Vec3) -> Self {
        Self {
            position,
            orientation,
            scale,
        }
    }
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            position: translation,
            ..Default::default()
        }
    }
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            orientation: Orientation::from(rotation),
            ..Default::default()
        }
    }
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        Self {
            position: translation,
            orientation: Orientation::from(rotation),
            ..Default::default()
        }
    }
    pub fn from_scale_translation(scale: Vec3, translation: Vec3) -> Self {
        Self {
            position: translation,
            scale,
            ..Default::default()
        }
    }
    pub fn from_scale_rotation(scale: Vec3, rotation: Quat) -> Self {
        Self {
            orientation: Orientation::from(rotation),
            scale,
            ..Default::default()
        }
    }
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            position: translation,
            orientation: Orientation::from(rotation),
            scale,
        }
    }
    pub fn from_look_at(position: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = target - position;
        Self {
            position,
            orientation: Orientation::new(forward, up),
            scale: Vec3::ONE,
        }
    }

    pub fn as_mat4_with_child(&self, other: &Self) -> Mat4 {
        self.as_mat4().mul_mat4(&other.as_mat4())
    }
    pub fn as_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.orientation.as_quat(), self.position)
    }
    pub fn forward(&self) -> Vec3 {
        self.orientation.forward()
    }
    pub fn right(&self) -> Vec3 {
        self.orientation.right()
    }
    pub fn up(&self) -> Vec3 {
        self.orientation.up()
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }
    pub fn scale(&self) -> Vec3 {
        self.scale
    }
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::from(Mat4::IDENTITY)
    }
}

impl From<Mat4> for Transform {
    fn from(m: Mat4) -> Self {
        let (scale, rotation, translation) = m.to_scale_rotation_translation();
        Self::from_scale_rotation_translation(scale, rotation, translation)
    }
}

impl Transformable for Transform {
    fn transform(&self) -> &Transform {
        self
    }
    fn transform_mut(&mut self) -> &mut Transform {
        self
    }
}

pub mod util {
    use crate::framework::scene::transform::Transformable;
    use crate::framework::util::math::f32::is_close_to_zero;
    use glam::{Mat3, Vec2, Vec3};

    pub trait Orbit: Transformable {
        fn target(&self) -> Vec3;
        fn set_target(&mut self, target: Vec3);
        fn orbit(&mut self, delta: Vec2, invert: bool) {
            if !(is_close_to_zero(delta.x) && is_close_to_zero(delta.y)) {
                let delta_scaled = delta * (std::f32::consts::TAU);

                // choose origin to orbit around
                let origin = if invert {
                    self.transform().position()
                } else {
                    self.target()
                };

                // choose point that is being orbited
                let position = if invert {
                    self.target()
                } else {
                    self.transform().position()
                };

                let center_to_eye = position - origin;
                let radius = center_to_eye.length();

                let z = center_to_eye.normalize();
                let y = self.transform().up();
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
                    self.set_target(new_position);
                } else {
                    self.transform_mut().set_position(new_position);
                }
            }
        }
    }
}
