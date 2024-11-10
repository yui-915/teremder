#[derive(Debug, Clone)]
pub(crate) struct Vec2<T> {
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

    pub fn resize(&mut self, width: usize, height: usize, value: T) {
        let mut new = Vec2::new(width, height);
        for x in 0..self.width.max(width) {
            if x >= width {
                break;
            }
            for y in 0..self.height.max(height) {
                if y >= height {
                    break;
                }
                let x = x as u16;
                let y = y as u16;
                if x < self.width as u16 && y < self.height as u16 {
                    new.set(x, y, self.get(x, y).clone());
                } else {
                    new.set(x, y, value);
                }
            }
        }
        *self = new;
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
