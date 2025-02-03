use std::collections::HashSet;

use itertools::Itertools;

use super::{grid_errors::GridError, Grid};

impl<T: Clone, const N: usize> Clone for Grid<T, N> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), dimensions: self.dimensions.clone() }
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<T, const N: usize> Grid<T, N> {
    pub fn new(vec: Vec<T>, dimensions: &[usize]) -> Result<Self, GridError> {
        if vec.is_empty() { return Err(GridError::Empty) }

        if dimensions.len() == N {
            if vec.len() != dimensions.iter().fold(1, |acc, n| acc * n) {
                return Err(GridError::UnevenDimensions);
            }
            let dimensions = <[usize; N]>::try_from(dimensions)
                .expect("We have already confirmed that the dimensions are the size of N.");
            Ok(Self { data: vec, dimensions })
        } else if dimensions.len() == N - 1 {
            let dimension_chunk = dimensions.iter()
                .fold(1, |acc, &n| acc * n);
            if vec.len() % dimension_chunk != 0 { return Err(GridError::UnevenDimensions); }
            let last_dim = vec.len() / dimension_chunk;
            let dimensions = dimensions.iter()
                .copied()
                .chain(std::iter::once(last_dim))
                .collect::<Vec<_>>();
            let dimensions = <[usize; N]>::try_from(dimensions)
                .expect("We have already increased the size of dimensions to N.");
            Ok(Self { data: vec, dimensions })
        } else {
            Err(GridError::UnevenDimensions)
        }
    }
}


impl<T> Grid<T, 2> {
    pub fn new2d(vec: Vec<T>, width: usize) -> Result<Self, GridError> {
        if vec.is_empty() { return Err(GridError::Empty) }
        if vec.len() % width != 0 { return Err(GridError::UnevenDimensions); }
        let height = vec.len() / width;
        Ok(Self { data: vec, dimensions: [width, height]  })
    }

    pub fn new2d_with_fn<F>(width: usize, height: usize, f: F) -> Grid<T, 2> 
    where 
        F: Fn(usize) -> T
    {
        let vec = (0..width * height)
            .map(|i| f(i))
            .collect();
        Grid { data: vec, dimensions: [width, height] }
    }
}

impl<T> Grid<T, 3> {
    pub fn new3d(vec: Vec<T>, width: usize, height: usize) -> Result<Self, GridError> {
        if vec.is_empty() { return Err(GridError::Empty) }
        if vec.len() % (width * height) != 0 { return Err(GridError::UnevenDimensions); }
        let depth = vec.len() / (width * height);
        Ok(Self { data: vec, dimensions: [width, height, depth]  })
    }

    pub fn new3d_with_fn<F>(width: usize, height: usize, depth: usize, f: F) -> Grid<T, 3> 
    where 
        F: Fn(usize) -> T
    {
        let vec = (0..width * height * depth)
            .map(|i| f(i))
            .collect();
        Grid { data: vec, dimensions: [width, height, depth] }
    }
}


impl<T> TryFrom<Vec<Vec<T>>> for Grid<T, 2> {
    type Error = GridError;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if value.is_empty() { return Err(GridError::Empty) }
        let width = value[0].len();
        if value.iter().any(|row| row.len() != width) {
            return Err(GridError::UnevenDimensions);
        }
        let vec = value.into_iter()
            .flatten()
            .collect();
        Self::new2d(vec, width)
    }
}

impl TryFrom<&str> for Grid<char, 2> {
    type Error = GridError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() { return Err(GridError::Empty) }
        if value.contains('\r') {return Err(GridError::ContainsCarriageReturns); }
        let mut breaks = vec![-1];
        let crs = value.chars().enumerate()
            .filter(|(_, c)| *c == '\n')
            .map(|(idx, _)| idx as i32);
        breaks.extend(crs);
        let width = *breaks[1..].first().ok_or(GridError::NoLineBreak)? as usize;
        if breaks.iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<HashSet<_>>()
            .len() > 1 {
                return Err(GridError::UnevenLineBreaks);
            }
        let vec: Vec<char> = value.chars()
            .filter(|&c| c != '\n')
            .collect();
        Grid::new2d(vec, width)
    }
}

