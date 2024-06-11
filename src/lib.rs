mod weighted; // Module containing weighted items
mod queues;   // Module containing various queue types

pub use queues::{SortedQueue, Queue}; // Re-exporting Queue and SortedQueue from the queues module
pub use weighted::Weighted;           // Re-exporting Weighted from the weighted module
pub use queues::QueueError;           // Re-exporting QueueError from the queues module
pub use queues::{merge_queues,        // Helper function to help manipulate queues
                merge_sorted_queues,
                queue_to_vec,
                sorted_queue_to_vec,};  
 
#[cfg(test)]
mod tests; // Module containing tests for the queues
