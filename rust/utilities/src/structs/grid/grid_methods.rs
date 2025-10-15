use crate::{
    enums::intercardinals::Intercardinal,
    structs::coord::{Coord, Coord2U},
};
use std::iter::Zip;
use std::slice::Iter;
use std::{
    cmp::min,
    ops::{Index, IndexMut},
    slice::IterMut,
};

use super::{Grid, GridAdjacent, GridError, GridIndex};

/// Getters and setters
impl<T, const N: usize> Index<usize> for Grid<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> Index<Coord<usize, N>> for Grid<T, N> {
    type Output = T;

    fn index(&self, index: Coord<usize, N>) -> &Self::Output {
        let index = index
            .get_index(&self.dimensions)
            .expect("Coord get_index only returns None if cannot cast to usize, and here we only supply usize.");
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Grid<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> IndexMut<Coord<usize, N>> for Grid<T, N> {
    fn index_mut(&mut self, index: Coord<usize, N>) -> &mut Self::Output {
        let index = index
            .get_index(&self.dimensions)
            .expect("Coord get_index only returns None if cannot cast to usize, and here we only supply usize.");
        &mut self.data[index]
    }
}

/// General methods
impl<T, const N: usize> Grid<T, N> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn iter_with_coords(
        &self,
    ) -> Zip<impl Iterator<Item = Coord<usize, { N }>> + use<'_, T, N>, Iter<'_, T>> {
        self.coords().zip(self.iter())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.dimensions[0]
    }

    pub fn get(&self, index: impl GridIndex<N>) -> Option<&T> {
        let index = index.as_usize(self)?;
        Some(&self.data[index])
    }

    pub fn get_mut(&mut self, index: impl GridIndex<N>) -> Option<&mut T> {
        let index = index.as_usize(self)?;
        Some(&mut self.data[index])
    }

    pub fn coord_of(&self, index: impl GridIndex<N>) -> Option<Coord<usize, N>> {
        index.as_coord(self)
    }

    pub fn index_of(&self, index: impl GridIndex<N>) -> Option<usize> {
        index.as_usize(self)
    }

    pub fn last_coord(&self) -> Coord<usize, N> {
        (self.data.len() - 1).as_coord(self).expect(
            "Empty backing data should result in GridError, so len() - 1 should always be valid.",
        )
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord<usize, N>> + '_ {
        (0..self.len()).map(|index| {
            index.as_coord(self).expect(
                "Iterates through confirmed indices, so there should always be a valid Coord.",
            )
        })
    }
}

impl<T, const N: usize> Grid<T, N>
where
    T: PartialEq,
{
    fn x_of_element<C, F>(&self, element: &T, get_index: F) -> Option<C>
    where
        C: GridIndex<N>,
        F: Fn(usize) -> Option<C>,
    {
        self.data
            .iter()
            .position(|it| it == element)
            .and_then(|index| get_index(index))
    }

    pub fn coord_of_element(&self, element: &T) -> Option<Coord<usize, N>> {
        self.x_of_element(element, |index| index.as_coord(self))
    }

    pub fn index_of_element(&self, element: &T) -> Option<usize> {
        self.x_of_element(element, |index| index.as_usize(self))
    }

    fn last_x_of_element<C, F>(&self, element: &T, get_index: F) -> Option<C>
    where
        C: GridIndex<N>,
        F: Fn(usize) -> Option<C>,
    {
        self.data
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, it)| it == element)
            .and_then(|(index, _)| get_index(index))
    }

    pub fn last_coord_of_element(&self, element: &T) -> Option<Coord<usize, N>> {
        self.last_x_of_element(element, |index| index.as_coord(self))
    }

    pub fn last_index_of_element(&self, element: &T) -> Option<usize> {
        self.last_x_of_element(element, |index| index.as_usize(self))
    }
}

