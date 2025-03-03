pub trait Container<T> {
    fn put(&mut self, item: T);
    fn get(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}
