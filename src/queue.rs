//! ##Queue
//!
//! Represents a queue of items in list
//! which allows you to handle each in order
//!
use super::queued_type::QueueType;

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> QueueType<T> for Queue<T> {
    /// Create a new queue object
    fn new() -> Self {
        Self { items: vec![] }
    }

    /// Removes all elements in queue
    fn clear(&mut self) {
        self.items = vec![];
    }

    /// Gets length of queue
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Insert an item into the top of the queue
    fn add(&mut self, item: T) {
        self.items.insert(0, item);
    }

    /// Take out an item from the queue
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Gets first item
    fn first(&self) -> Option<&T> {
        self.items.get(self.len() - 1)
    }
}

// Implementing Iter on Queue
impl<T> Iterator for Queue<T> {
    type Item = T;

    /// Gets first item in iter
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
