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
                if x < self.width && y < self.height {
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
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        *self.get_mut(x, y) = value;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
