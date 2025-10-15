use std::fmt::{Display, Formatter};
use crate::structs::grid::Grid;

impl Display for Grid<char, 2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        let mut iter = self.iter();
        for _y in 0..self.height() {
            for _x in 0..self.width() {
                display.push(*iter.next().expect("Iterator should never be empty"));
            }
            display.push('\n');
        }
        display.pop();
        write!(f, "{}", display)
    }
}

impl Display for Grid<bool, 2> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        let mut iter = self.iter();
        for _y in 0..self.height() {
            for _x in 0..self.width() {
                let c = if *iter.next().expect("Iterator should never be empty") {
                    '#'
                } else {
                    '.'
                };
                display.push(c);
            }
            display.push('\n');
        }
        display.pop();
        write!(f, "{}", display)
    }
}