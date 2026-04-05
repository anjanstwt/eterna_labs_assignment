use std::{sync::Arc, thread};

use criterion::{Criterion, criterion_group, criterion_main};
use eterna_labs_assignment::queue::{BoundedQueue, Queue};

fn varying_queue_size_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_size");

    for size in [64, 256, 1024] {
        group.bench_function(format! {"{size}"}, |b| {
            let q: Arc<Queue<i32>> = Arc::new(Queue::<i32>::new(size));
            b.iter(|| {
                let mut handles = Vec::new();
                for _ in 0..8 {
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

criterion_group!(benches, varying_queue_size_bench);
criterion_main!(benches);