impl<T, const N: usize> Grid<T, N>
where
    T: Clone,
{
    pub fn sub_grid(
        &self,
        start: Coord<usize, N>,
        size: Coord<usize, N>,
    ) -> Result<Self, GridError> {
        self.index_of(start).ok_or(GridError::OutOfRange)?;
        let adjusted_size: [usize; N] = std::array::from_fn(|n| {
            min(
                self.dimensions[n]
                    .checked_sub(start.0[n])
                    .unwrap_or_default(),
                size.0[n],
            )
        });
        if adjusted_size.iter().any(|dim| *dim == 0) {
            return Err(GridError::OutOfRange);
        }
        let dummy = Grid {
            data: vec![false; self.len()],
            dimensions: adjusted_size,
        };
        let sub = Self::new_with_fn(adjusted_size, |i| {
            self[i.as_coord(&dummy).unwrap() + start].clone()
        });
        Ok(sub)
    }
}
/// 2D Grid methods
impl<T> Grid<T, 2> {
    pub fn height(&self) -> usize {
        self.dimensions[1]
    }

    pub fn to_string<F>(&self, to_char: F) -> String
    where
        F: Fn(&T) -> char,
    {
        let mut output = String::new();

        for row in 0..self.dimensions[1] {
            for col in 0..self.dimensions[0] {
                let pos = Coord([col, row]);
                output.push(to_char(&self[pos]));
            }
            output.push('\n');
        }
        output.pop();
        output
    }

    pub fn row(&self, row_index: usize) -> Result<impl Iterator<Item = &T> + '_, GridError> {
        if row_index >= self.height() {
            return Err(GridError::OutOfRange);
        }
        let start = row_index * self.width();
        Ok(self.data[start..start + self.width()].iter())
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> + '_ {
        (0..self.height()).into_iter().map(|row_index| {
            self.row(row_index)
                .expect("Will never be out of range.")
                .collect()
        })
    }

    pub fn column(&self, column_index: usize) -> Result<impl Iterator<Item = &T> + '_, GridError> {
        if column_index >= self.width() {
            return Err(GridError::OutOfRange);
        }
        let iterator =
            (0..self.height()).map(move |row| &self.data[row * self.width() + column_index]);
        Ok(iterator)
    }

    pub fn columns(&self) -> impl Iterator<Item = Vec<&T>> + '_ {
        (0..self.width()).into_iter().map(|column_index| {
            self.column(column_index)
                .expect("Will never be out of range.")
                .collect()
        })
    }

    pub fn adjacent<I>(
        &self,
        index: I,
        include_intercardinals: bool,
    ) -> Option<impl Iterator<Item = GridAdjacent<'_, T>>>
    where
        I: GridIndex<2>,
    {
        let pos = index.as_coord(self)?;

        let dirs = match include_intercardinals {
            true => Intercardinal::entries().to_vec().into_iter(),
            false => Intercardinal::cardinal_entries().to_vec().into_iter(),
        };

        Some(dirs.filter_map(move |dir| self.single_adjacent(pos, dir)))
    }

    fn single_adjacent(&self, pos: Coord2U, dir: Intercardinal) -> Option<GridAdjacent<'_, T>> {
        let pos = pos.move_intercardinal(dir, 1)?;
        let value = self.get(pos)?;
        let index = pos.as_usize(self)?;

        Some(GridAdjacent {
            index,
            pos,
            dir,
            value,
        })
    }
}

/// 3D Grid methods
impl<T> Grid<T, 3> {
    pub fn height(&self) -> usize {
        self.dimensions[1]
    }

    pub fn depth(&self) -> usize {
        self.dimensions[2]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        enums::intercardinals::Intercardinal,
        structs::{
            coord::{Coord, Coord2U},
            grid::{Grid, GridAdjacent, GridError, GridIterator},
        },
    };

    fn num_pad() -> Grid<usize, 2> {
        (1..=9).try_collect_grid(3).unwrap()
    }

    #[test]
    fn getters() {
        let np = num_pad();
        assert_eq!(3, np[2]);
        assert_eq!(&5, &np[Coord2U::new2d(1, 1)]);
        assert_eq!(Some(&3), np.get(2i32));
        assert_eq!(None, np.get(-2));
        assert_eq!(Some(&5), np.get(Coord2U::new2d(1, 1)));
        assert_eq!(None, np.get(10));
    }

    #[test]
    fn setters() {
        let pos = Coord2U::new2d(1, 1);
        let mut np = num_pad();
        assert_eq!(5, np[4]);
        np[pos] = 9;
        assert_eq!(9, np[4]);
        let should_be_five = np.get_mut(pos).unwrap();
        *should_be_five = 5;
        assert_eq!(5, np[4]);
    }

