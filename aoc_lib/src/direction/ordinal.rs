use nd_vec::{vector, Vec2};
use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn try_advance<T: Num + Copy + PartialOrd>(&self, pos: Vec2<T>) -> Option<Vec2<T>> {
        Some(match self {
            Self::North => vector!(pos.x(), pos.y() + T::one()),
            Self::NorthEast => vector!(pos.x() + T::one(), pos.y() + T::one()),
            Self::East => vector!(pos.x() + T::one(), pos.y()),
            Self::SouthEast if pos.y() > T::zero() => {
                vector!(pos.x() + T::one(), pos.y() - T::one())
            }
            Self::South if pos.y() > T::zero() => vector!(pos.x(), pos.y() - T::one()),
            Self::SouthWest if pos.x() > T::zero() && pos.y() > T::zero() => {
                vector!(pos.x() - T::one(), pos.y() - T::one())
            }
            Self::West if pos.x() > T::zero() => vector!(pos.x() - T::one(), pos.y()),
            Self::NorthWest if pos.x() > T::zero() => {
                vector!(pos.x() - T::one(), pos.y() + T::one())
            }
            _ => return None,
        })
    }
}
