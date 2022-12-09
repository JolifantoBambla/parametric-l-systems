use glam::{Mat3, Mat4, Quat, Vec3, Vec4};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device, Label,
    RenderPass,
};
use crate::framework::context::Gpu;
use crate::framework::geometry::bounds::{Bounds, Bounds3};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::event::lifecycle::OnUpdate;
use crate::framework::mesh::mesh::Mesh;
use crate::framework::mesh::vertex::Vertex;
use crate::framework::renderer::drawable::{Draw, DrawInstanced, GpuMesh};
use crate::framework::scene::transform::{Orientation, Transform};

use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::turtle::command::TurtleCommand;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    matrix: Mat4,
    color: Vec4,
}

#[derive(Debug)]
pub struct LSystemModel {
    aabb: Bounds3,
    num_instances: u32,
    cylinder_instances_bind_group: BindGroup,
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
        bind_group_layout: &Arc<BindGroupLayout>,
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
                    let scale_vec = Vec3::new(
                        cylinder.radius(),
                        cylinder.length(),
                        cylinder.radius(),
                    );
                    let cylinder_transform = Transform::from_scale_rotation(
                        scale_vec,
                        base_rotation
                    );

                    let color = Vec3::new(
                        js_sys::Math::random() as f32,
                        js_sys::Math::random() as f32,
                        js_sys::Math::random() as f32,
                    );
                    cylinder_instances.push(Instance {
                        matrix: state.transform().as_mat4_with_child(&cylinder_transform),//matrix,
                        color: color.extend(1.0),
                    });

                    state.transform.move_forward(cylinder.length());

                    // todo: take cylinder into account
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

        let instances_buffer =
            Buffer::from_data("", &cylinder_instances, BufferUsages::STORAGE, gpu);

        let cylinder_instances_bind_group = gpu.device().create_bind_group(&BindGroupDescriptor {
            label: Label::from("instances bind group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: instances_buffer.buffer().as_entire_binding(),
            }],
        });

        Self {
            aabb,
            num_instances: cylinder_instances.len() as u32,
            cylinder_instances_bind_group,
        }
    }

    pub fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    pub fn cylinder_instances_bind_group(&self) -> &BindGroup {
        &self.cylinder_instances_bind_group
    }
}

pub struct LSystemManager {
    gpu: Arc<Gpu>,
    instance_buffer_bind_group_index: u32,
    instance_buffer_bind_group_layout: Arc<BindGroupLayout>,
    max_time_to_iterate: f32,
    cylinder_mesh: Arc<GpuMesh>,
    l_system: LSystem,
    target_iteration: u32,
    active_iteration: u32,
    iterations: Vec<LSystemModel>,
}

impl LSystemManager {
    pub fn new(
        l_system: LSystem,
        instance_buffer_bind_group_layout: &Arc<BindGroupLayout>,
        instance_buffer_bind_group_index: u32,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let cylinder_mesh = Arc::new(GpuMesh::from_mesh::<Vertex>(
            &Mesh::new_default_cylinder(true),
            gpu.device(),
        ));

        let mut iterations = Vec::new();
        let commands: Vec<TurtleCommand> = serde_wasm_bindgen::from_value(l_system.next_raw())
            .expect("Could not parse turtle commands");

        iterations.push(LSystemModel::from_turtle_commands(
            &commands,
            instance_buffer_bind_group_layout,
            gpu,
        ));

        Self {
            gpu: gpu.clone(),
            instance_buffer_bind_group_index,
            instance_buffer_bind_group_layout: instance_buffer_bind_group_layout.clone(),
            max_time_to_iterate: 50.,
            cylinder_mesh,
            l_system,
            target_iteration: 0,
            active_iteration: 0,
            iterations,
        }
    }

    pub fn set_target_iteration(&mut self, target_iteration: u32) {
        self.target_iteration = target_iteration;
        if self.target_iteration < self.iterations.len() as u32 {
            self.active_iteration = self.target_iteration
        }
    }
}

impl OnUpdate for LSystemManager {
    fn on_update(&mut self, input: &Input) {
        while self.target_iteration >= self.iterations.len() as u32 {
            let commands: Vec<TurtleCommand> =
                serde_wasm_bindgen::from_value(self.l_system.next_raw())
                    .expect("Could not parse turtle commands");
            self.iterations.push(LSystemModel::from_turtle_commands(
                &commands,
                &self.instance_buffer_bind_group_layout,
                &self.gpu,
            ));
            self.active_iteration = self.iterations.len() as u32 - 1;
            if instant::now() as f32 - input.time().now() >= self.max_time_to_iterate {
                break;
            }
        }
    }
}

impl Draw for LSystemManager {
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        let iteration = self
            .iterations
            .get(self.active_iteration as usize)
            .expect("could not access active iteration");
        pass.set_bind_group(
            self.instance_buffer_bind_group_index,
            &iteration.cylinder_instances_bind_group,
            &[],
        );
        self.cylinder_mesh
            .draw_instanced(pass, iteration.num_instances);
    }
}