pub trait GridIterator<T>: Iterator<Item = T> + Sized {
    fn try_collect_grid(self, width: usize) -> Result<Grid<T, 2>, GridError>;
}

impl<T, I> GridIterator<T> for I 
where 
    I: Iterator<Item = T>,
{
    fn try_collect_grid(self, width: usize) -> Result<Grid<T, 2>, GridError> {
        let vec: Vec<T> = self.collect();
        Grid::new2d(vec, width)
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::grid::{Grid, GridError, GridIterator};

    fn cromulent_vec() -> Vec<usize> {
        vec![1, 2, 3, 4, 5, 6]
    }
    
    #[test]
    fn new() {
        let new2d: Result<Grid<usize, 2>, GridError> = Grid::new(cromulent_vec(), &[2, 3]);
        assert_eq!(Ok(Grid { data: cromulent_vec(), dimensions: [2, 3] }), new2d);
        let new3d: Result<Grid<usize, 3>, GridError> = Grid::new(cromulent_vec(), &[2, 1]);
        assert_eq!(Ok(Grid { data: cromulent_vec(), dimensions: [2, 1, 3] }), new3d);
        let big_vec: Vec<_> = ('a'..='x').collect();
        let new4d: Result<Grid<char, 4>, GridError> = Grid::new(big_vec.clone(), &[2, 3, 2]);
        assert_eq!(Ok(Grid { data: big_vec.clone(), dimensions: [2, 3, 2, 2] }), new4d);
        let new4d: Result<Grid<char, 4>, GridError> = Grid::new(big_vec.clone(), &[2, 3, 2, 2]);
        assert_eq!(Ok(Grid { data: big_vec.clone(), dimensions: [2, 3, 2, 2] }), new4d);
    }
    
    
    #[test]
    fn new2d() {
        let new = Grid::new2d(cromulent_vec(), 3);
        assert_eq!(Ok(Grid { data: cromulent_vec(), dimensions: [3, 2] }), new);
    }
    
    #[test]
    fn new2d_uneven() {
        let new = Grid::new2d(cromulent_vec(), 4);
        assert_eq!(Err(GridError::UnevenDimensions), new);
    }
    
    #[test]
    fn new2d_empty() {
        let empty_vec: Vec<usize> = Vec::new();
        let new = Grid::new2d(empty_vec, 4);
        assert_eq!(Err(GridError::Empty), new);
    }
    
    
    #[test]
    fn clone() {
        let new = Grid::new2d(cromulent_vec(), 3).unwrap();
        assert_eq!(new.clone(), new);
    }
    
    #[test]
    fn try_from() {
        let new = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let new = Grid::try_from(new);
        assert_eq!(Grid::new2d(cromulent_vec(), 3), new);
    }
    
    #[test]
    fn try_from_uneven() {
        let new = vec![
            vec![1, 2, 3],
            vec![4, 5, 6, 7],
        ];
        let new = Grid::try_from(new);
        assert_eq!(Err(GridError::UnevenDimensions), new);
    }
    
    #[test]
    fn try_from_str() {
        let new = Grid::try_from("123\n456\n");
        let test = cromulent_vec().into_iter()
            .map(|c| char::from_digit(c as u32, 10).unwrap())
            .collect();
        assert_eq!(Grid::new2d(test, 3), new);
    }
    
    #[test]
    fn try_from_str_no_trailing_cr() {
        let new = Grid::try_from("123\n456");
        let test = cromulent_vec().into_iter()
            .map(|c| char::from_digit(c as u32, 10).unwrap())
            .collect();
        assert_eq!(Grid::new2d(test, 3), new);
    }
    
    #[test]
    fn try_from_str_uneven() {
        let new = Grid::try_from("12\n456\n7890");
        assert_eq!(Err(GridError::UnevenLineBreaks), new);
    }
    
    #[test]
    fn try_collect() {
        let new = [1, 2, 3, 4, 5, 6].into_iter().try_collect_grid(3);
        assert_eq!(Grid::new2d(cromulent_vec(), 3), new);
    }
    
    #[test]
    fn try_collect_uneven() {
        let new = [1, 2, 3, 4, 5, 6].into_iter().try_collect_grid(4);
        assert_eq!(Err(GridError::UnevenDimensions), new);
    }
    
}
