# BoundedQueue

This repository contains the implementation of a thread-safe bounded queue in Rust using `Mutex` and `Condvar` from the standard library.

## Implementation

The queue is implemented using:
- `Mutex<VecDeque<T>>`: protects shared access to the queue
- `Condvar` (not_full, not_empty): blocks and wakes threads efficiently
- No external crates: `std` only

## Given trait Object
```rust
pub trait BoundedQueue<T: Send>: Send + Sync {
    fn new(capacity: usize) -> Self;
    fn push(&self, item: T);        // blocks if full
    fn pop(&self) -> T;             // blocks if empty
    fn try_push(&self, item: T) -> Result<(), T>;  // returns Err if full
    fn try_pop(&self) -> Option<T>;                // returns None if empty
}
```

## Tests
Command:
```bash
cargo test
```

Covers:
- FIFO ordering under contention
- Capacity enforcement
- Blocking behavior
- try_push / try_pop non-blocking behavior
- No deadlock under multiple producers and consumers

Results:
<img width="737" height="317" alt="Screenshot 2026-04-06 at 1 20 27 AM" src="https://github.com/user-attachments/assets/49d2ff1c-7e4c-4c36-8e0b-104c608f12aa" />


## Benchmarks
Command:
```bash
cargo test
```

Measures throughput across:
- `Thread counts`: 1, 2, 4, 8, 16 producer/consumer pairs
<img width="586" height="495" alt="Screenshot 2026-04-06 at 1 19 33 AM" src="https://github.com/user-attachments/assets/4f1b6151-a998-4b3a-ba35-ca511fd865ed" />


- `Queue capacities`: 64, 256, 1024
<img width="586" height="287" alt="Screenshot 2026-04-06 at 1 19 05 AM" src="https://github.com/user-attachments/assets/1a2d389b-bf0b-44ae-ab15-d7aa8e53b49f" />


- `Asymmetric workloads`: 8 producers / 1 consumer and 1 producer / 8 consumers
<img width="586" height="214" alt="Screenshot 2026-04-06 at 1 18 47 AM" src="https://github.com/user-attachments/assets/1af5c347-7258-4c16-a5ac-41ec9460584b" />


`As per the requirement all the tests and the benchmarks are completed and provided.`
