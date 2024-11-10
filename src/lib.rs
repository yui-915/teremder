#![feature(const_float_methods)]

use std::{io::Write, time::Duration};

use crossterm::{event::*, *};
use num_traits::{AsPrimitive, FromPrimitive};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<&Pixel> for style::Color {
    fn from(p: &Pixel) -> Self {
        style::Color::Rgb {
            r: p.r,
            g: p.g,
            b: p.b,
        }
    }
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
            a: (a.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub const fn rgb_f(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
            a: 255,
        }
    }

    pub const fn hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    pub const fn hex_a(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }
}

pub const RED: Color = Color::rgb(255, 0, 0);
pub const GREEN: Color = Color::rgb(0, 255, 0);
pub const BLUE: Color = Color::rgb(0, 0, 255);
pub const BLACK: Color = Color::rgb(0, 0, 0);
pub const WHITE: Color = Color::rgb(255, 255, 255);

#[derive(Debug, Clone)]
struct Vec2<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default + Copy> Vec2<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl<T: Clone> Vec2<T> {
    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

impl<T> Vec2<T> {
    pub fn get(&self, x: u16, y: u16) -> &T {
        &self.data[y as usize * self.width + x as usize]
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut T {
        &mut self.data[y as usize * self.width + x as usize]
    }

    pub fn set(&mut self, x: u16, y: u16, value: T) {
        *self.get_mut(x, y) = value;
    }

    pub fn width(&self) -> u16 {
        self.width as u16
    }

    pub fn height(&self) -> u16 {
        self.height as u16
    }
}

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

static mut CONTEXT: Option<Context> = None;
pub fn ctx() -> &'static mut Context {
    unsafe {
        #[allow(static_mut_refs)]
        CONTEXT.get_or_insert_with(Context::new)
    }
}

macro_rules! numify {
    ($($i:ident),*) => {
        $(let $i = $i.as_();)*
    };
}

pub fn set_pixel<N>(x: N, y: N, color: Color)
where
    N: AsPrimitive<u16>,
{
    numify!(x, y);
    ctx().set_pixel(x, y, color);
}

pub fn begin_drawing() {
    ctx().begin_drawing();
}

pub fn next_frame() {
    ctx().next_frame();
}

pub fn fill_rect<X, Y, W, H>(x: X, y: Y, width: W, height: H, color: Color)
where
    X: AsPrimitive<u16>,
    Y: AsPrimitive<u16>,
    W: AsPrimitive<u16>,
    H: AsPrimitive<u16>,
{
    numify!(x, y, width, height);
    ctx().fill_rect(x, y, width, height, color);
}

pub fn screen_width<T>() -> T
where
    T: FromPrimitive,
{
    T::from_u16(ctx().screen_width()).unwrap()
}

pub fn screen_height<T>() -> T
where
    T: FromPrimitive,
{
    T::from_u16(ctx().screen_height()).unwrap()
}

pub fn clear_background(color: Color) {
    ctx().clear_background(color);
}

pub fn exit_app() {
    unsafe {
        #[allow(static_mut_refs)]
        CONTEXT.take(); // drop if any
    }
    std::process::exit(0);
}
