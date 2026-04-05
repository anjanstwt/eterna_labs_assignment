use std::{collections::HashMap, sync::Arc, thread};

use criterion::{Criterion, criterion_group, criterion_main};
use eterna_labs_assignment::queue::{BoundedQueue, Queue};

fn asymmetric_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("workload");

    // this stores in format of (producers, consumers)
    let load = HashMap::from([(8, 1), (1, 8)]);

    for (producers, consumers) in load {
        group.bench_function(
            format!("producers-{producers}, consumers-{consumers}"),
            |b| {
                let q = Arc::new(Queue::<i32>::new(1024));
                b.iter(|| {
                    let mut handles = Vec::new();

                    for _ in 0..producers {
                        let q_clone = Arc::clone(&q);
                        let handle = thread::spawn(move || {
                            for _ in 0..consumers {
                                q_clone.push(1);
                            }
                        });
                        handles.push(handle);
                    }

                    for _ in 0..consumers {
                        let q_clone = Arc::clone(&q);
                        let handle = thread::spawn(move || {
                            q_clone.pop();
                        });
                        handles.push(handle);
                    }
                    handles
                        .into_iter()
                        .for_each(|handle| handle.join().unwrap());

                    while q.try_pop().is_some() {}
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, asymmetric_workload);
criterion_main!(benches);
