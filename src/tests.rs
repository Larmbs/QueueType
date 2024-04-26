use crate::queued_type::QueueType;

use super::*;

#[test]
fn it_works() {
    let mut queue = Queue::new();
    queue.add(10);
    queue.add(5);

    assert_eq!(2, queue.len());

    queue.add(4);
    queue.add(2);

    assert_eq!(4, queue.len());

    queue.add(1);

    assert_eq!(5, queue.len());

    queue.pop();

    assert_eq!(4, queue.len());
}

#[test]
fn sorted_queue_test() {
    let mut sorted_queue = SortedQueue::new();
    sorted_queue.add(5);
    sorted_queue.add(6);

    sorted_queue.add(7);
    sorted_queue.add(4);
    sorted_queue.add(9);
    sorted_queue.add(1);
    sorted_queue.add(25);
    sorted_queue.add(2);

    for item in sorted_queue {
        println!("{}", item);
    }
    
    assert!(true);
}
