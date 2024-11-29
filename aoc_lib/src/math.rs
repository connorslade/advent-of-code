use num_traits::Num;

pub fn gcd<T: Num + Copy>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

pub fn lcm<T: Num + Copy>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}