    #[test]
    fn len() {
        assert_eq!(9, num_pad().len())
    }

    #[test]
    fn to_string() {
        assert_eq!(
            "123\n456\n789".to_string(),
            num_pad().to_string(|&n| (b'0' + n as u8) as char)
        )
    }

    #[test]
    fn iter() {
        assert_eq!(
            vec![&1, &2, &3, &4, &5, &6, &7, &8, &9],
            num_pad().iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn iter_mut() {
        let mut num_pad = num_pad();
        for num in num_pad.iter_mut() {
            *num -= 1;
        }
        let num_pad: Vec<_> = num_pad.iter().collect();
        assert_eq!(vec![&0, &1, &2, &3, &4, &5, &6, &7, &8], num_pad,);
    }

    #[test]
    fn iter_with_coords() {
        assert_eq!(
            vec![
                (Coord2U::new2d(0, 0), &1),
                (Coord2U::new2d(1, 0), &2),
                (Coord2U::new2d(2, 0), &3),
                (Coord2U::new2d(0, 1), &4),
                (Coord2U::new2d(1, 1), &5),
                (Coord2U::new2d(2, 1), &6),
                (Coord2U::new2d(0, 2), &7),
                (Coord2U::new2d(1, 2), &8),
                (Coord2U::new2d(2, 2), &9)
            ],
            num_pad().iter_with_coords().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn rows() {
        let np = num_pad();
        // row()
        assert_eq!(vec![&4, &5, &6], np.row(1).unwrap().collect::<Vec<_>>());
        assert_eq!(
            Err(GridError::OutOfRange),
            np.row(3).map(|row| row.collect::<Vec<_>>()),
        );

        // rows()
        let rows: Vec<_> = np.rows().collect();
        assert_eq!(vec![&4, &5, &6], rows[1],);
    }

    #[test]
    fn columns() {
        let np = num_pad();
        // column()
        assert_eq!(vec![&2, &5, &8], np.column(1).unwrap().collect::<Vec<_>>());
        assert_eq!(
            Err(GridError::OutOfRange),
            np.column(3).map(|row| row.collect::<Vec<_>>()),
        );

        // columns()
        let columns: Vec<_> = np.columns().collect();
        assert_eq!(vec![&2, &5, &8], columns[1],);
    }

    #[test]
    fn width_height_depth() {
        let cube: Vec<char> = ('a'..='x').collect();
        let cube = Grid::new3d(cube, 2, 3).unwrap();
        assert_eq!(2, cube.width());
        assert_eq!(3, cube.height());
        assert_eq!(4, cube.depth());
    }

    #[test]
    fn coord_of() {
        let cube: Vec<char> = ('a'..='x').collect();
        let cube = Grid::new3d(cube, 2, 3).unwrap();
        let index_usize = 17;
        let index_usize_invalid = 26;
        let index_coord = Coord([1, 2, 2]);
        let index_coord_invalid = Coord([2usize, 2, 2]);

        // successes
        assert_eq!(Some(index_coord), cube.coord_of(index_usize));
        assert_eq!(Some(index_coord), cube.coord_of(index_usize as i32));

        // failures
        assert_eq!(None, cube.coord_of(index_usize_invalid));
        assert_eq!(None, cube.coord_of(index_coord_invalid));
    }

    #[test]
    fn index_of() {
        let cube: Vec<char> = ('a'..='x').collect();
        let cube = Grid::new3d(cube, 2, 3).unwrap();
        let index_usize = 17;
        let index_usize_invalid = 26;
        let index_coord = Coord([1usize, 2, 2]);
        let index_coord_invalid = Coord([2usize, 2, 2]);

        //successes
        assert_eq!(Some(index_usize), cube.index_of(index_coord));
        assert_eq!(Some(index_usize), cube.index_of(Coord([1i32, 2, 2])));

        // failures
        assert_eq!(None, cube.index_of(index_usize_invalid));
        assert_eq!(None, cube.index_of(index_coord_invalid));
    }

    #[test]
    fn of_element() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1];
        let test = Grid::new2d(vec, 3).unwrap();

        //successes
        assert_eq!(Some(Coord2U::new2d(1, 0)), test.coord_of_element(&1));
        assert_eq!(Some(Coord2U::new2d(2, 1)), test.coord_of_element(&5));
        assert_eq!(Some(1), test.index_of_element(&1));
        assert_eq!(Some(5), test.index_of_element(&5));

        // failures
        assert_eq!(None, test.coord_of_element(&-1));
        assert_eq!(None, test.index_of_element(&-1));
    }

    #[test]
    fn last_of_element() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1];
        let test = Grid::new2d(vec, 3).unwrap();

        //successes
        assert_eq!(Some(Coord2U::new2d(2, 3)), test.last_coord_of_element(&1));
        assert_eq!(Some(Coord2U::new2d(2, 1)), test.last_coord_of_element(&5));
        assert_eq!(Some(11), test.last_index_of_element(&1));
        assert_eq!(Some(5), test.last_index_of_element(&5));

        // failures
        assert_eq!(None, test.last_coord_of_element(&-1));
        assert_eq!(None, test.last_index_of_element(&-1));
    }

