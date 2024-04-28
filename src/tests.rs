use super::*;

#[test]
fn it_works() {
    let mut queue = Queue::new(false);
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
    let mut sorted_queue = SortedQueue::new(false);
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

#[test]
fn test_reversal() {
    let mut queue = Queue::new(true);
    queue.add(10);
    queue.add(12);

    assert_eq!(queue.pop(), Some(12));

    let mut sorted = SortedQueue::new(true);
    sorted.add(10);
    sorted.add(18);
    sorted.add(8);

    assert_eq!(sorted.pop(), Some(18));

    let mut queue = Queue::new(false);
    queue.add(10);
    queue.add(12);

    assert_eq!(queue.pop(), Some(10));

    let mut sorted = SortedQueue::new(false);
    sorted.add(10);
    sorted.add(4);    
    sorted.add(19);

    assert_eq!(sorted.pop(), Some(4));
}

#[test]
fn test_sized() {
    let mut queue = Queue::new_sized(1, false);
    assert_eq!(queue.try_add(10), Ok(()));
    assert_eq!(queue.try_add(10), Err(QueueError::Full));

    let mut sorted = SortedQueue::new_sized(1, false);
    assert_eq!(sorted.try_add(10), Ok(()));
    assert_eq!(sorted.try_add(10), Err(QueueError::Full));
}