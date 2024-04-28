use thiserror::Error;

mod reg;    // A regular queue type
mod sorted; // Queue that can sort its items

pub use reg::Queue;
pub use sorted::SortedQueue;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("The queue is full try clearing it before filling it")]
    Full,
}
