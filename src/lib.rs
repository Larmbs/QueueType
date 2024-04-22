pub mod queue;
pub mod sorted;

pub use queue::Queue;
pub use sorted::SortedQueue;

#[cfg(test)]
mod tests;
