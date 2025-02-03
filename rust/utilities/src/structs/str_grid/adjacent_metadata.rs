use crate::enums::cardinals::Cardinal;

use super::IndexesToGrid;

pub struct AdjacentMetadata<T>
where 
    T: IndexesToGrid,
{
    pub pos: T,
    pub dir: Cardinal,
    pub b: u8,
}

impl<T: IndexesToGrid> AdjacentMetadata<T> {
    pub fn destruct(&self) -> (T, Cardinal, u8) {
        (self.pos, self.dir, self.b)
    }
}