use thiserror::Error;

/// Errors that can occur in a queue.
#[derive(Error, Debug, PartialEq)]
pub enum QueueError {
    /// Indicates that the queue is full.
    #[error("The queue is full try clearing it before filling it")]
    Full,
}

mod reg;    // A regular queue type
mod sorted; // Queue that can sort its items

pub use reg::Queue;
pub use sorted::SortedQueue;

pub fn merge_sorted_queues<T>(q1: SortedQueue<T>, q2: SortedQueue<T>, reverse:bool) -> SortedQueue<T> where T: Ord {
    let mut res_queue = SortedQueue::new(reverse);

    for item in q1 {
        res_queue.add(item);
    };
    for item in q2 {
        res_queue.add(item);
    };

    res_queue
}

pub fn merge_queues<T>(q1: Queue<T>, q2: Queue<T>, reverse: bool) -> Queue<T> {
    let mut res_queue = Queue::new(reverse);

    for item in q1 {
        res_queue.add(item);
    }
    for item in q2 {
        res_queue.add(item);
    }

    res_queue
}

pub fn queue_to_vec<T>(q: Queue<T>) -> Vec<T> {
    let mut vec = vec![];
    for item in q {
        vec.push(item);
    }
    vec
}

pub fn sorted_queue_to_vec<T>(q: SortedQueue<T>) -> Vec<T> where T: Ord {
    let mut vec = vec![];
    for item in q {
        vec.push(item);
    }
    vec
}
