use super::Grid;

impl<T, const N: usize> IntoIterator for Grid<T, N> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Grid<T, N> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::grid::Grid;

    fn num_pad() -> Grid<usize, 2> {
        use crate::structs::grid::GridIterator;
        (1..=9).try_collect_grid(3).unwrap()
    }

    #[test]
    fn iter() {
        assert_eq!(
            vec![&1, &2, &3, &4, &5, &6, &7, &8, &9],
            num_pad().iter().collect::<Vec<_>>(),
        );
    }

    #[test]
    fn into_iter() {
        let np = num_pad();
        let no_copy: Vec<_> = (1..=9).map(|n| vec![n]).collect();
        assert_eq!(
            vec![&1, &2, &3, &4, &5, &6, &7, &8, &9],
            (&np).into_iter().collect::<Vec<_>>(),
        );
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            np.into_iter().collect::<Vec<_>>(),
        );
        assert_eq!(
            vec![
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![5],
                vec![6],
                vec![7],
                vec![8],
                vec![9]
            ],
            no_copy.into_iter().collect::<Vec<_>>(),
        )
    }
}
