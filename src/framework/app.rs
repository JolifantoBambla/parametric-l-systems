use std::future::Future;
use wgpu::{SubmissionIndex, TextureView};
use winit::event_loop::EventLoopBuilder;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::framework::context::{ContextDescriptor, SurfaceContext, SurfaceTarget, WgpuContext};
use crate::framework::event::lifecycle::{OnCommandsSubmitted, PrepareRender, Update};
use crate::framework::event::window::{OnResize, OnUserEvent, OnWindowEvent};
use crate::framework::input::Input;
#[cfg(target_arch = "wasm32")]
use crate::framework::util::web::get_or_create_window;
use crate::framework::util::window::WindowConfig;

pub trait GpuApp: OnUserEvent {
    fn init(
        &mut self,
        window: &Window,
        event_loop: &EventLoop<Self::UserEvent>,
        context: &SurfaceContext,
    );
    fn render(&mut self, view: &TextureView, input: &Input) -> SubmissionIndex;
    fn get_context_descriptor() -> ContextDescriptor<'static>;
}

pub struct AppRunner<
    G: 'static + GpuApp + OnResize + OnWindowEvent + Update + PrepareRender + OnCommandsSubmitted,
> {
    ctx: WgpuContext,
    event_loop: Option<EventLoop<G::UserEvent>>,
    window: Window,
    app: G,
}

impl<G: 'static + GpuApp + OnResize + OnWindowEvent + Update + PrepareRender + OnCommandsSubmitted>
AppRunner<G> {
    #[cfg(target_arch = "wasm32")]
    pub async fn new<'a, F, Fut>(
        window_config: WindowConfig,
        create_app: F
    ) -> Self
    where
        F: FnOnce(&Window, &EventLoop<G::UserEvent>, &'a SurfaceContext) -> Fut,
        Fut: Future<Output = G> + 'a,
    {
        let event_loop = EventLoopBuilder::<G::UserEvent>::with_user_event().build();
        let window = get_or_create_window(&window_config, &event_loop);

        let context = WgpuContext::new(
            &G::get_context_descriptor(),
            Some(SurfaceTarget::Window(&window)),
        ).await;

        let app = create_app(&window, &event_loop, context.surface_context()).await;

        AppRunner {
            ctx: context,
            event_loop: Some(event_loop),
            window,
            app
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run(mut self) {
        let mut input = Input::new(
            self.ctx().surface_configuration().width,
            self.ctx().surface_configuration().height,
        );

        let event_loop = self.event_loop.take().unwrap();

        self.app.init(&self.window, &event_loop, self.ctx());

        log::debug!("Starting event loop");
        event_loop.spawn(move |event, _, control_flow| {
            // force ownership by the closure
            let _ = (self.ctx.instance(), self.ctx.adapter());

            *control_flow = ControlFlow::Poll;

            match event {
                event::Event::RedrawEventsCleared => {
                    self.window.request_redraw();
                }
                event::Event::WindowEvent {
                    event:
                        WindowEvent::Resized(size)
                        | WindowEvent::ScaleFactorChanged {
                            new_inner_size: &mut size,
                            ..
                        },
                    ..
                } => {
                    log::debug!("Resizing to {:?}", size);
                    let width = size.width.max(1);
                    let height = size.height.max(1);
                    self.ctx.surface_context_mut().on_resize(width, height);
                    self.app.on_resize(width, height);
                    input.on_resize(width, height);
                }
                event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input:
                            event::KeyboardInput {
                                virtual_keycode: Some(event::VirtualKeyCode::Escape),
                                state: event::ElementState::Pressed,
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {
                        input.on_window_event(&event);
                        self.app.on_window_event(&event);
                    }
                },
                event::Event::UserEvent(e) => {
                    self.app.on_user_event(&e);
                }
                event::Event::RedrawRequested(_) => {
                    let frame_input = input.prepare_next();
                    self.app.update(&frame_input);

                    self.app.prepare_render(&frame_input);

                    let frame = match self.ctx().surface().get_current_texture() {
                        Ok(frame) => frame,
                        Err(_) => {
                            self.ctx().configure_surface();
                            self.ctx()
                                .surface()
                                .get_current_texture()
                                .expect("Failed to acquire next surface texture!")
                        }
                    };
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let submission_index = self.app.render(&view, &frame_input);
                    self.app.on_commands_submitted(&frame_input, &submission_index);

                    frame.present();
                }
                _ => {}
            }
        });
    }

    pub fn event_loop(&self) -> &Option<EventLoop<G::UserEvent>> {
        &self.event_loop
    }
    pub fn ctx(&self) -> &SurfaceContext {
        self.ctx.surface_context()
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
}
