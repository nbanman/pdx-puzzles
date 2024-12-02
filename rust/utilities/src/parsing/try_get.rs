pub trait TryGet<T> {
    fn try_get<U: TryInto<usize>>(&self, n: U) -> Option<&T>;
}

impl<T> TryGet<T> for [T] {
    fn try_get<U: TryInto<usize>>(&self, n: U) -> Option<&T> {
        if let Ok(index) = n.try_into() {
            self.get(index)
        } else {
            None
        }
    }
}