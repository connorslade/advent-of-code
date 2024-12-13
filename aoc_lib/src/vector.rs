use nd_vec::Vec2;

pub trait AsTuple2<T> {
    fn as_tuple(self) -> (T, T);
}
impl<T: Copy> AsTuple2<T> for Vec2<T> {
    fn as_tuple(self) -> (T, T) {
        (self.x(), self.y())
    }
}
