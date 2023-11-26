use std::fmt::Debug;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    pub fn new_filled(width: usize, height: usize, val: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![val; width * height],
            width,
        }
    }

    pub fn raw(&self) -> &Vec<T> {
        &self.data
    }

    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn data(&mut self, data: Vec<T>) {
        self.data = data;
    }

    pub fn set(&mut self, x: usize, y: usize, data: T) {
        self.data[y * self.width + x] = data;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < self.width, "x out of bounds");
        &self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < self.width, "x out of bounds");
        &mut self.data[y * self.width + x]
    }
}
