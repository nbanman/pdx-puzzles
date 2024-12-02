use std::hash::Hash;

use indexmap::IndexSet;

#[derive(Clone, Default, Debug)]
pub struct Indexer<T: Hash + Eq> {
    id: usize,
    values: IndexSet<T>
}

impl <T: Hash + Eq> Indexer<T> {
    pub fn new() -> Self {
        let values = IndexSet::new();
        Self { id: 0, values }
    }

    pub fn get_or_assign_index(&mut self, value: T) -> usize {
        self.values.get_index_of(&value).unwrap_or_else(|| {
            let value_id = self.id;
            self.id += 1;
            self.values.insert(value);
            value_id
        })
    }

    pub fn get_index(&self, value: &T) -> Option<usize> {
        self.values.get_index_of(value)
    }

    pub fn assign(&mut self, value: T) -> Option<usize> {
        if self.values.contains(&value) {
            None
        } else {
            let value_id = self.id;
            self.id += 1;
            self.values.insert(value);
            Some(value_id)
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.values.contains(value)
    }

    pub fn get_value(&self, index: usize) -> Option<&T> {
        self.values.get_index(index)
    }

    pub fn len(&self) -> usize {
        self.id
    }
}

#[test]
fn basic_functionality() {
    use crate::structs::coord::Coord2;
    let mut indexer = Indexer::new();
    let one_one = Coord2::new2d(1, 1);
    let three_three = Coord2::new2d(3, 3);
    assert_eq!(Some(0), indexer.assign(one_one.clone()));
    assert_eq!(None, indexer.assign(one_one.clone()));
    assert_eq!(Some(1), indexer.assign(Coord2::origin()));
    assert_eq!(true, indexer.contains(&one_one));
    assert_eq!(false, indexer.contains(&three_three));
    assert_eq!(Some(&one_one), indexer.get_value(0));
    assert_eq!(None, indexer.get_value(2));
    assert_eq!(Some(1), indexer.get_index(&Coord2::origin()));
    assert_eq!(None, indexer.get_index(&Coord2::new2d(2, 2)));
    assert_eq!(1, indexer.get_or_assign_index(Coord2::origin()));
    assert_eq!(2, indexer.get_or_assign_index(three_three));
    assert_eq!(3, indexer.len());
}