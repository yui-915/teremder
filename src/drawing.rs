use crate::{Color, Context, Pixel};
use crossterm::{cursor, queue, style};
use std::io::Write;

impl Context {
    pub fn commit_drawing_buffer_to_display(&mut self) {
        let mut stdout = std::io::stdout();
        // queue!(stdout, terminal::BeginSynchronizedUpdate).unwrap();
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
                let fg = self.drawing_buffer.get(x, y * 2);
                let bg = self.drawing_buffer.get(x, y * 2 + 1);
                if fg == self.display_buffer.get(x, y * 2)
                    && bg == self.display_buffer.get(x, y * 2 + 1)
                {
                    continue;
                }
                queue!(
                    stdout,
                    cursor::MoveTo(x as u16, y as u16),
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

    pub fn set_pixel(&mut self, x: f32, y: f32, color: Color) {
        // TODO: alpha
        self.drawing_buffer.set(
            x as usize,
            y as usize,
            Pixel {
                r: color.r,
                g: color.g,
                b: color.b,
            },
        );
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        let (x, y, width, height) = (x as usize, y as usize, width as usize, height as usize);
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
                self.set_pixel(x as f32, y as f32, color);
            }
        }
    }

    pub fn clear_background(&mut self, color: Color) {
        self.drawing_buffer.fill(Pixel {
            r: color.r,
            g: color.g,
            b: color.b,
        });
    }

    pub fn fill_circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        for mx in 0..self.drawing_buffer.width() {
            for my in 0..self.drawing_buffer.height() {
                let ox = mx as f32 - x;
                let oy = my as f32 - y;
                let d = (ox * ox + oy * oy).sqrt();
                if d < radius {
                    self.set_pixel(mx as f32, my as f32, color);
                }
            }
        }
    }

    pub fn fill_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        color: Color,
    ) {
        let area = |x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32| {
            ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
        };
        for mx in 0..self.drawing_buffer.width() {
            for my in 0..self.drawing_buffer.height() {
                let x = mx as f32;
                let y = my as f32;
                let a = area(x1, y1, x2, y2, x3, y3);
                let b = area(x, y, x2, y2, x3, y3);
                let c = area(x1, y1, x, y, x3, y3);
                let d = area(x1, y1, x2, y2, x, y);
                if a == b + c + d {
                    self.set_pixel(mx as f32, my as f32, color);
                }
            }
        }
    }
}
