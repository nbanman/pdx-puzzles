use num_traits::{NumCast, PrimInt};

use crate::structs::coord::{Coord, Coordinate};

use super::Grid;

pub trait GridIndex<const N: usize>: Copy + Clone {
    fn as_usize<T>(&self, grid: &Grid<T, N>) -> Option<usize>;

    fn as_coord<T>(&self, grid: &Grid<T, N>) -> Option<Coord<usize, N>>;
}

macro_rules! impl_grid_index {
    ($($t:ty),*) => {
        $(
            impl<const N: usize> GridIndex<N> for $t {
                fn as_usize<T>(&self, grid: &Grid<T, N>) -> Option<usize> {
                    let usize_index = NumCast::from(*self)?;
                    if usize_index >= grid.len() { return None; }
                    Some(usize_index)
                }

                fn as_coord<T>(&self, grid: &Grid<T, N>) -> Option<Coord<usize, N>> {
                    let mut usize_index: usize = NumCast::from(*self)?;
                    if usize_index >= grid.len() { return None; }
                    let mut dimensions: Vec<usize> = multipliers(&grid.dimensions)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .map(|offset| {
                            let dim = usize_index / offset;
                            usize_index -= dim * offset;
                            dim
                        })
                        .collect();
                    dimensions.reverse();
                    let dimensions = <[usize; N]>::try_from(dimensions).unwrap();
                    Some(Coord(dimensions))
                }
            }
        )*
    }
}

impl_grid_index!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<I, const N: usize> GridIndex<N> for [I; N]
where
    I: PrimInt,
{
    fn as_usize<T>(&self, grid: &Grid<T, N>) -> Option<usize> {
        let dimensions: [Option<usize>; N] = self.map(|n| NumCast::from(n));
        if dimensions.iter().any(|it| it.is_none()) {
            return None;
        }
        let dimensions = dimensions.map(|dim| dim.unwrap());

        if dimensions
            .iter()
            .zip(grid.dimensions.iter())
            .any(|(pos_dim, grid_dim)| pos_dim >= grid_dim)
        {
            return None;
        }

        let usize_index = multipliers(&grid.dimensions)
            .zip(dimensions.into_iter())
            .map(|(mult, dim)| mult * dim)
            .sum();

        Some(usize_index)
    }

    fn as_coord<T>(&self, grid: &Grid<T, N>) -> Option<Coord<usize, N>> {
        let dimensions: [Option<usize>; N] = self.map(|n| NumCast::from(n));

        if dimensions.iter().any(|it| it.is_none()) {
            return None;
        }

        let dimensions = dimensions.map(|n| n.unwrap());

        if dimensions
            .iter()
            .zip(grid.dimensions.iter())
            .any(|(pos_dim, grid_dim)| pos_dim >= grid_dim)
        {
            return None;
        }

        Some(Coord(dimensions))
    }
}

impl<I, const N: usize> GridIndex<N> for Coord<I, N>
where
    I: Coordinate,
{
    fn as_usize<T>(&self, grid: &Grid<T, N>) -> Option<usize> {
        self.0.as_usize(grid)
    }

    fn as_coord<T>(&self, grid: &Grid<T, N>) -> Option<Coord<usize, N>> {
        self.0.as_coord(grid)
    }
}

/// Helper functions
fn multipliers<'a>(dimensions: &'a [usize]) -> impl Iterator<Item = usize> + 'a {
    let multipliers = dimensions[0..dimensions.len() - 1]
        .iter()
        .scan(1, |state, &n| {
            *state *= n;
            Some(*state)
        });
    std::iter::once(1).chain(multipliers)
}
