use glam::{Affine3A, Mat3, Mat4, Quat, Vec3};

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
        Mat3::from_cols(
            self.right,
            self.up,
            -self.forward
        )
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
        Self::new(-Vec3::Z, Vec3::Y)
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

pub struct Transform {
    position: Vec3,
    orientation: Orientation,
    scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, orientation: Orientation, scale: Vec3) -> Self {
        Self { position, orientation, scale }
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
        let forward = (target - position);
        Self {
            position,
            orientation: Orientation::new(forward, up),
            scale: Vec3::ONE,
        }
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }
    pub fn move_forward(&mut self, delta: f32) {
        self.translate(self.orientation.forward() * delta)
    }
    pub fn move_backward(&mut self, delta: f32) {
        self.move_forward(-delta);
    }
    pub fn move_right(&mut self, delta: f32) {
        self.translate(self.orientation.right() * delta);
    }
    pub fn move_left(&mut self, delta: f32) {
        self.move_right(-delta);
    }
    pub fn move_up(&mut self, delta: f32) {
        self.translate(self.orientation.up() * delta);
    }
    pub fn move_down(&mut self, delta: f32) {
        self.move_up(-delta);
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.orientation.rotate(rotation);
    }
    pub fn yaw(&mut self, angle: f32) {
        self.orientation.yaw(angle);
    }
    pub fn pitch(&mut self, angle: f32) {
        self.orientation.pitch(angle);
    }
    pub fn roll(&mut self, angle: f32) {
        self.orientation.roll(angle);
    }
    pub fn yaw_deg(&mut self, angle: f32) {
        self.orientation.yaw_deg(angle);
    }
    pub fn pitch_deg(&mut self, angle: f32) {
        self.orientation.pitch_deg(angle);
    }
    pub fn roll_deg(&mut self, angle: f32) {
        self.orientation.roll_deg(angle);
    }
    pub fn as_mat4_with_child(&self, other: &Self) -> Mat4 {
        self.as_mat4().mul_mat4(&other.as_mat4())
    }
    pub fn as_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.scale,
            self.orientation.as_quat(),
            self.position
        )
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
        Self {
            position: Vec3::ZERO,
            orientation: Orientation::default(),
            scale: Vec3::ONE,
        }
    }
}
