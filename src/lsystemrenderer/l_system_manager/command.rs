use serde::Deserialize;

// F(d,r)
// Positioniere einen Zylinder der Länge d mit Radius r entlang der h-Achse.
// Translation entlang der h-Achse um die Länge d (an den Endpunkt des Zylinders).
#[derive(Debug, Deserialize)]
pub struct AddCylinder {
    parameters: [f32; 2],
}

impl AddCylinder {
    pub fn length(&self) -> f32 {
        if let Some(length) = self.parameters.first() {
            *length
        } else {
            1.
        }
    }

    pub fn radius(&self, default_radius: f32) -> f32 {
        if let Some(radius) = self.parameters.get(1) {
            *radius
        } else {
            default_radius
        }
    }
}

// f(d)
// Translation entlang der H-Achse um die Länge d ohne Konstruktion.
#[derive(Debug, Deserialize)]
pub struct MoveForward {
    parameters: [f32; 1],
}

impl MoveForward {
    pub fn length(&self) -> f32 {
        *self.parameters.first().expect("MoveForward has no length")
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
        if let Some(&angle) = self.parameters.first() {
            angle
        } else {
            f32::to_radians(90.)
        }
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
        if let Some(&angle) = self.parameters.first() {
            angle
        } else {
            f32::to_radians(90.)
        }
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
        if let Some(&angle) = self.parameters.first() {
            angle
        } else {
            f32::to_radians(90.)
        }
    }
}

// |
// Drehung um 180° um die u-Achse (Shortcut für +(180))

// [
// Der aktuelle Zustand der Turtle wird auf einen Stack gelegt

// ]
// Der letzte Zustand wird vom Stack entfernt und die Turtle in diesen Zustand versetzt

#[derive(Debug, Deserialize)]
pub struct SetDefaultCylinderRadius {
    parameters: [f32; 1],
}

impl SetDefaultCylinderRadius {
    pub fn radius(&self) -> f32 {
        *self
            .parameters
            .first()
            .expect("SetDefaultCylinderRadius has no radius")
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveCommandParameter {
    String(String),
    Usize(usize)
}

#[derive(Debug, Deserialize)]
pub struct PrimitiveCommand {
    parameters: Vec<PrimitiveCommandParameter>,
}

impl PrimitiveCommand {
    pub fn name(&self) -> &str {
        match self.parameters
            .get(0)
            .expect("SurfaceCommand has no surface name") {
            PrimitiveCommandParameter::String(name) => name,
            _ => panic!("SurfaceCommand's first parameter is not a String")
        }
    }

    pub fn iteration(&self) -> usize {
        match self.parameters.get(1) {
            None => 0,
            Some(parameter) => {
                match parameter {
                    PrimitiveCommandParameter::Usize(iteration) => *iteration,
                    _ => panic!("SurfaceCommand's second parameter is a String")
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SetMaterialIndex {
    parameters: [f32; 1],
}

impl SetMaterialIndex {
    pub fn material_index(&self) -> usize {
        *self
            .parameters
            .first()
            .expect("SetMaterialIndex has no material index") as usize
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
pub enum TurtleCommand {
    #[serde(rename = "F")]
    AddCylinder(AddCylinder),

    #[serde(rename = "f")]
    MoveForward(MoveForward),

    #[serde(rename = "+")]
    RotateYaw(RotateYaw),

    #[serde(rename = "-")]
    RotateYawNegative(RotateYaw),

    #[serde(rename = "&")]
    RotatePitch(RotatePitch),

    #[serde(rename = "^")]
    RotatePitchNegative(RotatePitch),

    #[serde(rename = "/")]
    RotateRoll(RotateRoll),

    #[serde(rename = "\\")]
    RotateRollNegative(RotateRoll),

    #[serde(rename = "|")]
    Yaw180,

    #[serde(rename = "[")]
    PushToStack,

    #[serde(rename = "]")]
    PopFromStack,

    // every command below this line is not needed for the exercise
    #[serde(rename = "$")]
    ToHorizontal,

    #[serde(rename = "!")]
    SetDefaultCylinderRadius(SetDefaultCylinderRadius),

    #[serde(rename = "~")]
    AddPredefinedPrimitive(PrimitiveCommand),

    #[serde(rename = "BeginSurface")]
    BeginPrimitive(PrimitiveCommand),

    #[serde(rename = "EndSurface")]
    EndPrimitive(PrimitiveCommand),

    #[serde(rename = "{")]
    BeginPolygon,

    #[serde(rename = "}")]
    EndPolygon,

    #[serde(rename = "G")]
    MoveAlongEdge(MoveForward),

    #[serde(rename = ".")]
    RecordVertex,

    #[serde(rename = "´")]
    SetMaterialIndex(SetMaterialIndex),

    #[serde(rename = "%")]
    IgnoreRemainingBranch,

    #[serde(other)]
    Unknown,
}
