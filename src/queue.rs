use super::queue_type::QueueType;

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> QueueType<T> for Queue<T> {
    /// Create a new queue object
    fn new() -> Self {
        Self {
            items: vec![],
        }
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
}
