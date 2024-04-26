use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum QueueError {
    #[snafu(display("The queue has a max size of {size}, you tried putting a new item in when it was full."))]
    MaxCapacity {size: usize}
} 

pub trait QueueType<T> {
    fn new() -> Self;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn add(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn first(&self) -> Option<&T>;
}

pub trait SizedQueueType<T> {
    fn new(max_size: usize) -> Self;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn add(&mut self, item: T) -> Result<(), QueueError>;
    fn pop(&mut self) -> Option<T>;
    fn first(&self) -> Option<&T>;
}
