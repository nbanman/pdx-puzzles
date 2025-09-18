use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use itertools::Itertools;
use num_traits::{CheckedAdd, CheckedSub, NumCast, One, PrimInt, Signed, Zero};

use crate::enums::{cardinals::Cardinal, intercardinals::Intercardinal};

pub trait Coordinate: Default + PrimInt + Display + Zero + One + Mul {}

impl<T> Coordinate for T where T: Default + PrimInt + Display + Zero + One + Mul {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<T: Coordinate, const N: usize>(pub [T; N]);

pub type Coord2 = Coord<i64, 2>;
pub type Coord3 = Coord<i64, 3>;
pub type Coord2U = Coord<usize, 2>;
pub type Coord3U = Coord<usize, 3>;

impl<T: Coordinate, const N: usize> Coord<T, N> {
    pub fn new(coordinates: [T; N]) -> Self {
        Coord(coordinates)
    }

    pub fn get_index(&self, dimensions: &[usize]) -> Option<usize> {
        let mut usized = Vec::with_capacity(N);
        for n in self.0 {
            let as_usize = n.to_usize()?;
            usized.push(as_usize);
        }

        let mut multipliers = Vec::with_capacity(N);
        let mut acc = 1;
        multipliers.push(acc);
        for &dim in dimensions {
            acc *= dim;
            multipliers.push(acc);
        }

        let index = usized
            .into_iter()
            .zip(multipliers)
            .map(|(xyz, multiplier)| xyz * multiplier)
            .sum();

        Some(index)
    }

    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&a, &b)| match a.checked_sub(&b) {
                Some(val) => {
                    if val < T::zero() {
                        b - a
                    } else {
                        val
                    }
                }
                None => b - a,
            })
            .reduce(|acc, n| acc + n)
            .unwrap()
            .to_usize()
            .unwrap()
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.0.get(index).cloned()
    }
}

impl<T: Coordinate, const N: usize> Add for Coord<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0usize..N {
            sum[idx] = sum[idx] + rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T: Coordinate, const N: usize> Add<&Coord<T, N>> for Coord<T, N> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0usize..N {
            sum[idx] = sum[idx] + rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T, const N: usize> Add<T> for Coord<T, N>
where
    T: Coordinate + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let mut sum = self.0;
        for idx in 0usize..N {
            sum[idx] = sum[idx] + rhs;
        }
        Self(sum)
    }
}

impl<T, const N: usize> AddAssign for Coord<T, N>
where
    T: Coordinate + AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        for idx in 0usize..N {
            self.0[idx] = self.0[idx] + other.0[idx]
        }
    }
}

impl<T, const N: usize> AddAssign<T> for Coord<T, N>
where
    T: Coordinate + AddAssign,
{
    fn add_assign(&mut self, other: T) {
        for idx in 0usize..N {
            self.0[idx] = self.0[idx] + other
        }
    }
}

impl<T, const N: usize> CheckedAdd for Coord<T, N>
where
    T: Coordinate + CheckedAdd,
{
    fn checked_add(&self, v: &Self) -> Option<Self> {
        let mut sum = self.0;
        for (a, b) in sum.iter_mut().zip(v.0.iter()) {
            *a = a.checked_add(b)?
        }
        Some(Self(sum))
    }
}

impl<T: Coordinate, const N: usize> Sub for Coord<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0usize..N {
            sum[idx] = sum[idx] - rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T, const N: usize> Sub<T> for Coord<T, N>
where
    T: Coordinate + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let mut difference = self.0;
        for idx in 0usize..N {
            difference[idx] = difference[idx] - rhs;
        }
        Self(difference)
    }
}

impl<T, const N: usize> SubAssign for Coord<T, N>
where
    T: Coordinate + SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        for idx in 0usize..N {
            self.0[idx] = self.0[idx] - other.0[idx]
        }
    }
}

impl<T, const N: usize> SubAssign<T> for Coord<T, N>
where
    T: Coordinate + SubAssign,
{
    fn sub_assign(&mut self, other: T) {
        for idx in 0usize..N {
            self.0[idx] = self.0[idx] - other
        }
    }
}

impl<T, const N: usize> CheckedSub for Coord<T, N>
where
    T: Coordinate + CheckedSub,
{
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        let mut diff = self.0;
        for (a, b) in diff.iter_mut().zip(v.0.iter()) {
            *a = a.checked_sub(b)?
        }
        Some(Self(diff))
    }
}

