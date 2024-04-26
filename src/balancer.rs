use super::queued_type::QueueType;
use std::marker::PhantomData;

struct Balancer<Q, T> where Q: QueueType<T> {
    index: usize,
    queues: Vec<Q>,
    _phantom: PhantomData<T>,
}

impl<Q, T> Balancer<Q, T> where Q: QueueType<T> {
    pub fn new() -> Self {
        Balancer {
            index: 0,
            queues: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn add_queue(&mut self, queue: Q) {
        self.queues.push(queue);
    }

    pub fn add(&mut self, item: T) {
        let queue = self.queues.get(self.index % self.queues.len()).unwrap();
        //queue.add(item);
    }
}   
