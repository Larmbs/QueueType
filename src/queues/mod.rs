use thiserror::Error;

/// Errors that can occur in a queue.
#[derive(Error, Debug)]
pub enum QueueError {
    /// Indicates that the queue is full.
    #[error("The queue is full try clearing it before filling it")]
    Full,
}

mod reg;    // A regular queue type
mod sorted; // Queue that can sort its items

pub use reg::Queue;
pub use sorted::SortedQueue;
