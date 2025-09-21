use advent_ocr::Scannable;
use crate::structs::grid::Grid;

impl Scannable for Grid<bool, 2> {
    fn normalize(&self) -> String {
        let mut image = String::new();
        for row in self.rows() {
            let row_iter = row.iter().map(|&&on| if on { '#' } else { '.' });
            image.extend(row_iter);
            image.push('\n');
        }
        image
    }
}

impl Scannable for Grid<char, 2> {
    fn normalize(&self) -> String {
        let mut image = String::new();
        for row in self.rows() {
            image.extend(row.iter().copied());
            image.push('\n');
        }
        image
    }
}