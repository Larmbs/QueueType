use super::queue_type::QueueType;

/// Represents a queue that is sorted 
/// for faster access and better utility
pub struct SortedQueue<T> where T: Ord {
    items: Vec<T>
}

impl<T> QueueType<T> for SortedQueue<T> where T: Ord {
    /// Creates a new queue
    fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    /// Removes all elements in queue
    fn clear(&mut self) {
        self.items = vec![];
    }

    /// Returns length of queue
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Add a items into the sorted queue
    fn add(&mut self, item: T) {
        let mut low = 0;
        let mut high = self.len();
    
        while low < high {
            let mid = (low + high) / 2;
    
            if self.items[mid] < item {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
    
        self.items.insert(low, item);
    }

    /// Take out an item from the queue
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
