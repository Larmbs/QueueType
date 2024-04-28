use queued_rust::{Queue, SortedQueue, Weighted};

fn main() {
    // Creating a regular queue
    println!("Testing Out Queues\n");
    let mut queue = Queue::new(false);

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
        println!("Remaining: {} Item: {}", queue.len(), item);
    }

    // Creating a sorted queue
    println!("\n\nTesting Out Sorted Queues\n");
    let mut sorted_queue = SortedQueue::new(false);

    // Add items to the sorted queue
    sorted_queue.add(4);
    sorted_queue.add(1);
    sorted_queue.add(3);
    sorted_queue.add(5);
    sorted_queue.add(2);

    // Notice how the items are printed in order 1, 2, 3, 4, 5
    println!("Printing items from sorted queue");
    while let Some(item) = sorted_queue.next() {
        println!("Remaining: {} Item: {}", sorted_queue.len(), item);
    }

    // Creating a sorted queue with weights
    println!("\n\nTesting Out Sorted Weighted Queues\n");
    let mut sorted_weighted_queue = SortedQueue::new(false);

    // Add items to the sorted queue
    sorted_weighted_queue.add(Weighted::new("hello", 10));
    sorted_weighted_queue.add(Weighted::new("this queue", 4523));
    sorted_weighted_queue.add(Weighted::new("is harder to order", 12412));
    sorted_weighted_queue.add(Weighted::new("properly", 14));
    sorted_weighted_queue.add(Weighted::new("so it might be better", 214));
    sorted_weighted_queue.add(Weighted::new("to add a weight system", 41444));


    println!("Printing items from sorted queue");
    while let Some(item) = sorted_weighted_queue.next() {
        println!("Remaining: {} Item: {}", sorted_weighted_queue.len(), item.into_item());
    }

    // Creating a sized queue
    println!("\n\nTesting Out Sized Queues\n");
    let mut sized_queue = Queue::new_sized(2, false); // Queue with max size of two items

    // These two will work good
    if let Err(error) = sized_queue.try_add(1) {
        eprintln!("{}", error);
    }
    if let Err(error) = sized_queue.try_add(2) {
        eprintln!("{}", error);
    }

    // This will fail
    if let Err(error) = sized_queue.try_add(3) {
        eprintln!("{}", error);
    }

    println!("Printing items from sized queue");
    for item in sized_queue {
        println!("{}", item);
    }



}