impl<T: Coordinate, const N: usize> Mul for Coord<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0usize..N {
            sum[idx] = sum[idx] * rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T, const N: usize> Mul<T> for Coord<T, N>
where
    T: Coordinate + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut difference = self.0;
        for idx in 0usize..N {
            difference[idx] = difference[idx] * rhs;
        }
        Self(difference)
    }
}

impl<T: Coordinate, const N: usize> Div for Coord<T, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0usize..N {
            sum[idx] = sum[idx] / rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T: Coordinate, const N: usize> Display for Coord<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.0.iter().map(|pos| pos.to_string()).join(", ");
        write!(f, "({})", output)
    }
}

impl<T: Coordinate, const N: usize> PartialOrd for Coord<T, N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl<T: Coordinate, const N: usize> Ord for Coord<T, N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Coordinate + Signed + std::fmt::Debug, const N: usize> Neg for Coord<T, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let iter = self.0.iter().map(|&n| -n);
        let neg_self = <[T; N]>::try_from(iter.collect::<Vec<_>>()).unwrap();
        Self(neg_self)
    }
}

impl<T: Coordinate + Signed> Coord<T, 2> {
    pub fn all_adjacent() -> [Coord<T, 2>; 8] {
        let one = T::one();
        let zero = T::zero();
        let neg = T::neg(one);
        [
            Coord::new2d(neg, neg),
            Coord::new2d(zero, neg),
            Coord::new2d(one, neg),
            Coord::new2d(neg, zero),
            Coord::new2d(one, zero),
            Coord::new2d(neg, one),
            Coord::new2d(zero, one),
            Coord::new2d(one, one),
        ]
    }
}

impl<T: Coordinate> Coord<T, 2> {
    pub fn new2d(x: T, y: T) -> Self {
        let mut contents = [T::default(); 2];
        contents[0] = x;
        contents[1] = y;
        Self(contents)
    }

