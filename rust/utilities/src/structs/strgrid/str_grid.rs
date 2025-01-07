use std::{collections::HashSet, usize};

use itertools::Itertools;

use crate::{enums::cardinals::Cardinal, structs::coord::Coord};

use super::{adjacent_metadata::AdjacentMetadata, indexes_to_grid::IndexesToGrid, str_grid_error::StrGridError};

type Pos = Coord<usize, 2>;

#[derive(Debug, Clone)]
pub struct StrGrid<'a> {
    pub s: &'a [u8],
    pub width: usize,
    pub height: usize,
}

impl<'a> StrGrid<'a> {
    pub fn new(s: &'a str) -> Result<Self, StrGridError> {
        if !s.is_ascii() {
            return Err(StrGridError::IsUTF);
        }
        let s = s.as_bytes();
        if s.contains(&b'\r') {
            return Err(StrGridError::ContainsCarriageReturns);
        }
        let breaks: Vec<usize> = s.iter().enumerate()
            .filter(|(_, &c)| c == b'\n')
            .map(|(idx, _)| idx)
            .collect();
        let width = *breaks.first().ok_or(StrGridError::NoLineBreak)? + 1;
        if breaks.iter().tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<HashSet<_>>()
            .len() != 1 {
                return Err(StrGridError::UnevenWidth);
            }
        let offset = if s[s.len() - 1] == b'\n' { 0 } else { 1 };
        let height = (s.len() + offset) / width;
        Ok(Self { s, width, height })
    }

    pub fn idx_to_coord(&self, idx: &usize) -> Pos {
        Pos::new2d(idx % self.width, idx / self.width)
    }
    
    pub fn coord_to_idx(&self, coord: &Pos) -> usize {
        coord.y() * self.width + coord.x()
    }
    
    pub fn get(&self, idx: impl IndexesToGrid) -> Option<u8> {
        self.get_index(idx.as_grid_idx(self))
    }
    
    pub fn get_index(&self, idx: usize) -> Option<u8> {
        let b = self.s.get(idx)?;
        if *b == b'\n' {
            None
        } else {
            Some(*b)
        }
    }

    pub fn get_coord(&self, coord: Pos) -> Option<u8> {
        let idx = self.coord_to_idx(&coord);
        self.get(idx)
    }
    
    pub fn try_get<T: TryInto<usize>>(&self, idx: T) -> Option<u8> {
        if let Ok(idx) = idx.try_into() {
            self.get(idx)
        } else {
            None
        }
    }
    
    pub fn try_get_coord<T: TryInto<Pos>>(&self, pos: T) -> Option<u8> {
        if let Ok(pos) = pos.try_into() {
            let idx = self.coord_to_idx(&pos);
            self.try_get(idx)
        } else {
            None
        }
        
    }
    
    pub fn adjacent<T>(&'a self, idx: T) -> impl Iterator<Item = AdjacentMetadata<T>> + 'a 
    where 
        T: IndexesToGrid + 'a
    {
        let idx_usize = idx.as_grid_idx(self);
        Cardinal::entries().into_iter()
            .filter_map(move |dir| {
                let a_idx = match dir {
                    Cardinal::North => idx_usize.checked_sub(self.width),
                    Cardinal::East => Some(idx_usize + 1),
                    Cardinal::South => Some(idx_usize + self.width),
                    Cardinal::West => idx_usize.checked_sub(1),
                }?;
                let a_b = self.get(a_idx)?;
                Some(AdjacentMetadata { pos: idx.to_self(a_idx, &self), dir, b: a_b  })
            })
    } 
}

 