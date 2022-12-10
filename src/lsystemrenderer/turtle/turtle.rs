use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::geometry::bounds::{Bounds, Bounds3};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::mesh::mesh::Mesh;
use crate::framework::mesh::vertex::Vertex;
use crate::framework::renderer::drawable::{Draw, DrawInstanced, GpuMesh};
use crate::framework::scene::transform::{Orientation, Transform, Transformable};
use glam::{Mat3, Mat4, Quat, Vec3, Vec4};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device, Label,
    RenderPass,
};

use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::renderer::{RenderObject, RenderObjectCreator};
use crate::lsystemrenderer::turtle::command::TurtleCommand;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    matrix: Mat4,
    color: Vec4,
}

pub struct LSystemModel {
    aabb: Bounds3,
    cylinder_instances_buffer: Buffer<Instance>,
}

#[derive(Copy, Clone, Debug, Default)]
struct TurtleState {
    transform: Transform,
    initial_orientation: Orientation,
}

impl TurtleState {
    pub fn reset_orientation(&mut self) {
        self.transform.set_orientation(self.initial_orientation);
    }
    pub fn transform(&self) -> Transform {
        self.transform
    }
}

impl LSystemModel {
    pub fn from_turtle_commands(
        commands: &Vec<TurtleCommand>,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let mut aabb = Bounds3::new(Vec3::ZERO, Vec3::ZERO);
        let mut cylinder_instances: Vec<Instance> = Vec::new();

        let mut stack = VecDeque::new();
        let mut state = TurtleState::default();

        let base_rotation = Quat::from_rotation_x((-90. as f32).to_radians());
        for c in commands {
            match c {
                TurtleCommand::AddCylinder(cylinder) => {
                    let scale_vec =
                        Vec3::new(cylinder.radius(), cylinder.length(), cylinder.radius());
                    let cylinder_transform =
                        Transform::from_scale_rotation(scale_vec, base_rotation);

                    let color = Vec3::new(
                        js_sys::Math::random() as f32,
                        js_sys::Math::random() as f32,
                        js_sys::Math::random() as f32,
                    );
                    cylinder_instances.push(Instance {
                        matrix: state.transform().as_mat4_with_child(&cylinder_transform), //matrix,
                        color: color.extend(1.0),
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
                    state.reset_orientation();
                }
                _ => {
                    log::debug!("encountered unknown command {:?}", c);
                }
            }
        }

        let scale_value = 1. / aabb.diagonal().max_element();
        let model_translation = Mat4::from_translation(-aabb.center());
        let model_scale = Mat4::from_scale(Vec3::new(scale_value, scale_value, scale_value));

        cylinder_instances.iter_mut()
            .for_each(|c| {
                c.matrix = model_scale.mul_mat4(&model_translation)
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
    cylinder_mesh: Arc<GpuMesh>,
    l_system: LSystem,
    target_iteration: u32,
    active_iteration: u32,
    iterations: Vec<LSystemModel>,
    render_objects: Vec<Vec<RenderObject>>,
}

impl LSystemManager {
    pub fn new(l_system: LSystem, gpu: &Arc<Gpu>) -> Self {
        let cylinder_mesh = Arc::new(GpuMesh::from_mesh::<Vertex>(
            &Mesh::new_default_cylinder(true),
            gpu.device(),
        ));

        let mut iterations = Vec::new();
        let commands: Vec<TurtleCommand> = serde_wasm_bindgen::from_value(l_system.next_raw())
            .expect("Could not parse turtle commands");

        iterations.push(LSystemModel::from_turtle_commands(
            &commands,
            gpu,
        ));

        Self {
            gpu: gpu.clone(),
            max_time_to_iterate: 50.,
            cylinder_mesh,
            l_system,
            target_iteration: 0,
            active_iteration: 0,
            iterations,
            render_objects: Vec::new(),
        }
    }

    pub fn set_target_iteration(&mut self, target_iteration: u32) {
        self.target_iteration = target_iteration;
        if self.target_iteration < self.iterations.len() as u32 {
            self.active_iteration = self.target_iteration
        }
    }

    pub fn prepare_render(&mut self, render_object_creator: &RenderObjectCreator) {
        if self.render_objects.len() <= self.active_iteration as usize {
            let active_iteration = self.iterations.get(self.active_iteration as usize)
                .expect("Active iteration does not exist");
            self.render_objects.push(vec![render_object_creator.create_render_object(
                &self.cylinder_mesh,
                &active_iteration.cylinder_instances_buffer
            )]);
        }
    }

    pub fn get_render_objects(&self) -> &Vec<RenderObject> {
        self.render_objects.get(self.active_iteration as usize)
            .expect("Active render objects do not exist")
    }
}

impl Update for LSystemManager {
    fn update(&mut self, input: &Input) {
        while self.target_iteration >= self.iterations.len() as u32 {
            let commands: Vec<TurtleCommand> =
                serde_wasm_bindgen::from_value(self.l_system.next_raw())
                    .expect("Could not parse turtle commands");
            self.iterations.push(LSystemModel::from_turtle_commands(
                &commands,
                &self.gpu,
            ));
            self.active_iteration = self.iterations.len() as u32 - 1;
            if instant::now() as f32 - input.time().now() >= self.max_time_to_iterate {
                break;
            }
        }
    }
}
