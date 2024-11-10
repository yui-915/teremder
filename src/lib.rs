#![feature(const_float_methods)]

use crossterm::{event::*, *};
use std::{io::Write, time::Duration};

mod color;
pub use color::*;

mod vec2;
use vec2::Vec2;

mod api;
pub use api::*;

pub struct Context {
    display_buffer: Vec2<Pixel>,
    drawing_buffer: Vec2<Pixel>,
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
        )
        .unwrap();

        let (width, height) = terminal::size().unwrap();

        let mut display_buffer = Vec2::new(width as usize, height as usize * 2);
        let drawing_buffer = display_buffer.clone();
        display_buffer.fill(Pixel { r: 1, g: 2, b: 3 });

        let mut ctx = Self {
            display_buffer,
            drawing_buffer,
        };
        ctx.next_frame();
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
        while event::poll(Duration::from_millis(0)).unwrap() {
            let ev = event::read().unwrap();
            match ev {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                        exit_app();
                    }
                }
                _ => {}
            }
        }
    }
}
