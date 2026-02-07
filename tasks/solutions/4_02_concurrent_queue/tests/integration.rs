use solution::*;

#[test]
fn test_basic_push_pop() {
    let q = BoundedQueue::new(10);
    q.push(1);
    q.push(2);
    assert_eq!(q.pop(), 1);
    assert_eq!(q.pop(), 2);
}

#[test]
fn test_len() {
    let q = BoundedQueue::new(5);
    assert_eq!(q.len(), 0);
    q.push(10);
    q.push(20);
    assert_eq!(q.len(), 2);
    q.pop();
    assert_eq!(q.len(), 1);
}

#[test]
fn test_fifo_order() {
    let q = BoundedQueue::new(10);
    for i in 0..5 {
        q.push(i);
    }
    for i in 0..5 {
        assert_eq!(q.pop(), i);
    }
}

#[test]
fn test_concurrent_producers_consumers() {
    use std::sync::Arc;
    use std::thread;

    let q = Arc::new(BoundedQueue::new(4));
    let mut producers = vec![];
    let mut consumers = vec![];

    for i in 0..8 {
        let qc = Arc::clone(&q);
        producers.push(thread::spawn(move || {
            qc.push(i);
        }));
    }

    for _ in 0..8 {
        let qc = Arc::clone(&q);
        consumers.push(thread::spawn(move || {
            qc.pop()
        }));
    }

    for h in producers {
        h.join().unwrap();
    }

    let mut results: Vec<i32> = consumers.into_iter().map(|h| h.join().unwrap()).collect();
    results.sort();
    // All pushed values 0..8 must appear exactly once in popped results
    assert_eq!(results, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_blocking_on_full() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    let q = Arc::new(BoundedQueue::new(2));
    q.push(1);
    q.push(2);
    // Queue is now full

    let qc = Arc::clone(&q);
    let producer = thread::spawn(move || {
        qc.push(3); // should block until space available
    });

    // Give producer time to block
    thread::sleep(Duration::from_millis(50));
    assert_eq!(q.len(), 2); // still full

    let val = q.pop(); // free one slot
    assert_eq!(val, 1);
    producer.join().unwrap();
    assert_eq!(q.len(), 2); // 2 and 3 now in queue
}

#[test]
fn test_blocking_on_empty() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    let q: Arc<BoundedQueue<i32>> = Arc::new(BoundedQueue::new(5));

    let qc = Arc::clone(&q);
    let consumer = thread::spawn(move || {
        qc.pop() // should block until item available
    });

    thread::sleep(Duration::from_millis(50));
    q.push(42);
    let val = consumer.join().unwrap();
    assert_eq!(val, 42);
}
