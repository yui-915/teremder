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
