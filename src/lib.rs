#![feature(const_float_methods)]

use crossterm::{event::*, *};
use enumset::{EnumSet, EnumSetType};
use std::{
    io::Write,
    time::{Duration, Instant},
};

mod color;
pub use color::*;

mod vec2;
use vec2::Vec2;

mod api;
pub use api::*;

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

    pub fn set_pixel(&mut self, x: u16, y: u16, color: Color) {
        // TODO: alpha
        self.drawing_buffer.set(
            x,
            y,
            Pixel {
                r: color.r,
                g: color.g,
                b: color.b,
            },
        );
    }

    pub fn begin_drawing(&mut self) {}

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

    pub fn commit_drawing_buffer_to_display(&mut self) {
        let mut stdout = std::io::stdout();
        // queue!(stdout, terminal::BeginSynchronizedUpdate).unwrap();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let fg = self.drawing_buffer.get(x, y * 2);
                let bg = self.drawing_buffer.get(x, y * 2 + 1);
                if fg == self.display_buffer.get(x, y * 2)
                    && bg == self.display_buffer.get(x, y * 2 + 1)
                {
                    continue;
                }
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::SetForegroundColor(fg.into()),
                    style::SetBackgroundColor(bg.into()),
                    style::Print('▀'),
                )
                .unwrap();
            }
        }
        // queue!(stdout, terminal::EndSynchronizedUpdate).unwrap();
        self.display_buffer = self.drawing_buffer.clone();
        stdout.flush().unwrap();
    }

    pub fn fill_rect(&mut self, x: u16, y: u16, width: u16, height: u16, color: Color) {
        let x_end = x + width;
        let y_end = y + height;
        for x in x..x_end {
            if x >= self.drawing_buffer.width() {
                break;
            }
            for y in y..y_end {
                if y >= self.drawing_buffer.height() {
                    break;
                }
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn screen_width(&self) -> u16 {
        self.drawing_buffer.width()
    }

    pub fn screen_height(&self) -> u16 {
        self.drawing_buffer.height()
    }

    pub fn clear_background(&mut self, color: Color) {
        self.drawing_buffer.fill(Pixel {
            r: color.r,
            g: color.g,
            b: color.b,
        });
    }

    pub fn handle_events(&mut self) {
        self.previous_state = self.current_state.clone();
        self.current_state.mouse_positions.clear();
        while event::poll(Duration::from_millis(0)).unwrap() {
            let ev = event::read().unwrap();
            match ev {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                        exit_app();
                    }
                }
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
        self.display_buffer.resize(
            width as usize,
            height as usize * 2,
            Pixel { r: 1, g: 2, b: 3 },
        );
        self.drawing_buffer.resize(
            width as usize,
            height as usize * 2,
            Pixel { r: 0, g: 0, b: 0 },
        );
        self.commit_drawing_buffer_to_display();
    }

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
                eprintln!("unhandled mouse event: {:?}", event);
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
