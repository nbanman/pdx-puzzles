use std::hash::Hash;

use indexmap::IndexMap;

#[derive(Clone, Default, Debug)]
pub struct Store<K: Hash + Eq, V> {
    id: usize,
    values: IndexMap<K, V>,
}

impl<K: Hash + Eq, V> Store<K, V> {
    pub fn new() -> Self {
        let values = IndexMap::new();
        Self { id: 0, values }
    }

    pub fn get_or_assign_index(&mut self, key: K, value: V) -> usize {
        self.values.get_index_of(&key).unwrap_or_else(|| {
            let value_id = self.id;
            self.id += 1;
            self.values.insert(key, value);
            value_id
        })
    }

    pub fn get_index(&self, key: &K) -> Option<usize> {
        self.values.get_index_of(key)
    }

    pub fn get_index_or_next(&self, key: &K) -> usize {
        self.values.get_index_of(key).unwrap_or(self.id)
    }

    pub fn assign(&mut self, key: K, value: V) -> Option<usize> {
        if self.values.contains_key(&key) {
            None
        } else {
            let value_id = self.id;
            self.id += 1;
            self.values.insert(key, value);
            Some(value_id)
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.values.contains_key(key)
    }

    pub fn contains_index(&self, index: usize) -> bool {
        self.id > index
    }

    pub fn get_entry(&self, index: usize) -> Option<(&K, &V)> {
        self.values.get_index(index)
    }

    pub fn get_entry_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.values.get_index_mut(index)
    }

    pub fn len(&self) -> usize {
        self.id
    }

    pub fn is_empty(&self) -> bool {
        self.id == 0
    }

    pub fn iter(&self) -> indexmap::map::Iter<'_, K, V> {
        self.values.iter()
    }
}

impl<K: Hash + Eq, V> IntoIterator for Store<K, V> {
    type Item = (K, V);

    type IntoIter = <IndexMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[test]
fn basic_functionality() {
    use crate::structs::coord::Coord2;
    let mut store = Store::new();
    let one_one = Coord2::new2d(1, 1);
    let three_three = Coord2::new2d(3, 3);
    assert_eq!(Some(0), store.assign(one_one, three_three));
    assert_eq!(None, store.assign(one_one, three_three));
    assert_eq!(Some(1), store.assign(Coord2::origin(), three_three));
    assert_eq!(true, store.contains(&one_one));
    assert_eq!(false, store.contains(&three_three));
    assert_eq!(Some((&one_one, &three_three)), store.get_entry(0));
    assert_eq!(None, store.get_entry(2));
    assert_eq!(Some(1), store.get_index(&Coord2::origin()));
    assert_eq!(None, store.get_index(&Coord2::new2d(2, 2)));
    assert_eq!(1, store.get_or_assign_index(Coord2::origin(), three_three));
    assert_eq!(2, store.get_or_assign_index(three_three, three_three));
    assert_eq!(3, store.len());
}
