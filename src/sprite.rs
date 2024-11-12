use crate::{ctx, Color, Context, Vec2};
use num_traits::AsPrimitive;

pub struct Sprite {
    data: Vec2<Color>,
}

impl Sprite {
    pub fn new(width: u16, height: u16) -> Self {
        let mut data = Vec2::new_with(width as usize, height as usize, Color::rgba(0, 0, 0, 0));
        data.fill(Color::rgba(0, 0, 0, 0));
        Self { data }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, color: Color) {
        self.data.set(x, y, color);
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Color {
        *self.data.get(x, y)
    }

    pub fn width(&self) -> u16 {
        self.data.width()
    }

    pub fn height(&self) -> u16 {
        self.data.height()
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.data
            .resize_with(width as usize, height as usize, Color::rgba(0, 0, 0, 0));
    }

    pub fn draw<X, Y>(&self, x: X, y: Y)
    where
        X: AsPrimitive<u16>,
        Y: AsPrimitive<u16>,
    {
        self.draw_with_ctx(ctx(), x.as_(), y.as_());
    }

    pub fn draw_with_ctx(&self, ctx: &mut Context, x: u16, y: u16) {
        for local_x in 0..self.width() {
            let x = x + local_x;
            if x >= ctx.screen_width() {
                break;
            }
            for local_y in 0..self.height() {
                let y = y + local_y;
                if y >= ctx.screen_height() {
                    break;
                }
                let color = self.get_pixel(local_x, local_y);
                ctx.set_pixel(x, y, color);
            }
        }
    }
}
