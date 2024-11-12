#![feature(const_float_methods)]

use crossterm::{
    event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    *,
};
use enumset::EnumSet;
use std::{
    io::Write,
    time::{Duration, Instant},
};

mod color;
pub use color::*;
mod vec2;
pub use vec2::*;
mod api;
pub use api::*;
mod sprite;
pub use sprite::*;
mod mouse;
pub use mouse::*;
mod drawing;
mod events;

#[derive(Debug, Clone)]
struct State {
    mouse_position: (u16, u16),
    mouse_positions: Vec<(u16, u16)>,
    mouse_buttons: EnumSet<MouseButton>,

    time: Instant,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_position: (0, 0),
            mouse_positions: Vec::new(),
            mouse_buttons: EnumSet::empty(),

            time: Instant::now(),
        }
    }
}

pub struct Context {
    display_buffer: Vec2<Pixel>,
    drawing_buffer: Vec2<Pixel>,

    previous_state: State,
    current_state: State,
    target_fps: u16,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        execute!(
            std::io::stdout(),
            cursor::Show,
            style::ResetColor,
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture,
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

impl Context {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        execute!(
            std::io::stdout(),
            cursor::Hide,
            terminal::EnterAlternateScreen,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all()),
            event::EnableMouseCapture,
        )
        .unwrap();

        let (width, height) = terminal::size().unwrap();

        let mut display_buffer = Vec2::new(width as usize, height as usize * 2);
        let drawing_buffer = display_buffer.clone();
        display_buffer.fill(Pixel { r: 1, g: 2, b: 3 });

        let mut ctx = Self {
            display_buffer,
            drawing_buffer,

            previous_state: State::default(),
            current_state: State::default(),
            target_fps: u16::MAX,
        };
        ctx.commit_drawing_buffer_to_display();
        ctx
    }

    pub fn width(&self) -> u16 {
        self.display_buffer.width()
    }

    pub fn height(&self) -> u16 {
        self.display_buffer.height() / 2
    }

    pub fn next_frame(&mut self) {
        self.commit_drawing_buffer_to_display();
        self.handle_events();
        self.cap_fps();
    }

    pub fn cap_fps(&mut self) {
        let now = Instant::now();
        let elapsed = now - self.previous_state.time;
        let sleep_time = Duration::from_micros(1_000_000 / self.target_fps as u64);
        if elapsed < sleep_time {
            let sleep_time = sleep_time - elapsed;
            std::thread::sleep(sleep_time);
        }
        self.current_state.time = now;
    }

    pub fn set_target_fps(&mut self, fps: u16) {
        self.target_fps = fps;
    }

    pub fn screen_width(&self) -> u16 {
        self.drawing_buffer.width()
    }

    pub fn screen_height(&self) -> u16 {
        self.drawing_buffer.height()
    }
}
