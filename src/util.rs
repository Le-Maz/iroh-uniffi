pub trait Apply<T> {
    fn apply(&mut self, f: impl FnOnce(T) -> T);
}

impl<T> Apply<T> for Option<T> {
    fn apply(&mut self, f: impl FnOnce(T) -> T) {
        *self = self.take().map(f);
    }
}
