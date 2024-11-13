#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default + Copy> Vec2d<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, T::default())
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.resize_with(width, height, T::default())
    }
}

impl<T: Clone> Vec2d<T> {
    pub fn new_with(width: usize, height: usize, value: T) -> Self {
        Self {
            data: vec![value; width * height],
            width,
            height,
        }
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    pub fn resize_with(&mut self, width: usize, height: usize, value: T) {
        let mut new = Vec2d::new_with(width, height, value.clone());
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
                    new.set(x, y, value.clone());
                }
            }
        }
        *self = new;
    }
}

impl<T> Vec2d<T> {
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
