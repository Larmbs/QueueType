use super::QueueError;

#[derive(Clone, Debug)]
pub struct Queue<T> {
    items: Vec<T>,
    max_size: Option<usize>,
    reverse: bool,
}

impl<T> Queue<T> {
    /// Creates a new `Queue` with no maximum size.
    pub fn new(reverse: bool) -> Self {
        Self { items: vec![], max_size: None, reverse }
    }
    /// Creates a new `Queue` with a specified maximum size.
    pub fn new_sized(size: usize, reverse: bool) -> Self {
        Self { items: vec![], max_size: Some(size), reverse }
    }

    /// Removes all elements in queue
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

    /// Adds an item to the queue.
    pub fn add(&mut self, item: T) {
        if self.reverse {
            self.items.push(item);
        } else {
            self.items.insert(0, item);
        }
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

impl<T> Iterator for Queue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
