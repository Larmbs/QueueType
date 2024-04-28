mod weighted;
mod queues;

pub use queues::{SortedQueue, Queue};  // Module with array of queue types
pub use weighted::Weighted;            // Implements ordering can be used with sorted
pub use queues::QueueError;            // Errors attributed to queues

#[cfg(test)]
mod tests;
