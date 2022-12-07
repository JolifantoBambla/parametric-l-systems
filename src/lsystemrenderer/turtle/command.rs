use std::collections::VecDeque;
use glam::{Mat3, Mat4, Vec3};
use serde::{Deserialize};
use crate::framework::geometry::bounds::{Bounds, Bounds3};

// todo: move Orientation & CoordinateFrame to framework

#[derive(Copy, Clone, Debug)]
pub struct Orientation {
    forward: Vec3,
    right: Vec3,
    up: Vec3,
}

impl Orientation {
    pub fn yaw(&mut self, angle: f32) {
        let rotation = Mat3::from_axis_angle(self.up, angle);
        self.forward = rotation.mul_vec3(self.forward);
        self.right = rotation.mul_vec3(self.right);
    }

    pub fn pitch(&mut self, angle: f32) {
        let rotation = Mat3::from_axis_angle(self.right, angle);
        self.forward = rotation.mul_vec3(self.forward);
        self.up = rotation.mul_vec3(self.up);
    }

    pub fn roll(&mut self, angle: f32) {
        let rotation = Mat3::from_axis_angle(self.forward, angle);
        self.right = rotation.mul_vec3(self.right);
        self.up = rotation.mul_vec3(self.up);
    }

    pub fn yaw_degree(&mut self, angle: f32) {
        self.yaw(angle.to_radians());
    }

    pub fn pitch_degree(&mut self, angle: f32) {
        self.pitch(angle.to_radians());
    }

    pub fn roll_degree(&mut self, angle: f32) {
        self.roll(angle.to_radians());
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
    pub fn as_mat3(&self) -> Mat3 {
        //Mat3::from_cols(self.right, self.up, -self.forward)
        Mat3::from_cols(self.up, self.forward, -self.right)
    }
    pub fn as_mat4(&self) -> Mat4 {
        Mat4::look_at_rh(Vec3::ZERO, self.forward, self.up)
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            forward: -Vec3::Z,
            right: Vec3::X,
            up: Vec3::Y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CoordinateFrame {
    origin: Vec3,
    min_distance_to_target: f32,
    distance_to_target: f32,
    orientation: Orientation,
    original_orientation: Orientation,
}

impl CoordinateFrame {
    pub fn translate(&mut self, translation: Vec3) {
        self.origin += translation;
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.translate(self.orientation.forward() * delta);
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

    pub fn zoom_in(&mut self, delta: f32) {
        self.distance_to_target = (self.distance_to_target - delta).max(self.min_distance_to_target);
    }

    pub fn zoom_out(&mut self, delta: f32) {
        self.zoom_in(-delta);
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

    pub fn yaw_degree(&mut self, angle: f32) {
        self.orientation.yaw_degree(angle);
    }

    pub fn pitch_degree(&mut self, angle: f32) {
        self.orientation.pitch_degree(angle);
    }

    pub fn roll_degree(&mut self, angle: f32) {
        self.orientation.roll_degree(angle);
    }

    pub fn reset_orientation(&mut self) {
        self.orientation = self.original_orientation;
    }

    pub fn as_mat4(&self) -> Mat4 {
        //Mat4::from_translation(self.origin)
            //.mul_mat4(&self.orientation.as_mat4())
            //.mul_mat4(&Mat4::from_mat3(self.orientation.as_mat3().inverse()))
        //self.orientation.as_mat4()
            //.mul_mat4(&Mat4::from_translation(self.origin))
        //Mat4::look_at_rh(self.origin, self.target(), self.orientation.up)

        /*
        let rotation = Mat4::from_mat3(
            Mat3::from_cols(
                self.orientation.right(),
                self.orientation.up(),
                -self.orientation.forward()
            ),
        );
         */

        let rotation = Mat4::look_at_rh(Vec3::ZERO, self.orientation.up(), self.orientation.forward());

        let translation = Mat4::from_translation(self.origin);

        translation.mul_mat4(&rotation)
        //rotation.mul_mat4(&translation)
        //translation
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn target(&self) -> Vec3 {
        self.origin + self.orientation().forward() * self.distance_to_target
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}

impl Default for CoordinateFrame {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            min_distance_to_target: f32::EPSILON,
            distance_to_target: 1.0,
            orientation: Orientation::default(),
            original_orientation: Orientation::default(),
        }
    }
}

// F(d,r)
// Positioniere einen Zylinder der Länge d mit Radius r entlang der h-Achse.
// Translation entlang der h-Achse um die Länge d (an den Endpunkt des Zylinders).
#[derive(Debug, Deserialize)]
pub struct AddCylinder {
    parameters: [f32; 2],
}

impl AddCylinder {
    pub fn length(&self) -> f32 {
        self.parameters[0]
    }

    pub fn radius(&self) -> f32 {
        self.parameters[1]
    }
}

// f(d)
// Translation entlang der H-Achse um die Länge d ohne Konstruktion.
#[derive(Debug, Deserialize)]
pub struct Translate {
    parameters: [f32; 1],
}

impl Translate {
    pub fn length(&self) -> f32 {
        self.parameters[0]
    }
}

// +()
// Rotation um die u-Achse um den Winkel  (Yaw).
#[derive(Debug, Deserialize)]
pub struct RotateYaw {
    parameters: [f32; 1],
}

impl RotateYaw {
    pub fn angle(&self) -> f32 {
        self.parameters[0]
    }
}

// &()
// Rotation um die r-Achse um den Winkel  (Pitch).
#[derive(Debug, Deserialize)]
pub struct RotatePitch {
    parameters: [f32; 1],
}

impl RotatePitch {
    pub fn angle(&self) -> f32 {
        self.parameters[0]
    }
}

// /(d)
// Rotation um die h-Achse um den Winkel  (Roll).
#[derive(Debug, Deserialize)]
pub struct RotateRoll {
    parameters: [f32; 1],
}

impl RotateRoll {
    pub fn angle(&self) -> f32 {
        self.parameters[0]
    }
}

// |
// Drehung um 180° um die u-Achse (Shortcut für +(180))

// [
// Der aktuelle Zustand der Turtle wird auf einen Stack gelegt

// ]
// Der letzte Zustand wird vom Stack entfernt und die Turtle in diesen Zustand versetzt

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
pub enum TurtleCommand {
    #[serde(rename = "F")]
    AddCylinder(AddCylinder),

    #[serde(rename = "f")]
    Translate(Translate),

    #[serde(rename = "+")]
    RotateYaw(RotateYaw),

    #[serde(rename = "&")]
    RotatePitch(RotatePitch),

    #[serde(rename = "/")]
    RotateRoll(RotateRoll),

    #[serde(rename = "|")]
    Yaw180,

    #[serde(rename = "[")]
    PushToStack,

    #[serde(rename = "]")]
    PopFromStack,

    // every command below this line is not needed for the exercise

    #[serde(rename = "$")]
    ToHorizontal,

    #[serde(other)]
    Unknown,
}

pub fn test_commands() -> Vec<TurtleCommand> {
    vec![
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotateYaw(RotateYaw { parameters: [90.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotateYaw(RotateYaw { parameters: [45.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotatePitch(RotatePitch { parameters: [90.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotateRoll(RotateRoll { parameters: [90.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::ToHorizontal,
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotatePitch(RotatePitch { parameters: [90.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotatePitch(RotatePitch { parameters: [-90.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::RotateYaw(RotateYaw { parameters: [90.,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
        TurtleCommand::AddCylinder(AddCylinder { parameters: [0.5, 0.25,] }),
    ]
}
