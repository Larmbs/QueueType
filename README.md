# Queued Rust Crate Documentation
## Introduction
Welcome to the documentation for Queued Rust, a crate designed to provide queue data structures for your Rust projects. This crate offers efficient implementations of both standard queues and sorted queues.

## Features
 - [] Queue: A basic FIFO (First-In, First-Out) queue.
 - [] SortedQueue: A queue that maintains elements in sorted order.

## Usage
To use Queued Rust in your project, simply add it to your Cargo.toml file:

```toml
[dependencies]
queued_rust = "0.1.0"
```

## Structs

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
