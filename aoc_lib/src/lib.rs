pub mod line;
pub mod matrix;

pub use line::Line;
pub use matrix::Matrix;
use nd_vec::{Vec2, Vec3};

pub type Point<T> = Vec2<T>;
pub type Point3<T> = Vec3<T>;
