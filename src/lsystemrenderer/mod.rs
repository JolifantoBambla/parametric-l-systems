use std::collections::HashMap;
use crate::framework::app::GpuApp;
use crate::framework::context::{ContextDescriptor, Gpu, SurfaceContext};
use crate::framework::event::lifecycle::{OnCommandsSubmitted, PrepareRender, Update};
#[cfg(target_arch = "wasm32")]
use crate::framework::event::web::{
    dispatch_canvas_event, register_custom_canvas_event_dispatcher,
};
use crate::framework::event::window::{OnResize, OnUserEvent, OnWindowEvent};
use crate::framework::input::Input;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::event::{LSystemEvent, SceneEvent, UiEvent};
use crate::lsystemrenderer::renderer::Renderer;
use crate::lsystemrenderer::scene::LSystemScene;
use std::sync::Arc;
use wgpu::{
    CommandEncoderDescriptor, DownlevelCapabilities, DownlevelFlags, Label, Limits, ShaderModel,
    SubmissionIndex, SurfaceConfiguration, TextureView,
};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;
use crate::lsystemrenderer::scene_descriptor::LSystemSceneDescriptor;

pub mod camera;
pub mod event;
pub mod renderer;
pub mod scene;
pub mod scene_descriptor;
pub mod turtle;

pub struct App {
    gpu: Arc<Gpu>,
    scene: LSystemScene,
    renderer: Renderer,
}

impl App {
    pub fn new(
        gpu: &Arc<Gpu>,
        surface_configuration: &SurfaceConfiguration,
        l_systems: HashMap<String, HashMap<String, LSystem>>,
        scene_descriptor: LSystemSceneDescriptor,
    ) -> Self {
        let width = surface_configuration.width;
        let height = surface_configuration.height;
        let aspect_ratio = width as f32 / height as f32;

        let renderer = Renderer::new(gpu, surface_configuration);
        let scene = LSystemScene::new(l_systems, &scene_descriptor, aspect_ratio, gpu);

        Self {
            gpu: gpu.clone(),
            scene,
            renderer,
        }
    }
}

impl GpuApp for App {
    fn init(
        &mut self,
        window: &Window,
        event_loop: &EventLoop<Self::UserEvent>,
        _context: &SurfaceContext,
    ) {
        #[cfg(target_arch = "wasm32")]
        {
            let canvas = window.canvas();
            register_custom_canvas_event_dispatcher("ui::scene::background-color", &canvas, event_loop);
            register_custom_canvas_event_dispatcher("ui::scene::new", &canvas, event_loop);
            register_custom_canvas_event_dispatcher("ui::lsystem::iteration", &canvas, event_loop);
            if dispatch_canvas_event("app::initialized", &canvas).is_err() {
                log::error!("Could not dispatch 'app::initialized' event");
            }
        }
    }

    fn render(&mut self, view: &TextureView, _input: &Input) -> SubmissionIndex {
        let mut command_encoder =
            self.gpu
                .device()
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Label::from("frame command encoder"),
                });
        self.renderer
            .render(view, &self.scene, &mut command_encoder);
        self.gpu.queue().submit(vec![command_encoder.finish()])
    }

    fn get_context_descriptor() -> ContextDescriptor<'static> {
        ContextDescriptor {
            required_limits: Limits::downlevel_webgl2_defaults(),
            required_downlevel_capabilities: DownlevelCapabilities {
                flags: DownlevelFlags::empty(),
                shader_model: ShaderModel::Sm5,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl OnResize for App {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.scene.on_resize(width, height);
        self.renderer.on_resize(width, height);
    }
}

impl OnUserEvent for App {
    type UserEvent = UiEvent;

    fn on_user_event(&mut self, event: &Self::UserEvent) {
        match event {
            UiEvent::LSystem(LSystemEvent::Iteration(iteration)) => {
                self.scene.set_target_iteration(iteration.object_name(), iteration.iteration());
            }
            UiEvent::Scene(SceneEvent::BackgroundColor(color)) => {
                self.scene.set_background_color(*color);
            },
            UiEvent::Scene(SceneEvent::New(new_scene)) => {
                self.scene = LSystemScene::new(
                    LSystem::from_l_system_definitions(new_scene.l_system_definitions()),
                    new_scene.scene_descriptor(),
                    self.scene.aspect_ratio(),
                    &self.gpu
                );
            }
        }
    }
}

impl OnWindowEvent for App {
    fn on_window_event(&mut self, _event: &WindowEvent) {}
}

impl Update for App {
    fn update(&mut self, input: &Input) {
        self.scene.update(input);
    }
}

impl PrepareRender for App {
    fn prepare_render(&mut self, _input: &Input) {
        self.scene.prepare_render(
            self.renderer.render_object_creator(),
            self.renderer.light_sources_bind_group_creator(),
        );
    }
}

impl OnCommandsSubmitted for App {
    fn on_commands_submitted(&mut self, _input: &Input, _submission_index: &SubmissionIndex) {}
}
