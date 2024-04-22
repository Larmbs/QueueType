
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