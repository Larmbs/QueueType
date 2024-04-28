use std::cmp::Ordering;

/// A wrapper around an item with a weight.
pub struct Weighted<T> {
    item: T,
    weight: usize,
}

impl<T> Weighted<T> {
    /// Creates a new `Weighted` instance with the given item and weight.
    pub fn new(item: T, weight: usize) -> Self {
        Weighted {
            item,
            weight,
        }
    }
    /// Returns the weight of the item.
    pub fn weight(&self) -> usize {
        self.weight
    }
    /// Returns a reference to the internal item.
    pub fn item(&self) -> &T {
        &self.item
    }
    /// Consumes the `Weighted` object and returns the item.
    pub fn into_item(self) -> T {
        self.item
    }
}

impl<T> PartialEq for Weighted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T> Eq for Weighted<T> {}

impl<T> PartialOrd for Weighted<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Weighted<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}
