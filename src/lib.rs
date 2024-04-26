mod queue;
mod sorted;
mod queued_type;
mod weighted;

pub use queue::Queue;
pub use sorted::SortedQueue;
pub use queued_type::QueueType;
pub use weighted::Weighted; // Implements ordering can be used with sorted

#[cfg(test)]
mod tests;
