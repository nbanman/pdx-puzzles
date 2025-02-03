use super::Grid;

enum GridRotation {
    Left,
    Right,
    OneEighty,
    FlipX,
    FlipY,
}

impl<T: Clone> Grid<T, 2> {
    fn rotate(&self, rotation: GridRotation) -> Grid<T, 2> 
    {
        let mut rotated = Vec::with_capacity(self.len());
        let width = self.width();
        let height = self.height();
        
        match rotation {
            GridRotation::Left => {
                for x in (0..width).rev() {
                    for y in 0..height {
                        rotated.push(self.data[y * width + x].clone());
                    }
                }
            },
            GridRotation::Right => {
                for x in 0..width {
                    for y in (0..height).rev() {
                        rotated.push(self.data[y * width + x].clone());
                    }
                }
            },
            GridRotation::OneEighty => {
                for y in (0..height).rev() {
                    for x in (0..width).rev() {
                        rotated.push(self.data[y * width + x].clone());
                    }
                }
            },
            GridRotation::FlipX => {
                for y in 0..height {
                    for x in (0..width).rev() {
                        rotated.push(self.data[y * width + x].clone());
                    }
                }
            },
            GridRotation::FlipY => {
                for y in (0..height).rev() {
                    for x in 0..width {
                        rotated.push(self.data[y * width + x].clone());
                    }
                }
            },
        }

        let dimensions: [usize; 2] = match rotation {
            GridRotation::Left | GridRotation::Right => [self.dimensions[1], self.dimensions[0]],
            _ => self.dimensions,
        };
        
        Grid { data: rotated, dimensions }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::grid::{Grid, GridIterator};

    fn num_pad() -> Grid<usize, 2> {
        (1..=8).try_collect_grid(4).unwrap()
    }

    #[test]
    fn rotate_right() {
        let np = num_pad().rotate(super::GridRotation::Right);
        let test: Grid<usize, 2> = Grid::new(vec![5, 1, 6, 2, 7, 3, 8, 4], &[2]).unwrap();
        assert_eq!(np, test)
    }

    #[test]
    fn rotate_left() {
        let np = num_pad().rotate(super::GridRotation::Left);
        let test: Grid<usize, 2> = Grid::new(vec![4, 8, 3, 7, 2, 6, 1, 5], &[2]).unwrap();
        assert_eq!(np, test)
    }

    #[test]
    fn rotate_180() {
        let np = num_pad().rotate(super::GridRotation::OneEighty);
        let test: Grid<usize, 2> = Grid::new(vec![8, 7, 6, 5, 4, 3, 2, 1], &[4]).unwrap();
        assert_eq!(np, test)
    }

    #[test]
    fn flip_x() {
        let np = num_pad().rotate(super::GridRotation::FlipX);
        let test: Grid<usize, 2> = Grid::new(vec![4, 3, 2, 1, 8, 7, 6, 5], &[4]).unwrap();
        assert_eq!(np, test)
    }

    #[test]
    fn flip_y() {
        let np = num_pad().rotate(super::GridRotation::FlipY);
        let test: Grid<usize, 2> = Grid::new(vec![5, 6, 7, 8, 1, 2, 3, 4], &[4]).unwrap();
        assert_eq!(np, test)
    }
}