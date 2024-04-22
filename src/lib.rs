pub mod queue;
pub mod sorted;
pub mod queue_type;

pub use queue::Queue;
pub use sorted::SortedQueue;
pub use queue_type::QueueType;

#[cfg(test)]
mod tests;
