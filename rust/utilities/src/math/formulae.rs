use num_traits::PrimInt;

pub fn gcd<N: PrimInt>(mut x: N, mut y: N) -> N {

    while x != N::zero() {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y
}

pub fn lcm<N: PrimInt>(x: N, y: N) -> N {
    x * y / gcd(x, y)
}