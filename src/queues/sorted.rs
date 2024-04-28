use super::QueueError;

#[derive(Clone, Debug)]
pub struct SortedQueue<T>
where
    T: Ord,
{
    items: Vec<T>,
    max_size: Option<usize>,
}

impl<T> SortedQueue<T>
where
    T: Ord,
{
    /// Creates a queue object with no max size
    pub fn new() -> Self {
        Self { items: vec![], max_size: None }
    }

    /// Creates a queue object with a max size
    pub fn new_sized(size: usize) -> Self {
        Self { items: vec![], max_size: Some(size) }
    }

    /// Removes all elements in queue
    pub fn clear(&mut self) {
        self.items = vec![];
    }

    /// Returns length of queue
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Gets length of queue
    pub fn is_full(&self) -> bool {
        if let Some(max_size) = self.max_size {
            return self.len() >= max_size;
        }
        false
    }

    /// Add a items into the sorted queue
    pub fn add(&mut self, item: T) {
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;

            if self.items[mid] > item { // Flip to switch order
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        self.items.insert(low, item);
    }

    pub fn try_add(&mut self, item: T) -> Result<(), QueueError> {
        // Check if queue is full
        if self.is_full() {return  Err(QueueError::Full);}

        self.add(item);
        Ok(())
    }

    /// Take out an item from the queue
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Gets first item
    pub fn first(&self) -> Option<&T> {
        self.items.last()
    }
}

// Implementing Iter on Queue
impl<T> Iterator for SortedQueue<T> where T: Ord {
    type Item = T;

    /// Gets first item in iter
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
