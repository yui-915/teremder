use crate::{Context, Pixel};
use crossterm::event::{self, Event};
use std::time::Duration;

impl Context {
    pub fn handle_events(&mut self) {
        self.previous_state = self.current_state.clone();
        self.current_state.mouse_positions.clear();
        while event::poll(Duration::from_millis(0)).unwrap() {
            let ev = event::read().unwrap();
            match ev {
                Event::Key(key) => self.handle_keyboard_event(key),
                Event::Mouse(mouse) => self.handle_mouse_event(mouse),
                Event::Resize(width, height) => self.handle_resize_event(width, height),
                _ => {}
            }
        }
        if self.current_state.mouse_positions.is_empty() {
            self.current_state
                .mouse_positions
                .push(self.current_state.mouse_position);
        }
    }

    pub fn handle_resize_event(&mut self, width: u16, height: u16) {
        self.display_buffer.resize_with(
            width as usize,
            height as usize * 2,
            Pixel { r: 1, g: 2, b: 3 },
        );
        self.drawing_buffer.resize_with(
            width as usize,
            height as usize * 2,
            Pixel { r: 0, g: 0, b: 0 },
        );
        self.commit_drawing_buffer_to_display();
    }
}
