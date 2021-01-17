pub trait Walker<T> {
    fn walk(&mut self, _: T) -> T;
}
