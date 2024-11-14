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
}
