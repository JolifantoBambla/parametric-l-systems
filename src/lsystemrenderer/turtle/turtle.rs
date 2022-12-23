use glam::{Mat3, Mat4, Quat, Vec3, Vec4};
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device, Label,
    RenderPass,
};
use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::geometry::bounds::{Bounds, Bounds3};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::mesh::mesh::Mesh;
use crate::framework::mesh::vertex::Vertex;
use crate::framework::renderer::drawable::{Draw, DrawInstanced, GpuMesh};
use crate::framework::scene::transform::{Orientation, Transform, Transformable};
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::renderer::{RenderObject, RenderObjectCreator};
use crate::lsystemrenderer::scene_descriptor::LSystemInstance;
use crate::lsystemrenderer::turtle::command::TurtleCommand;

#[repr(C)]
#[derive(Copy, Clone, Debug, Deserialize, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    color: Vec4,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Vec4::ONE,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    matrix: Mat4,
    material: Material,
}

pub struct LSystemModel {
    aabb: Bounds3,
    cylinder_instances_buffer: Buffer<Instance>,
}

#[derive(Copy, Clone, Debug, Default)]
enum MaterialMode {
    MaterialIndex(usize),
    White,
    Black,
    Red,
    Green,
    Blue,
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
            (materials.clone(), MaterialMode::MaterialIndex(instance.start_material()))
        } else {
            (Vec::new(), MaterialMode::default())
        };
        Self {
            materials,
            material_mode
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
        let orientation = Orientation::new(
            self.transform.forward(),
            self.initial_orientation.up(),
        );
        self.transform.set_orientation(orientation);
    }
    pub fn transform(&self) -> Transform {
        self.transform
    }

    fn make_random_material(&self) -> Material {
        Material {
            color: Vec3::new(
                js_sys::Math::random() as f32,
                js_sys::Math::random() as f32,
                js_sys::Math::random() as f32,
            ).extend(1.)
        }
    }

    pub fn get_material(&self) -> Material {
        match self.material_state.material_mode {
            MaterialMode::MaterialIndex(idx) => {
                *self.material_state.materials.get(idx)
                    .unwrap_or(&self.make_random_material())
            }
            MaterialMode::White => Material { color: Vec4::ONE },
            MaterialMode::Black => Material { color: Vec3::ZERO.extend(1.0) },
            MaterialMode::Red => Material { color: Vec3::X.extend(1.0) },
            MaterialMode::Green => Material { color: Vec3::Y.extend(1.0) },
            MaterialMode::Blue => Material { color: Vec3::Z.extend(1.0) },
            MaterialMode::Random => self.make_random_material()
        }
    }
}

impl LSystemModel {
    fn from_turtle_commands(
        commands: &Vec<TurtleCommand>,
        l_system_transform: Transform,
        initial_material_state: MaterialState,
        gpu: &Arc<Gpu>
    ) -> Self {
        let mut aabb = Bounds3::new(Vec3::ZERO, Vec3::ZERO);
        let mut cylinder_instances: Vec<Instance> = Vec::new();

        let mut stack = VecDeque::new();
        let mut state = TurtleState {
            material_state: initial_material_state,
            ..Default::default()
        };

        let cylinder_base_rotation = Quat::from_rotation_x((-90. as f32).to_radians());
        for c in commands {
            match c {
                TurtleCommand::AddCylinder(cylinder) => {
                    let scale_vec =
                        Vec3::new(cylinder.radius(), cylinder.length(), cylinder.radius());
                    let cylinder_transform =
                        Transform::from_scale_rotation(scale_vec, cylinder_base_rotation);

                    cylinder_instances.push(Instance {
                        matrix: state.transform().as_mat4_with_child(&cylinder_transform), //matrix,
                        material: state.get_material()
                    });

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
                    state.material_state.material_mode = MaterialMode::MaterialIndex(
                        set_material_index.material_index()
                    );
                }
                TurtleCommand::AddPredefinedSurface(surface_command) => {
                    log::debug!("unhandled add surface command {:?}", surface_command);
                },
                TurtleCommand::BeginSurface(surface_command) => {
                    log::debug!("unhandled begin surface command {:?}", surface_command);
                },
                TurtleCommand::EndSurface(surface_command) => {
                    log::debug!("unhandled end surface command {:?}", surface_command);
                },
                TurtleCommand::BeginPolygon => {
                    log::debug!("unhandled begin polygon command");
                }
                TurtleCommand::EndPolygon => {
                    log::debug!("unhandled end polygon command");
                },
                TurtleCommand::RecordVertex => {
                    log::debug!("unhandled record vertex command");
                },
                TurtleCommand::Unknown => {
                    log::debug!("encountered unknown command");
                }
            }
        }

        let model_transform = l_system_transform.as_mat4();
        let scale_value = 1. / aabb.diagonal().max_element();
        let model_translation = Mat4::from_translation(-aabb.center());
        let model_scale = Mat4::from_scale(
            Vec3::new(scale_value, scale_value, scale_value),
        );

        cylinder_instances.iter_mut().for_each(|c| {
            c.matrix = model_transform.mul_mat4(&model_scale)
                .mul_mat4(&model_translation)
                .mul_mat4(&c.matrix);
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

pub struct LSystemManager {
    gpu: Arc<Gpu>,
    max_time_to_iterate: f32,
    transform: Transform,
    l_system: LSystem,
    max_target_iteration: u32,
    iterations: Vec<LSystemModel>,
    material_state: MaterialState,
}

impl LSystemManager {
    pub fn new(
        l_system: LSystem,
        transform: Transform,
        initial_material_state: Option<MaterialState>,
        gpu: &Arc<Gpu>
    ) -> Self {
        let mut iterations = Vec::new();
        let commands: Vec<TurtleCommand> = serde_wasm_bindgen::from_value(l_system.next_raw())
            .expect("Could not parse turtle commands");

        let material_state = initial_material_state.unwrap_or(MaterialState::default());
        iterations.push(LSystemModel::from_turtle_commands(
            &commands,
            transform,
            material_state.clone(),
            gpu
        ));

        Self {
            gpu: gpu.clone(),
            max_time_to_iterate: 50.,
            transform,
            l_system,
            max_target_iteration: 0,
            iterations,
            material_state,
        }
    }

    pub fn maybe_increase_max_iteration(&mut self, max_iteration: u32) {
        self.max_target_iteration = max_iteration.max(self.max_target_iteration);
    }

    pub fn try_get_iteration(&self, iteration: u32) -> (u32, &LSystemModel) {
        if self.iterations.len() as u32 > iteration {
            (iteration, &self.iterations[iteration as usize])
        } else {
            let i = self.iterations.len() - 1;
            (i as u32, &self.iterations[i])
        }
    }
}

impl Update for LSystemManager {
    fn update(&mut self, input: &Input) {
        while self.max_target_iteration >= self.iterations.len() as u32 {
            let commands: Vec<TurtleCommand> =
                serde_wasm_bindgen::from_value(self.l_system.next_raw())
                    .expect("Could not parse turtle commands");
            self.iterations
                .push(LSystemModel::from_turtle_commands(
                    &commands,
                    self.transform,
                    self.material_state.clone(),
                    &self.gpu
                ));
            if instant::now() as f32 - input.time().now() >= self.max_time_to_iterate {
                break;
            }
        }
    }
}

impl Drop for LSystemManager {
    fn drop(&mut self) {
        for iteration in self.iterations.iter_mut() {
            iteration.cylinder_instances_buffer.buffer().destroy();
        }
    }
}
