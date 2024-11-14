use crate::{ctx, Color, Context, Vec2d};

pub struct Sprite {
    data: Vec2d<Color>,
}

impl Sprite {
    pub fn new(width: f32, height: f32) -> Self {
        let mut data = Vec2d::new_with(width as usize, height as usize, Color::rgba(0, 0, 0, 0));
        data.fill(Color::rgba(0, 0, 0, 0));
        Self { data }
    }

    pub fn set_pixel(&mut self, x: f32, y: f32, color: Color) {
        self.data.set(x as usize, y as usize, color);
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> Color {
        *self.data.get(x as usize, y as usize)
    }

    pub fn width(&self) -> f32 {
        self.data.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.data.height() as f32
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.data
            .resize_with(width as usize, height as usize, Color::rgba(0, 0, 0, 0));
    }

    pub fn draw(&self, x: f32, y: f32) {
        self.draw_with_ctx(ctx(), x, y);
    }

    pub fn draw_with_ctx(&self, ctx: &mut Context, x: f32, y: f32) {
        for local_x in 0..self.width() as usize {
            let local_x = local_x as f32;
            let x = x + local_x;
            if x >= ctx.screen_width() {
                break;
            }
            for local_y in 0..self.height() as usize {
                let local_y = local_y as f32;
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
