use std::cmp::Ordering;

pub struct Weighted<T> {
    item: T,
    weight: usize,
}

impl<T> Weighted<T> {
    pub fn new(item: T, weight: usize) -> Self {
        Weighted {
            item,
            weight,
        }
    }

    /// Get weight
    pub fn weight(&self) -> usize {
        self.weight
    }

    /// Returns reference to internal value
    pub fn item(&self) -> &T {
        &self.item
    }

    /// Consumes object and returns value
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
