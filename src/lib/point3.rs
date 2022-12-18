use derive_more::{Add, AddAssign, Mul, Sub};
use num_traits::{abs, signum, Num, Signed};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
pub struct Point3<T: Num = i32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Point3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Num + Signed + Copy> Point3<T> {
    pub fn normalize(&self) -> Self {
        Self {
            x: signum(self.x),
            y: signum(self.y),
            z: signum(self.z),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: abs(self.x),
            y: abs(self.y),
            z: abs(self.z),
        }
    }
}

impl<T: Num + Ord + Copy> Point3<T> {
    pub fn max_value(&self) -> T {
        self.x.max(self.y).max(self.z)
    }

    pub fn min_value(&self) -> T {
        self.x.min(self.y).min(self.z)
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }
}
