use crate::framework::context::Gpu;
use crate::framework::geometry::bounds::{Bounds, Bounds3};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::scene::transform::{Orientation, Transform, Transformable};
use crate::lsystemrenderer::scene_descriptor::LSystemInstance;
use crate::lsystemrenderer::l_system_manager::command::TurtleCommand;
use glam::{Mat4, Quat, Vec3};
use std::collections::VecDeque;
use std::sync::Arc;
use wgpu::BufferUsages;
use crate::lsystemrenderer::instancing::{Instance, Material};

pub struct LSystemModel {
    aabb: Bounds3,
    cylinder_instances_buffer: Buffer<Instance>,
}

#[derive(Copy, Clone, Debug, Default)]
enum MaterialMode {
    MaterialIndex(usize),
    #[default]
    Random,
}

#[derive(Clone, Debug, Default)]
pub struct MaterialState {
    material_mode: MaterialMode,
    materials: Vec<Material>,
}

impl From<&LSystemInstance> for MaterialState {
    fn from(instance: &LSystemInstance) -> Self {
        let (materials, material_mode) = if let Some(materials) = instance.materials() {
            (
                materials.clone(),
                MaterialMode::MaterialIndex(instance.start_material()),
            )
        } else {
            (Vec::new(), MaterialMode::default())
        };
        Self {
            materials,
            material_mode,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TurtleState {
    transform: Transform,
    initial_orientation: Orientation,
    material_state: MaterialState,
}

impl TurtleState {
    pub fn rotate_to_horizontal(&mut self) {
        let orientation = Orientation::new(self.transform.forward(), self.initial_orientation.up());
        self.transform.set_orientation(orientation);
    }
    pub fn transform(&self) -> Transform {
        self.transform
    }

    fn make_random_material(&self) -> Material {
        Material::new(
            Vec3::new(
                js_sys::Math::random() as f32,
                js_sys::Math::random() as f32,
                js_sys::Math::random() as f32,
            )
            .extend(1.),
        )
    }

    pub fn get_material(&self) -> Material {
        match self.material_state.material_mode {
            MaterialMode::MaterialIndex(idx) => *self
                .material_state
                .materials
                .get(idx)
                .unwrap_or(&self.make_random_material()),
            MaterialMode::Random => self.make_random_material(),
        }
    }
}

impl LSystemModel {
    pub fn from_turtle_commands(
        commands: &Vec<TurtleCommand>,
        l_system_transform: Transform,
        initial_material_state: MaterialState,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let mut aabb = Bounds3::new(Vec3::ZERO, Vec3::ZERO);
        let mut cylinder_instances: Vec<Instance> = Vec::new();

        let mut stack = VecDeque::new();
        let mut state = TurtleState {
            material_state: initial_material_state,
            ..Default::default()
        };

        let cylinder_base_rotation = Quat::from_rotation_x(f32::to_radians(-90.));
        for c in commands {
            match c {
                TurtleCommand::AddCylinder(cylinder) => {
                    let scale_vec =
                        Vec3::new(cylinder.radius(), cylinder.length(), cylinder.radius());
                    let cylinder_transform =
                        Transform::from_scale_rotation(scale_vec, cylinder_base_rotation);

                    cylinder_instances.push(Instance::new(
                        state.transform().as_mat4_with_child(&cylinder_transform), //matrix,
                        state.get_material(),
                    ));

                    state.transform.move_forward(cylinder.length());

                    // the bounding box is only approximated by the turtle's position
                    aabb.grow(state.transform().position());
                }
                TurtleCommand::MoveForward(t) => {
                    state.transform.move_forward(t.length());
                }
                TurtleCommand::RotateYaw(yaw) => {
                    state.transform.yaw_deg(yaw.angle());
                }
                TurtleCommand::RotatePitch(pitch) => {
                    state.transform.pitch_deg(pitch.angle());
                }
                TurtleCommand::RotateRoll(roll) => {
                    state.transform.roll_deg(roll.angle());
                }
                TurtleCommand::Yaw180 => {
                    state.transform.yaw_deg(180.);
                }
                TurtleCommand::PushToStack => {
                    stack.push_front(state.clone());
                }
                TurtleCommand::PopFromStack => {
                    state = stack
                        .pop_front()
                        .expect("Invalid PopFromStack command: empty stack");
                }
                TurtleCommand::ToHorizontal => {
                    state.rotate_to_horizontal();
                }
                TurtleCommand::SetMaterialIndex(set_material_index) => {
                    state.material_state.material_mode =
                        MaterialMode::MaterialIndex(set_material_index.material_index());
                }
                TurtleCommand::AddPredefinedSurface(surface_command) => {
                    log::debug!("unhandled add surface command {:?}", surface_command);
                }
                TurtleCommand::BeginSurface(surface_command) => {
                    log::debug!("unhandled begin surface command {:?}", surface_command);
                }
                TurtleCommand::EndSurface(surface_command) => {
                    log::debug!("unhandled end surface command {:?}", surface_command);
                }
                TurtleCommand::BeginPolygon => {
                    log::debug!("unhandled begin polygon command");
                }
                TurtleCommand::EndPolygon => {
                    log::debug!("unhandled end polygon command");
                }
                TurtleCommand::RecordVertex => {
                    log::debug!("unhandled record vertex command");
                }
                TurtleCommand::Unknown => {
                    log::debug!("encountered unknown command");
                }
            }
        }

        let model_transform = l_system_transform.as_mat4();
        let scale_value = 1. / aabb.diagonal().max_element();
        let model_translation = Mat4::from_translation(-aabb.center());
        let model_scale = Mat4::from_scale(Vec3::new(scale_value, scale_value, scale_value));

        cylinder_instances.iter_mut().for_each(|c| {
            c.set_matrix(model_transform
                .mul_mat4(&model_scale)
                .mul_mat4(&model_translation)
                .mul_mat4(&c.matrix())
            );
        });

        let instances_buffer =
            Buffer::from_data("", &cylinder_instances, BufferUsages::STORAGE, gpu);

        Self {
            aabb,
            cylinder_instances_buffer: instances_buffer,
        }
    }

    pub fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    pub fn cylinder_instances_buffer(&self) -> &Buffer<Instance> {
        &self.cylinder_instances_buffer
    }
}
