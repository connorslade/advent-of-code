use nd_vec::Vec2;

pub trait IntoTuple2<T> {
    fn into_tuple(self) -> (T, T);
}
impl<T: Copy> IntoTuple2<T> for Vec2<T> {
    fn into_tuple(self) -> (T, T) {
        (self.x(), self.y())
    }
}
