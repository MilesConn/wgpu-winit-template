use wgpu::naga::proc::NameKey;
use winit::event::{ElementState, KeyEvent, WindowEvent};

pub(super) struct UserInput {
    // TODO: eventually switch over to ROPE but for now
    // no
    user_input: Vec<u8>,
}

impl UserInput {
    pub(super) fn new() -> Self {
        Self {
            user_input: Vec::new(),
        }
    }

    pub(super) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    event @ KeyEvent {
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                use winit::keyboard::{Key, NamedKey::*};
                match event {
                    KeyEvent {
                        text: Some(text), ..
                    } => {
                        // Um is text ever greater than one byte????
                        log::info!("HERE WITH {}", text.as_str());
                        self.user_input.extend_from_slice(text.as_bytes());
                    }
                    KeyEvent {
                        logical_key: Key::Named(key),
                        ..
                    } => match key {
                        Delete => log::info!("Delete was hit!"),
                        _ => {}
                    },
                    _ => {}
                };
                false
            }
            _ => false,
        }
    }
}

impl Default for UserInput {
    fn default() -> Self {
        UserInput::new()
    }
}
