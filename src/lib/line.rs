use derive_more::{Add, AddAssign, Mul, Sub};
use num_traits::Num;

use super::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
pub struct Line<T: Num>(Point<T>, Point<T>);

impl<T: Num> Line<T> {
    pub fn new(a: Point<T>, b: Point<T>) -> Self {
        Self(a, b)
    }
}
