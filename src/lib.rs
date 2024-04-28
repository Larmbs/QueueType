mod weighted; // Module containing weighted items
mod queues;   // Module containing various queue types

pub use queues::{SortedQueue, Queue}; // Re-exporting Queue and SortedQueue from the queues module
pub use weighted::Weighted;           // Re-exporting Weighted from the weighted module
pub use queues::QueueError;           // Re-exporting QueueError from the queues module

#[cfg(test)]
mod tests; // Module containing tests for the queues
