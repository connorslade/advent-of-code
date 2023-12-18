use nd_vec::{vector, Vec2};
use num_traits::{Num, Signed};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn as_vector<T: Num + Signed + Copy>(&self) -> Vec2<T> {
        match self {
            Self::Up => vector!(T::zero(), -T::one()),
            Self::Down => vector!(T::zero(), T::one()),
            Self::Left => vector!(-T::one(), T::zero()),
            Self::Right => vector!(T::one(), T::zero()),
        }
    }

    #[rustfmt::skip]
    pub fn try_advance<T: Num + Copy + PartialOrd>(&self, pos: Vec2<T>) -> Option<Vec2<T>> {
        Some(match self {
            Self::Up   if pos.y() > T::zero() => vector!(pos.x(), pos.y() - T::one()),
            Self::Down                        => vector!(pos.x(), pos.y() + T::one()),
            Self::Left if pos.x() > T::zero() => vector!(pos.x() - T::one(), pos.y()),
            Self::Right                       => vector!(pos.x() + T::one(), pos.y()),
            _ => return None,
        })
    }

    #[rustfmt::skip]
    pub fn advance<T: Num + Copy>(&self, pos: Vec2<T>) -> Vec2<T> {
        match self {
            Self::Up    => vector!(pos.x(), pos.y() - T::one()),
            Self::Down  => vector!(pos.x(), pos.y() + T::one()),
            Self::Left  => vector!(pos.x() - T::one(), pos.y()),
            Self::Right => vector!(pos.x() + T::one(), pos.y()),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}
