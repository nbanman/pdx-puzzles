pub trait TryGet<T> {
    fn try_get<U: TryInto<usize>>(&self, n: U) -> Option<&T>;
}

impl<T> TryGet<T> for [T] {
    fn try_get<U: TryInto<usize>>(&self, n: U) -> Option<&T> {
        match n.try_into() { Ok(index) => {
            self.get(index)
        } _ => {
            None
        }}
    }
}