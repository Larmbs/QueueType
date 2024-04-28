use super::QueueError;

/// A sorted queue implementation.
#[derive(Clone, Debug)]
pub struct SortedQueue<T>
where
    T: Ord,
{
    items: Vec<T>,
    max_size: Option<usize>,
    reverse: bool,
}

impl<T> SortedQueue<T>
where
    T: Ord,
{
    /// Creates a new `SortedQueue` with no maximum size.
    pub fn new(reverse: bool) -> Self {
        Self { items: vec![], max_size: None, reverse }
    }
    /// Creates a new `SortedQueue` with a specified maximum size.
    pub fn new_sized(size: usize, reverse: bool) -> Self {
        Self { items: vec![], max_size: Some(size), reverse }
    }

    /// Removes all elements from the queue.
    pub fn clear(&mut self) {
        self.items = vec![];
    }
    /// Returns the length of the queue.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Checks if the queue is full.
    pub fn is_full(&self) -> bool {
        if let Some(max_size) = self.max_size {
            return self.len() >= max_size;
        }
        false
    }
    /// Returns queue reversed state
    pub fn is_reversed(&self) -> bool {
        self.reverse
    }

    /// Adds an item to the sorted queue.
    ///
    /// Items are inserted such that the queue remains sorted in ascending order.
    pub fn add(&mut self, item: T) {
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;


            if (self.items[mid] > item)^self.reverse { // Flip to switch order
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        self.items.insert(low, item);
    }

    /// Attempts to add an item to the queue.
    ///
    /// If the queue is full, returns an error.
    pub fn try_add(&mut self, item: T) -> Result<(), QueueError> {
        if self.is_full() {
            Err(QueueError::Full)
        } else {
            self.add(item);
            Ok(())
        }
    }

    /// Removes and returns an item from the queue.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Returns a reference to the first item in the queue.
    pub fn first(&self) -> Option<&T> {
        self.items.last()
    }
}

impl<T> Iterator for SortedQueue<T> where T: Ord {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
