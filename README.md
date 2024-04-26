# Queued Rust Crate Documentation
## Introduction
Welcome to the documentation for Queued Rust, a crate designed to provide queue data structures for your Rust projects. This crate offers efficient implementations of both standard queues and sorted queues.

## Features
 - [] Queue: A basic FIFO (First-In, First-Out) queue.
 - [] SortedQueue: A queue that maintains elements in sorted order.

## Types

### Queue
A basic FIFO (First-In, First-Out) queue.

Example
```rust
use queued_rust::{Queue, QueueType};

fn main() {
    // Creating a regular queue
    let mut queue = Queue::new();

    // Add items to the regular queue
    queue.add(4);
    queue.add(1);
    queue.add(3);
    queue.add(5);
    queue.add(2);

    // Two methods for iterating (.next and .iter)
    println!("Printing items from regular queue");
    // Notice how the items are printed in order of add 4, 1, 3, 5, 2
    while let Some(item) = queue.next() {
        println!("{}. items left: {}", item, queue.len());
    }
}
```

### SortedQueue
A queue that maintains elements in sorted order.

Example
```rust
use queued_rust::{SortedQueue, QueueType};

fn main() {
    // Creating a sorted queue
    let mut sorted_queue = SortedQueue::new();

    // Add items to the sorted queue
    sorted_queue.add(4);
    sorted_queue.add(1);
    sorted_queue.add(3);
    sorted_queue.add(5);
    sorted_queue.add(2);

    // Notice how the items are printed in order 1, 2, 3, 4, 5
    println!("Printing items from sorted queue");
    for item in sorted_queue {
        println!("{}", item)
    }
}
```

## Weighted
A simple wrapper type that allows you to stick a weight number next to any item to easily use the sorted type as it needs the type to implement ordering traits

Example 
```Rust
use queued_rust::{SortedQueue, QueueType, Weighted};

fn main() {
    // Creating a sorted queue with weights
    let mut sorted_weighted_queue = SortedQueue::new();

    // Add items to the sorted queue
    sorted_weighted_queue.add(Weighted::new("hello", 10));
    sorted_weighted_queue.add(Weighted::new("this queue", 4523));
    sorted_weighted_queue.add(Weighted::new("is harder to order", 12412));
    sorted_weighted_queue.add(Weighted::new("properly", 14));
    sorted_weighted_queue.add(Weighted::new("so it might be better", 214));
    sorted_weighted_queue.add(Weighted::new("to add a weight system", 41444));


    println!("Printing items from sorted queue");
    for wrapper in sorted_weighted_queue {
        println!("{}", wrapper.into_item())
    }
}
```

## Future Plans

New objects
- [] Add sized constraint to queues to force size
- [] Create a queue load balancer
- [] Allow an interface for queues with channels 
