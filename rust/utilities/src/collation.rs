pub trait Collate {
    type Item;
    fn collate(self, threads: usize) -> Vec<Vec<Self::Item>>
    where
        Self: Sized;
}

impl<T, I> Collate for I
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn collate(self, threads: usize) -> Vec<Vec<Self::Item>> {
        let mut partitions: Vec<Vec<T>> = (0..threads).map(|_| Vec::new()).collect();
        for (i, elem) in self.enumerate() {
            partitions[i % threads].push(elem);
        }
        partitions
    }
}