    #[test]
    fn coords() {
        let cube: Vec<char> = ('a'..='x').collect();
        let cube = Grid::new3d(cube, 2, 3).unwrap();
        let cube_coords: Vec<_> = cube.coords().collect();
        assert_eq!(24, cube.coords().collect::<Vec<_>>().len());
        assert_eq!(Coord::new3d(0, 0, 0), cube_coords[0]);
        assert_eq!(Coord::new3d(1, 2, 3), cube_coords[cube_coords.len() - 1]);
    }

    #[test]
    fn adjacent() {
        let np = num_pad();

        // top left, diagonals
        let adjacent: Vec<_> = np.adjacent(0, true).unwrap().collect();
        let test = vec![
            GridAdjacent {
                index: 1,
                pos: Coord([1, 0]),
                dir: Intercardinal::East,
                value: &2,
            },
            GridAdjacent {
                index: 4,
                pos: Coord([1, 1]),
                dir: Intercardinal::Southeast,
                value: &5,
            },
            GridAdjacent {
                index: 3,
                pos: Coord([0, 1]),
                dir: Intercardinal::South,
                value: &4,
            },
        ];
        assert_eq!(test, adjacent);

        // middle, diagonals
        let adjacent: Vec<_> = np.adjacent(Coord2U::new2d(1, 1), true).unwrap().collect();
        let test = vec![
            GridAdjacent {
                index: 0,
                pos: Coord([0, 0]),
                dir: Intercardinal::Northwest,
                value: &1,
            },
            GridAdjacent {
                index: 1,
                pos: Coord([1, 0]),
                dir: Intercardinal::North,
                value: &2,
            },
            GridAdjacent {
                index: 2,
                pos: Coord([2, 0]),
                dir: Intercardinal::Northeast,
                value: &3,
            },
            GridAdjacent {
                index: 5,
                pos: Coord([2, 1]),
                dir: Intercardinal::East,
                value: &6,
            },
            GridAdjacent {
                index: 8,
                pos: Coord([2, 2]),
                dir: Intercardinal::Southeast,
                value: &9,
            },
            GridAdjacent {
                index: 7,
                pos: Coord([1, 2]),
                dir: Intercardinal::South,
                value: &8,
            },
            GridAdjacent {
                index: 6,
                pos: Coord([0, 2]),
                dir: Intercardinal::Southwest,
                value: &7,
            },
            GridAdjacent {
                index: 3,
                pos: Coord([0, 1]),
                dir: Intercardinal::West,
                value: &4,
            },
        ];
        assert_eq!(test, adjacent);

        // side, no diagonals
        let adjacent: Vec<_> = np.adjacent(5, false).unwrap().collect();
        let test = vec![
            GridAdjacent {
                index: 2,
                pos: Coord([2, 0]),
                dir: Intercardinal::North,
                value: &3,
            },
            GridAdjacent {
                index: 8,
                pos: Coord([2, 2]),
                dir: Intercardinal::South,
                value: &9,
            },
            GridAdjacent {
                index: 4,
                pos: Coord([1, 1]),
                dir: Intercardinal::West,
                value: &5,
            },
        ];
        assert_eq!(test, adjacent);
    }
}
