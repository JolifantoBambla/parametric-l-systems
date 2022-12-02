use std::marker::PhantomData;
use wgpu::{SurfaceConfiguration, TextureView};
use winit::{
    dpi::PhysicalSize,
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

use crate::framework::context::{ContextDescriptor, Gpu, SurfaceContext, SurfaceTarget, WgpuContext};
use crate::framework::input::event_buffer::EventBuffer;
use crate::framework::input::Input;
use crate::framework::scene::Update;
#[cfg(target_arch = "wasm32")]
use crate::framework::util::web::{get_or_create_window};
use crate::framework::util::window::{Resize, WindowConfig};

pub trait GpuApp<UserEvent> {
    fn init(&mut self, event_loop: &EventLoop<()>, context: &SurfaceContext);
    fn on_user_event(&mut self, _event: &UserEvent);
    fn on_window_event(&mut self, _event: &WindowEvent);
    fn render(&mut self, view: &TextureView, input: &Input);
    fn get_context_descriptor() -> ContextDescriptor<'static>;
}

pub struct AppRunner<G: 'static + GpuApp<()> + Resize + Update> {
    ctx: WgpuContext,
    event_loop: Option<EventLoop<()>>,
    window: Window,
    phantom_data: PhantomData<G>,
}

impl<G: 'static + GpuApp<()> + Resize + Update> AppRunner<G> {
    #[cfg(target_arch = "wasm32")]
    pub async fn new(window_config: WindowConfig) -> Self {
        let event_loop = EventLoop::new();
        let window = get_or_create_window(&window_config, &event_loop);

        let context = WgpuContext::new(&G::get_context_descriptor(), Some(SurfaceTarget::Window(&window)))
                .await;

        AppRunner {
            ctx: context,
            event_loop: Some(event_loop),
            window,
            phantom_data: PhantomData,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run<'a>(mut self, mut app: G) {
        let mut input = Input::default();
        let mut event_buffer = EventBuffer::default();

        let event_loop = self.event_loop.take().unwrap();

        app.init(&event_loop, &self.ctx());

        log::debug!("Starting event loop");
        event_loop
            .spawn(move |event, _, control_flow| {
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
                        self.ctx.surface_context_mut().resize(width, height);
                        app.resize(width, height);
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
                            event_buffer.handle_event(&event);
                            app.on_window_event(&event);
                        }
                    },
                    event::Event::UserEvent(e) => {
                        app.on_user_event(&e);
                    },
                    event::Event::RedrawRequested(_) => {
                        input = input.next(event_buffer.drain());
                        app.update(&input);

                        let frame = match self.ctx().surface().get_current_texture() {
                            Ok(frame) => frame,
                            Err(_) => {
                                self.ctx().configure_surface();
                                self.ctx().surface()
                                    .get_current_texture()
                                    .expect("Failed to acquire next surface texture!")
                            }
                        };
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        app.render(&view, &input);

                        frame.present();
                    },
                    _ => {}
                }
            });
    }

    pub fn event_loop(&self) -> &Option<EventLoop<()>> {
        &self.event_loop
    }
    pub fn ctx(&self) -> &SurfaceContext { &self.ctx.surface_context() }
    pub fn window(&self) -> &Window {
        &self.window
    }
}
