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
        if let Some(length) = self.parameters.get(0) {
            *length
        } else {
            1.
        }
    }

    pub fn radius(&self) -> f32 {
        if let Some(radius) = self.parameters.get(1) {
            *radius
        } else {
            0.5
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
        *self.parameters.get(0).expect("MoveForward has no length")
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
        *self.parameters.get(0).expect("RotateYaw has no angle")
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
        *self.parameters.get(0).expect("RotatePitch has no angle")
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
        *self.parameters.get(0).expect("RotateRoll has no angle")
    }
}

// |
// Drehung um 180° um die u-Achse (Shortcut für +(180))

// [
// Der aktuelle Zustand der Turtle wird auf einen Stack gelegt

// ]
// Der letzte Zustand wird vom Stack entfernt und die Turtle in diesen Zustand versetzt

#[derive(Debug, Deserialize)]
pub struct SurfaceCommand {
    parameters: Vec<String>,
}

impl SurfaceCommand {
    pub fn name(&self) -> &str {
        self.parameters.get(0)
            .expect("SurfaceCommand has no surface name")
    }
}

#[derive(Debug, Deserialize)]
pub struct SetMaterialIndex {
    parameters: [f32; 1],
}

impl SetMaterialIndex {
    pub fn material_index(&self) -> usize {
        *self.parameters.get(0)
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

    #[serde(rename = "~")]
    AddPredefinedSurface(SurfaceCommand),

    #[serde(rename = "BeginSurface")]
    BeginNewSurface(SurfaceCommand),

    #[serde(rename = "EndSurface")]
    EndNewSurface(SurfaceCommand),

    #[serde(rename = "{")]
    StartPolygon,

    #[serde(rename = "}")]
    EndPolygon,

    #[serde(rename = ".")]
    RecordVertex,

    #[serde(rename = "´")]
    SetMaterialIndex(SetMaterialIndex),

    #[serde(other)]
    Unknown,
}
