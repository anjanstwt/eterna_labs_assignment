use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

use crate::queue::BoundedQueue;

pub struct Queue<T> {
    values: Mutex<VecDeque<T>>,
    not_full: Condvar,
    not_empty: Condvar,
    capacity: usize,
}

impl<T: Send> BoundedQueue<T> for Queue<T> {
    fn new(capacity: usize) -> Self {
        Queue {
            values: Mutex::new(VecDeque::with_capacity(capacity)),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
            capacity,
        }
    }

    fn push(&self, item: T) {
        // get the vec
        let mut guard = self.values.lock().unwrap();

        // the queue is full
        while guard.len() == self.capacity {
            guard = self.not_full.wait(guard).unwrap();
        }
        guard.push_back(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let mut guard = self.values.lock().unwrap();

        // the queue is empty
        while guard.is_empty() {
            guard = self.not_empty.wait(guard).unwrap();
        }

        let item = guard.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }

    fn try_push(&self, item: T) -> Result<(), T> {
        let mut guard = self.values.lock().unwrap();

        // queue is full, return the element
        if guard.len() == self.capacity {
            return Err(item);
        }

        guard.push_back(item);
        self.not_empty.notify_one();
        Ok(())
    }

    fn try_pop(&self) -> Option<T> {
        let mut guard = self.values.lock().unwrap();

        // queue is empty, nothing to pop
        if guard.is_empty() {
            return None;
        }

        let item = guard.pop_front().unwrap();
        self.not_full.notify_one();
        Some(item)
    }
}
