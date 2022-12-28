use crate::framework::context::Gpu;
use crate::framework::geometry::bounds::{Bounds, Bounds3};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::scene::transform::{Orientation, Transform, Transformable};
use crate::lsystemrenderer::instancing::{Instance, Material};
use crate::lsystemrenderer::l_system_manager::command::TurtleCommand;
use crate::lsystemrenderer::scene_descriptor::LSystemInstance;
use glam::{Mat4, Quat, Vec3};
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use wgpu::BufferUsages;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Tropism {
    direction: Vec3,
    e: f32,
}

impl Tropism {
    pub fn correct_direction(&self, direction: Vec3) -> Vec3 {
        let alpha = self.e * (self.direction.cross(direction)).length();
        (direction + (self.direction * alpha)).normalize()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LSystemPrimitive {
    transform: Option<Transform>,
    material: Option<Material>,
}

impl LSystemPrimitive {
    pub fn transform(&self) -> Transform {
        self.transform.unwrap_or_default()
    }
    pub fn material(&self) -> Option<Material> {
        self.material
    }
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

#[derive(Clone, Debug)]
struct TurtleState {
    transform: Transform,
    initial_orientation: Orientation,
    material_state: MaterialState,
    default_cylinder_diameter: f32,
    ignoring_branch: bool,
}

impl TurtleState {
    pub fn rotate_to_horizontal(&mut self) {
        self.transform.set_orientation(Orientation::new(
            self.transform.forward(),
            self.initial_orientation.up(),
        ));
    }

    pub fn set_forward(&mut self, forward: Vec3) {
        let orientation = Orientation::new(forward, self.transform.up());
        self.transform.set_orientation(orientation);
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    #[cfg(target_arch = "wasm32")]
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

    pub fn set_default_cylinder_diameter(&mut self, diameter: f32) {
        self.default_cylinder_diameter = diameter;
    }
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            initial_orientation: Default::default(),
            material_state: Default::default(),
            default_cylinder_diameter: 0.5,
            ignoring_branch: false,
        }
    }
}

pub struct LSystemModel {
    aabb: Bounds3,
    cylinder_instances_buffer: Buffer<Instance>,
    primitive_instances_buffers: HashMap<String, HashMap<usize, Buffer<Instance>>>,
}

impl LSystemModel {
    pub fn from_turtle_commands(
        commands: &Vec<TurtleCommand>,
        l_system_transform: Transform,
        initial_material_state: MaterialState,
        primitives: &HashMap<String, LSystemPrimitive>,
        world_tropism: &Option<Tropism>,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let mut aabb = Bounds3::new(Vec3::ZERO, Vec3::ZERO);
        let mut cylinder_instances: Vec<Instance> = Vec::new();
        let mut primitive_instances: HashMap<String, HashMap<usize, Vec<Instance>>> =
            HashMap::new();

        let mut stack = VecDeque::new();
        let mut state = TurtleState {
            material_state: initial_material_state,
            ..Default::default()
        };
        let tropism = world_tropism.as_ref().map(|world_tropism| Tropism {
            direction: l_system_transform
                .as_mat4()
                .inverse()
                .mul_vec4(world_tropism.direction.extend(0.))
                .truncate(),
            e: world_tropism.e,
        });

        let cylinder_base_rotation = Quat::from_rotation_x(f32::to_radians(-90.));
        for c in commands {
            if state.ignoring_branch {
                match c {
                    TurtleCommand::PopFromStack => {
                        state.ignoring_branch = false;
                    }
                    _ => continue,
                }
            }
            match c {
                TurtleCommand::AddCylinder(cylinder) => {
                    let radius = cylinder.diameter(state.default_cylinder_diameter) * 0.5;
                    let scale_vec = Vec3::new(radius, cylinder.length(), radius);
                    let cylinder_transform =
                        Transform::from_scale_rotation(scale_vec, cylinder_base_rotation);

                    cylinder_instances.push(Instance::new(
                        state.transform().as_mat4_with_child(&cylinder_transform),
                        state.get_material(),
                    ));

                    state.transform.move_forward(cylinder.length());

                    // the bounding box is only approximated by the turtle's position
                    aabb.grow(state.transform().position());

                    if let Some(t) = tropism {
                        state.set_forward(t.correct_direction(state.transform.forward()));
                    }
                }
                TurtleCommand::MoveForward(t) => {
                    state.transform.move_forward(t.length());
                }
                TurtleCommand::RotateYaw(yaw) => {
                    state.transform.yaw_deg(yaw.angle());
                }
                TurtleCommand::RotateYawNegative(yaw) => {
                    state.transform.yaw_deg(-yaw.angle());
                }
                TurtleCommand::RotatePitch(pitch) => {
                    state.transform.pitch_deg(pitch.angle());
                }
                TurtleCommand::RotatePitchNegative(pitch) => {
                    state.transform.pitch_deg(-pitch.angle());
                }
                TurtleCommand::RotateRoll(roll) => {
                    state.transform.roll_deg(roll.angle());
                }
                TurtleCommand::RotateRollNegative(roll) => {
                    state.transform.roll_deg(-roll.angle());
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
                TurtleCommand::SetDefaultCylinderDiameter(set_default_cylinder_radius) => {
                    state.set_default_cylinder_diameter(set_default_cylinder_radius.radius());
                }
                TurtleCommand::SetMaterialIndex(set_material_index) => {
                    if !state.material_state.materials.is_empty() {
                        let new_index = if let Some(index) = set_material_index.material_index() {
                            *index
                        } else {
                            let current_index = match state.material_state.material_mode {
                                MaterialMode::MaterialIndex(i) => i,
                                _ => 0,
                            };
                            current_index + 1
                        };
                        state.material_state.material_mode = MaterialMode::MaterialIndex(new_index);
                    }
                }
                TurtleCommand::IgnoreRemainingBranch => {
                    state.ignoring_branch = true;
                }
                TurtleCommand::AddPredefinedPrimitive(surface_command) => {
                    let surface_id = surface_command.name();
                    let surface_iteration = surface_command.iteration();
                    if let Some(primitive) = primitives.get(surface_id) {
                        if !primitive_instances.contains_key(surface_id) {
                            primitive_instances.insert(surface_id.to_string(), HashMap::new());
                        }
                        if !primitive_instances
                            .get(surface_id)
                            .unwrap()
                            .contains_key(&surface_iteration)
                        {
                            primitive_instances
                                .get_mut(surface_id)
                                .unwrap()
                                .insert(surface_iteration, Vec::new());
                        }

                        let instance = Instance::new(
                            state.transform().as_mat4_with_child(&primitive.transform()), //matrix,
                            primitive.material.unwrap_or_else(|| state.get_material()),
                        );

                        primitive_instances
                            .get_mut(surface_id)
                            .unwrap()
                            .get_mut(&surface_iteration)
                            .unwrap()
                            .push(instance);
                    }
                }
                TurtleCommand::BeginPrimitive(surface_command) => {
                    log::debug!("unhandled begin surface command {:?}", surface_command);
                }
                TurtleCommand::EndPrimitive(surface_command) => {
                    log::debug!("unhandled end surface command {:?}", surface_command);
                }
                TurtleCommand::BeginPolygon => {
                    log::debug!("unhandled begin polygon command");
                }
                TurtleCommand::EndPolygon => {
                    log::debug!("unhandled end polygon command");
                }
                TurtleCommand::MoveAlongEdge(_) => {
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

        let scale_value = 1. / aabb.diagonal().max_element();
        let model_transform = l_system_transform
            .as_mat4()
            .mul_mat4(&Mat4::from_scale(Vec3::new(
                scale_value,
                scale_value,
                scale_value,
            )))
            .mul_mat4(&Mat4::from_translation(-aabb.center()));

        cylinder_instances.iter_mut().for_each(|c| {
            c.set_matrix(model_transform.mul_mat4(&c.matrix()));
        });

        let cylinder_instances_buffer =
            Buffer::from_data("", &cylinder_instances, BufferUsages::STORAGE, gpu);

        let mut primitive_instances_buffers = HashMap::new();
        for (id, primitive) in primitive_instances.iter_mut() {
            let mut instances_buffers = HashMap::new();
            for (&iteration, instances) in primitive.iter_mut() {
                instances.iter_mut().for_each(|c| {
                    c.set_matrix(model_transform.mul_mat4(&c.matrix()));
                });

                instances_buffers.insert(
                    iteration,
                    Buffer::from_data("", instances, BufferUsages::STORAGE, gpu),
                );
            }
            primitive_instances_buffers.insert(id.clone(), instances_buffers);
        }

        Self {
            aabb,
            cylinder_instances_buffer,
            primitive_instances_buffers,
        }
    }

    pub fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    pub fn cylinder_instances_buffer(&self) -> &Buffer<Instance> {
        &self.cylinder_instances_buffer
    }

    pub fn primitive_instances(&self) -> &HashMap<String, HashMap<usize, Buffer<Instance>>> {
        &self.primitive_instances_buffers
    }
}
