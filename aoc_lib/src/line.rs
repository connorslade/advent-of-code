use nd_vec::Vec2;
use num_traits::Num;

pub struct Line<T: Num>(Vec2<T>, Vec2<T>);

impl<T: Num> Line<T> {
    pub fn new(a: Vec2<T>, b: Vec2<T>) -> Self {
        Self(a, b)
    }
}
