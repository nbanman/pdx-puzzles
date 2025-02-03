use crate::structs::coord::Coord;

use super::StrGrid;

type Pos = Coord<usize, 2>;

pub trait IndexesToGrid: Copy+Clone {
    fn as_grid_idx(&self, grid: &StrGrid) -> usize;
    fn to_self(&self, idx: usize, grid: &StrGrid) -> Self;
}

impl IndexesToGrid for usize {
    fn as_grid_idx(&self, _: &StrGrid) -> usize { *self }

    fn to_self(&self, idx: usize, _: &StrGrid) -> Self { idx }
    
}
impl IndexesToGrid for Pos {
    fn as_grid_idx(&self, grid: &StrGrid) -> usize {
        grid.coord_to_idx(self)
    }

    fn to_self(&self, idx: usize, grid: &StrGrid) -> Self {
        grid.idx_to_coord(&idx)
    }
}