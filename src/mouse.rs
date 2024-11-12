use crossterm::event::{self, MouseEvent, MouseEventKind};
use enumset::EnumSetType;

use crate::Context;

#[derive(Debug, EnumSetType)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<event::MouseButton> for MouseButton {
    fn from(btn: event::MouseButton) -> Self {
        match btn {
            event::MouseButton::Left => MouseButton::Left,
            event::MouseButton::Right => MouseButton::Right,
            event::MouseButton::Middle => MouseButton::Middle,
        }
    }
}

impl Context {
    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(btn) => {
                self.current_state.mouse_buttons.insert(btn.into());
            }
            MouseEventKind::Up(btn) => {
                self.current_state.mouse_buttons.remove(btn.into());
            }
            MouseEventKind::Moved => {
                self.current_state.mouse_position.0 = event.column;
                self.current_state.mouse_position.1 = event.row * 2;
                self.current_state
                    .mouse_positions
                    .push(self.current_state.mouse_position);
            }
            MouseEventKind::Drag(btn) => {
                self.current_state.mouse_buttons.insert(btn.into());

                self.current_state.mouse_position.0 = event.column;
                self.current_state.mouse_position.1 = event.row * 2;
                self.current_state
                    .mouse_positions
                    .push(self.current_state.mouse_position);
            }
            _ => {
                panic!("unhandled mouse event: {:?}", event);
            }
        }
    }

    pub fn is_mouse_button_down(&self, btn: MouseButton) -> bool {
        self.current_state.mouse_buttons.contains(btn)
    }

    pub fn is_mouse_button_pressed(&self, btn: MouseButton) -> bool {
        self.current_state.mouse_buttons.contains(btn)
            && !self.previous_state.mouse_buttons.contains(btn)
    }

    pub fn is_mouse_button_released(&self, btn: MouseButton) -> bool {
        !self.current_state.mouse_buttons.contains(btn)
            && self.previous_state.mouse_buttons.contains(btn)
    }

    pub fn mouse_position(&self) -> (u16, u16) {
        self.current_state.mouse_position
    }

    pub fn mouse_positions(&self) -> &[(u16, u16)] {
        &self.current_state.mouse_positions
    }
}
