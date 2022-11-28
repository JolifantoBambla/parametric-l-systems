use std::collections::VecDeque;
use glam::Mat4;
use serde::{Deserialize, Deserializer};

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
    pub fn yaw(&self) -> f32 {
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
    pub fn pitch(&self) -> f32 {
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
    pub fn roll(&self) -> f32 {
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

pub fn execute_turtle_commands(commands: &Vec<TurtleCommand>) -> Vec<Mat4> {
    let mut matrices = Vec::new();

    let mut state = Mat4::IDENTITY;
    let mut stack = VecDeque::new();
    for c in commands {
        match c {
            TurtleCommand::AddCylinder(cylinder) => {
                let mut instance = state.clone();
                // todo: scale instance by cylinders parameters
                matrices.push(instance);
                // todo: translate state by cylinder's length
            }
            TurtleCommand::Translate(t) => {
                // todo: translate state by t in current direction
            }
            TurtleCommand::RotateYaw(yaw) => {
                // todo: rotate state by yaw
            }
            TurtleCommand::RotatePitch(pitch) => {
                // todo: rotate state by pitch
            }
            TurtleCommand::RotateRoll(roll) => {
                // todo: rotate state by roll
            }
            TurtleCommand::Yaw180 => {
                // todo: rotate state by yaw = 180°
            }
            TurtleCommand::PushToStack => {
                stack.push_front(state.clone());
            }
            TurtleCommand::PopFromStack => {
                state = stack.pop_front()
                    .expect("Invalid PopFromStack command: empty stack");
            }
            _ => {
                log::warn!("encountered unknown command {:?}", c);
            }
        }
    }

    matrices
}
