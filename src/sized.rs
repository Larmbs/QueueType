use super::queued_type::{SizedQueueType, QueueError};

pub struct SizedQueue<T> {
    max_size: usize,
    items: Vec<T>,
}

impl<T> SizedQueueType<T> for SizedQueue<T> {
    fn new(max_size: usize) -> Self {
        let items = vec![];
        SizedQueue {
            max_size,
            items,
        }
    }

    fn clear(&mut self) {
        self.items = vec![];
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn add(&mut self, item: T) -> Result<(), QueueError> {
        // Check if size is less than max size
        if self.len() < self.max_size {
            self.items.push(item);
            Ok(())
        } else {
            Err(QueueError::MaxCapacity { size: self.len() })
        }
        
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn first(&self) -> Option<&T> {
        self.items.get(self.len() - 1)
    }
}

// Implementing Iter on Queue
impl<T> Iterator for SizedQueue<T> where T: Ord {
    type Item = T;

    /// Gets first item in iter
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