    pub fn from_index(index: usize, width: usize) -> Option<Self> {
        let x = NumCast::from(index % width)?;
        let y = NumCast::from(index / width)?;
        Some(Self([x, y]))
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn origin() -> Self {
        Self([T::default(); 2])
    }

    pub fn adjacent(&self, diagonals: bool) -> Vec<Self> {
        let capacity = if diagonals { 8 } else { 4 };

        let mut neighbors = Vec::with_capacity(capacity);
        // north
        if let Some(y) = self.y().checked_sub(&T::one()) {
            neighbors.push(Self::new2d(self.x(), y));
        }
        // northeast
        if diagonals {
            if let Some(y) = self.y().checked_sub(&T::one()) {
                if let Some(x) = self.x().checked_add(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        // east
        if let Some(x) = self.x().checked_add(&T::one()) {
            neighbors.push(Self::new2d(x, self.y()));
        }
        // southeast
        if diagonals {
            if let Some(y) = self.y().checked_add(&T::one()) {
                if let Some(x) = self.x().checked_add(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        // south
        if let Some(y) = self.y().checked_add(&T::one()) {
            neighbors.push(Self::new2d(self.x(), y));
        }
        // southwest
        if diagonals {
            if let Some(y) = self.y().checked_add(&T::one()) {
                if let Some(x) = self.x().checked_sub(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        // west
        if let Some(x) = self.x().checked_sub(&T::one()) {
            neighbors.push(Self::new2d(x, self.y()));
        }
        // northwest
        if diagonals {
            if let Some(y) = self.y().checked_sub(&T::one()) {
                if let Some(x) = self.x().checked_sub(&T::one()) {
                    neighbors.push(Self::new2d(x, y));
                }
            }
        }
        neighbors
    }

    pub fn move_direction(&self, dir: Cardinal, distance: T) -> Option<Self> {
        match dir {
            Cardinal::North => Some(Self([self.x(), self.y().checked_sub(&distance)?])),
            Cardinal::East => Some(Self([self.x() + distance, self.y()])),
            Cardinal::South => Some(Self([self.x(), self.y() + distance])),
            Cardinal::West => Some(Self([self.x().checked_sub(&distance)?, self.y()])),
        }
    }

    pub fn move_intercardinal(&self, dir: Intercardinal, distance: T) -> Option<Self> {
        match dir {
            Intercardinal::North => Some(Self([self.x(), self.y().checked_sub(&distance)?])),
            Intercardinal::Northeast => Some(Self([
                self.x() + distance,
                self.y().checked_sub(&distance)?,
            ])),
            Intercardinal::East => Some(Self([self.x() + distance, self.y()])),
            Intercardinal::Southeast => Some(Self([self.x() + distance, self.y() + distance])),
            Intercardinal::South => Some(Self([self.x(), self.y() + distance])),
            Intercardinal::Southwest => Some(Self([
                self.x().checked_sub(&distance)?,
                self.y() + distance,
            ])),
            Intercardinal::West => Some(Self([self.x().checked_sub(&distance)?, self.y()])),
            Intercardinal::Northwest => Some(Self([
                self.x().checked_sub(&distance)?,
                self.y().checked_sub(&distance)?,
            ])),
        }
    }

    pub fn destructured(&self) -> (T, T) {
        (self.0[0], self.0[1])
    }
}

impl<T: Coordinate> From<(T, T)> for Coord<T, 2> {
    fn from(value: (T, T)) -> Self {
        Self::new2d(value.0, value.1)
    }
}

impl<T: Coordinate> Coord<T, 3> {
    pub fn new3d(x: T, y: T, z: T) -> Self {
        let mut contents = [T::default(); 3];
        contents[0] = x;
        contents[1] = y;
        contents[2] = z;
        Self(contents)
    }

    pub fn y(&self) -> T {
        self.0[1]
    }
    pub fn z(&self) -> T {
        self.0[2]
    }

    pub fn origin() -> Self {
        Self([T::default(); 3])
    }
}

impl<T: Coordinate> From<(T, T, T)> for Coord<T, 3> {
    fn from(value: (T, T, T)) -> Self {
        Self::new3d(value.0, value.1, value.2)
    }
}

#[test]
fn unsigned_math_operations() {
    // unsigned 2d
    let pos1 = Coord::new2d(4usize, 7);
    let pos2 = Coord::new2d(3usize, 6);
    assert_eq!(Coord::new2d(7, 13), pos1 + pos2);
    assert_eq!(Coord::new2d(1, 1), pos1 - pos2);
    assert_eq!(None, pos2.checked_sub(&pos1));
    assert_eq!(Some(Coord::new2d(1, 1)), pos1.checked_sub(&pos2));
    assert_eq!(Coord::new2d(12, 42), pos1 * pos2);
    assert_eq!(Coord::new2d(1, 1), pos1 / pos2);
    assert_eq!(2, pos1.manhattan_distance(&pos2));
    // unsigned 3d
    let pos1 = Coord::new3d(4usize, 7, 9);
    let pos2 = Coord::new3d(3usize, 6, 3);
    assert_eq!(Coord::new3d(7, 13, 12), pos1 + pos2);
    assert_eq!(Coord::new3d(1, 1, 6), pos1 - pos2);
    assert_eq!(None, pos2.checked_sub(&pos1));
    assert_eq!(Some(Coord::new3d(1, 1, 6)), pos1.checked_sub(&pos2));
    assert_eq!(Coord::new3d(12, 42, 27), pos1 * pos2);
    assert_eq!(Coord::new3d(1, 1, 3), pos1 / pos2);
    assert_eq!(8, pos1.manhattan_distance(&pos2));
}

#[test]
fn signed_math_operations() {
    // unsigned 2d
    let pos1 = Coord::new2d(-4isize, 7);
    let pos2 = Coord::new2d(3isize, -6);
    assert_eq!(Coord::new2d(-1, 1), pos1 + pos2);
    assert_eq!(Coord::new2d(-7, 13), pos1 - pos2);
    assert_eq!(Some(Coord::new2d(7, -13)), pos2.checked_sub(&pos1));
    assert_eq!(Some(Coord::new2d(-7, 13)), pos1.checked_sub(&pos2));
    assert_eq!(Coord::new2d(-12, -42), pos1 * pos2);
    assert_eq!(Coord::new2d(-1, -1), pos1 / pos2);
    assert_eq!(20, pos1.manhattan_distance(&pos2));
    // unsigned 3d
    let pos1 = Coord::new3d(-4isize, 7, -9);
    let pos2 = Coord::new3d(3isize, -6, 3);
    assert_eq!(Coord::new3d(-1, 1, -6), pos1 + pos2);
    assert_eq!(Coord::new3d(-7, 13, -12), pos1 - pos2);
    assert_eq!(Some(Coord::new3d(7, -13, 12)), pos2.checked_sub(&pos1));
    assert_eq!(Some(Coord::new3d(-7, 13, -12)), pos1.checked_sub(&pos2));
    assert_eq!(Coord::new3d(-12, -42, -27), pos1 * pos2);
    assert_eq!(Coord::new3d(-1, -1, -3), pos1 / pos2);
    assert_eq!(32, pos1.manhattan_distance(&pos2));
}
