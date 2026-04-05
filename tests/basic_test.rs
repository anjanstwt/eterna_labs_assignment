use std::{
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};

use eterna_labs_assignment::queue::{BoundedQueue, Queue};

#[test]
fn queue_fifo_test() {
    let q = Queue::<i32>::new(3);

    q.push(1);
    q.push(2);
    q.push(3);

    assert_eq!(1, q.pop());
    assert_eq!(2, q.pop());
    assert_eq!(3, q.pop());
}

#[test]
fn capacity_test() {
    let q = Queue::<i32>::new(3);

    assert!(q.try_push(1).is_ok());
    assert!(q.try_push(2).is_ok());
    assert!(q.try_push(3).is_ok());
    assert!(q.try_push(4).is_err());
}

#[test]
fn blocking_test() {
    let q = Arc::new(Queue::<i32>::new(2));

    let q2 = Arc::clone(&q);
    let q3 = Arc::clone(&q);

    let handle = thread::spawn(move || {
        q2.push(1);
        q2.push(2);
        q2.push(3);
    });

    thread::spawn(move || {
        // sleeping this for the till the 3rd item is reached
        sleep(Duration::from_millis(100));
        let val = q3.pop();
        assert_eq!(1, val);
    });

    handle.join().unwrap();
}

#[test]
fn try_push_and_pop_test() {
    let q = Queue::<i32>::new(2);

    assert!(q.try_push(1).is_ok());
    assert!(q.try_push(2).is_ok());
    assert!(q.try_push(3).is_err());

    assert!(q.try_pop().is_some());
    assert!(q.try_pop().is_some());
    assert!(q.try_pop().is_none());
}
