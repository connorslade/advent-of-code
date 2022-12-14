use derive_more::{Add, AddAssign, Mul, Sub};
use num_traits::{abs, signum, Num, Signed};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
pub struct Point<T: Num = i32> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Signed + Copy> Point<T> {
    pub fn normalize(&self) -> Self {
        Self {
            x: signum(self.x),
            y: signum(self.y),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: abs(self.x),
            y: abs(self.y),
        }
    }
}

impl<T: Num + Ord + Copy> Point<T> {
    pub fn max(&self) -> T {
        self.x.max(self.y)
    }
}
