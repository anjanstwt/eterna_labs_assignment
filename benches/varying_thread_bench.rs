use std::{sync::Arc, thread};

use criterion::{Criterion, criterion_group, criterion_main};
use eterna_labs_assignment::queue::{BoundedQueue, Queue};

fn varying_thread_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_counts");

    for num_of_thread in [1, 2, 4, 8, 16] {
        group.bench_function(format!("{num_of_thread} pairs"), |b| {
            let q = Arc::new(Queue::<i32>::new(1024));
            b.iter(|| {
                let mut handles = Vec::new();

                for _ in 0..num_of_thread {
                    let q_clone = Arc::clone(&q);
                    let handle = thread::spawn(move || {
                        q_clone.push(1);
                        q_clone.pop();
                    });
                    handles.push(handle);
                }

                handles.into_iter().for_each(|handle| handle.join().unwrap());

            });
        });
    }

    group.finish();
}

criterion_group!(benches, varying_thread_bench);
criterion_main!(benches);
