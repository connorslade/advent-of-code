use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
};

use nd_vec::{vector, Vec2};
use num_traits::{Num, ToPrimitive};

pub struct Grid<T> {
    pub data: Vec<T>,
    pub size: Vec2<usize>,
}

impl<T> Grid<T> {
    pub fn new(size: Vec2<usize>, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; size.x() * size.y()],
            size,
        }
    }

    pub fn parse(input: &str, parse: fn(char) -> T) -> Self {
        let mut data = Vec::with_capacity(input.len());
        let mut size = vector!(0, 0);

        for line in input.lines() {
            size = vector!(line.len(), size.y() + 1);
            for c in line.chars() {
                data.push(parse(c));
            }
        }

        Self { data, size }
    }

    pub fn contains<K: Num + ToPrimitive + Copy + PartialOrd>(&self, pos: Vec2<K>) -> bool {
        if pos.x() < K::zero() || pos.y() < K::zero() {
            return false;
        }

        let pos = pos.num_cast::<usize>().unwrap();
        pos.x() < self.size.x() && pos.y() < self.size.y()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2<usize>, &T)> {
        (0..self.data.len()).map(|x| {
            let pos = vector!(x % self.size.x(), x / self.size.x());
            let data = self.get(pos).unwrap();
            (pos, data)
        })
    }

    pub fn get(&self, pos: Vec2<usize>) -> Option<&T> {
        (pos.x() < self.size.x() && pos.y() < self.size.y())
            .then(|| &self.data[pos.y() * self.size.x() + pos.x()])
    }

    pub fn get_mut(&mut self, pos: Vec2<usize>) -> Option<&mut T> {
        (pos.x() < self.size.x() && pos.y() < self.size.y())
            .then(|| &mut self.data[pos.y() * self.size.x() + pos.x()])
    }

    pub fn set<K: ToPrimitive + Copy>(&mut self, pos: Vec2<K>, value: T) {
        let pos = pos.num_cast::<usize>().unwrap();
        self.data[pos.y() * self.size.x() + pos.x()] = value;
    }

    pub fn size(&self) -> Vec2<usize> {
        self.size
    }

    pub fn find(&self, value: T) -> Option<Vec2<usize>>
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .position(|x| x == &value)
            .map(|x| vector!(x % self.size.x(), x / self.size.x()))
    }
}

impl<T: Clone> Grid<T> {
    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[1] * self.size.x() + index[0]]
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
        &mut self.data[index[1] * self.size.x() + index[0]]
    }
}

impl<T, K: ToPrimitive + Copy> Index<Vec2<K>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2<K>) -> &Self::Output {
        let index = index.num_cast::<usize>().unwrap();
        &self.data[index.y() * self.size.x() + index.x()]
    }
}

impl<T, K: ToPrimitive + Copy> IndexMut<Vec2<K>> for Grid<T> {
    fn index_mut(&mut self, index: Vec2<K>) -> &mut T {
        let index = index.num_cast::<usize>().unwrap();
        &mut self.data[index.y() * self.size.x() + index.x()]
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            size: self.size,
        }
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}

impl<T: Eq> Eq for Grid<T> {}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.size.hash(state);
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y() {
            for x in 0..self.size.x() {
                write!(f, "{:?} ", self[[x, y]])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
