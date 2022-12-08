use glam::Mat4;

pub struct StaticTransform {
    object_to_world: Mat4,
    world_to_object: Mat4,
}

impl StaticTransform {
    pub fn new(object_to_world: Mat4, world_to_object: Mat4) -> Self {
        Self {
            object_to_world,
            world_to_object,
        }
    }
}

pub struct DynamicTransform {}

pub enum Transform {
    Static(StaticTransform),
    Dynamic(DynamicTransform),
}

impl Transform {
    fn object_to_world(&self) -> Mat4 {
        match self {
            Transform::Static(t) => t.object_to_world,
            Transform::Dynamic(t) => {
                todo!("implement dynamic transform")
            }
        }
    }

    fn world_to_object(&self) -> Mat4 {
        match self {
            Transform::Static(t) => t.world_to_object,
            Transform::Dynamic(t) => {
                todo!("implement dynamic transform")
            }
        }
    }
}
