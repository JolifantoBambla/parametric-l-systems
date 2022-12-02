use glam::Vec2;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use crate::framework::util::window::Resize;

#[derive(Copy, Clone, Debug, Default)]
pub struct MouseState {
    cursor_position: Vec2,
    left_button_pressed: bool,
    right_button_pressed: bool,
    middle_button_pressed: bool,
    other_buttons_pressed: bool,
}

impl MouseState {
    pub fn cursor_position(&self) -> Vec2 {
        self.cursor_position
    }
    pub fn left_button_pressed(&self) -> bool {
        self.left_button_pressed
    }
    pub fn right_button_pressed(&self) -> bool {
        self.right_button_pressed
    }
    pub fn middle_button_pressed(&self) -> bool {
        self.middle_button_pressed
    }
    pub fn other_buttons_pressed(&self) -> bool {
        self.other_buttons_pressed
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MouseButtonEvent {
    button: MouseButton,
    state: ElementState,
}

impl MouseButtonEvent {
    pub fn button(&self) -> MouseButton {
        self.button
    }
    pub fn state(&self) -> ElementState {
        self.state
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MouseMove {
    /// The cursor's state after this event.
    state: MouseState,

    /// The difference between the cursor's current and last position.
    delta: Vec2,
}

impl MouseMove {
    pub fn state(&self) -> MouseState {
        self.state
    }
    pub fn delta(&self) -> Vec2 {
        self.delta
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MouseScroll {
    delta: f32,
}

impl MouseScroll {
    pub fn delta(&self) -> f32 {
        self.delta
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MouseEvent {
    Move(MouseMove),
    Button(MouseButtonEvent),
    Scroll(MouseScroll),
}

#[derive(Clone, Debug, Default)]
pub struct Mouse {
    window_size: Vec2,
    state: MouseState,
    events: Vec<MouseEvent>,
}

impl Mouse {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            window_size: Vec2::new(width as f32, height as f32),
            state: MouseState::default(),
            events: Vec::new(),
        }
    }

    pub fn next(&self) -> Self {
        Self {
            window_size: self.window_size.clone(),
            state: self.state.clone(),
            events: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let new_position = Vec2::new(position.x as f32, position.y as f32);
                let delta = (new_position - self.state.cursor_position) / self.window_size;
                self.state.cursor_position = new_position;
                self.events.push(MouseEvent::Move(MouseMove {
                    state: self.state.clone(),
                    delta
                }));
            }
            WindowEvent::CursorEntered { .. } => {
                log::warn!("cursor enter event not handled");
            }
            WindowEvent::CursorLeft { .. } => {
                log::warn!("cursor left event not handled");
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::PixelDelta(delta),
                ..
            } => {
                self.events.push(MouseEvent::Scroll(MouseScroll { delta: delta.y as f32 }));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let state = match state {
                    ElementState::Pressed => ElementState::Pressed,
                    ElementState::Released => ElementState::Released,
                };
                match button {
                    MouseButton::Left => {
                        self.state.left_button_pressed = state == ElementState::Pressed
                    }
                    MouseButton::Right => {
                        self.state.right_button_pressed = state == ElementState::Pressed
                    }
                    MouseButton::Middle => {
                        self.state.middle_button_pressed = state == ElementState::Pressed
                    }
                    MouseButton::Other(_) => {
                        self.state.other_buttons_pressed = state == ElementState::Pressed
                    }
                }
                self.events.push(MouseEvent::Button(MouseButtonEvent {
                    button: button.clone(),
                    state: state.clone(),
                }));
            }
            _ => {}
        }
    }

    pub fn state(&self) -> MouseState {
        self.state
    }
    pub fn events(&self) -> &Vec<MouseEvent> {
        &self.events
    }
}

impl Resize for Mouse {
    fn resize(&mut self, width: u32, height: u32) {
        self.window_size = Vec2::new(width as f32, height as f32);
    }
}
