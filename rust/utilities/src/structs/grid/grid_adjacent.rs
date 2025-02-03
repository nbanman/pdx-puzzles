use crate::{enums::intercardinals::Intercardinal, structs::coord::Coord2U};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridAdjacent<'a, T> {
    pub index: usize,
    pub pos: Coord2U,
    pub dir: Intercardinal,
    pub value: &'a T,
}

impl<'a, T> GridAdjacent<'a, T> {
    pub fn destruct(&self) -> (usize, Coord2U, Intercardinal, &T) {
        (self.index, self.pos, self.dir, self.value)
    }
}