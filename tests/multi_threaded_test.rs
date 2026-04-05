use std::{sync::Arc, thread};

use eterna_labs_assignment::queue::{BoundedQueue, Queue};

#[test]
fn multi_prod_multi_con_test() {
    let q = Arc::new(Queue::<i32>::new(2));

    let mut push_handles = Vec::new();
    let mut pop_handles = Vec::new();

    for i in 0..4 {
        let q_clone = Arc::clone(&q);
        let handle = thread::spawn(move || {
            q_clone.push(i);
        });
        push_handles.push(handle);
    }

    for _i in 0..4 {
        let q_clone = Arc::clone(&q);

        // returning the popped value to check at last
        let handle = thread::spawn(move || q_clone.pop());
        pop_handles.push(handle);
    }

    push_handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    let popped = pop_handles.into_iter().map(|handle| handle.join().unwrap());

    assert_eq!(4, popped.len());
}
