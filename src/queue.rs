//! ##Queue
//! 
//! Represents a queue of items in list
//! which allows you to handle each in order
//! 

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    /// Create a new queue object
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Removes all elements in queue
    pub fn clear(&mut self) {
        self.items = vec![];
    }

    /// Gets length of queue
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Insert an item into the top of the queue
    pub fn add(&mut self, item: T) {
        self.items.insert(0, item);
    }

    /// Take out an item from the queue
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// Implementing Iter on Queue
impl<T> Iterator for Queue<T> where T: Ord {
    type Item = T;

    /// Gets first item in iter
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
