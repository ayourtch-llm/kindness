use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BoundedQueue<T> {
    inner: Mutex<VecDeque<T>>,
    capacity: usize,
    not_full: Condvar,
    not_empty: Condvar,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
            capacity,
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    pub fn push(&self, val: T) {
        let mut queue = self.inner.lock().unwrap();
        while queue.len() >= self.capacity {
            queue = self.not_full.wait(queue).unwrap();
        }
        queue.push_back(val);
        self.not_empty.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut queue = self.inner.lock().unwrap();
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        let val = queue.pop_front().unwrap();
        self.not_full.notify_one();
        val
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
}
