pub trait QueueType<T> {
    fn new() -> Self;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn add(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn first(&self) -> Option<&T>;
}
