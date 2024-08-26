# tc_triple_buffer

## Overview

`tc_triple_buffer` is a lock-free, single-producer single-consumer (SPSC) triple
buffer implemented in Rust. This crate enables a safe and efficient method for
transferring data between a single producer thread and a single consumer thread
with minimal contention.

## Features
- Lock-free: Eliminates the need for traditional mutexes or synchronization
primitives, ensuring low-latency communication.
- SPSC Model: Designed specifically for scenarios with one producer and one
consumer, offering simplicity and optimized performance.

Installation
To add `tc_triple_buffer` to your Rust project, include the following in your
Cargo.toml:

```toml
[dependencies]
tc_triple_buffer = { git = "https://github.com/tiger-chan/tc-rs" }
```

## Example Usage

Here’s a basic example of how to use the TripleBuffer in a producer-consumer
scenario:

```rust
use tc_triple_buffer::*; 
use std::thread;

fn main() {
    let TripleBuffer::<u64>(mut publisher, mut subscriber) = TripleBuffer::default();

    let producer = thread::spawn(move || {
    for i in 1..1000 {
        *publisher.data() = i;
        publisher.commit();
    }
    });

    let mut prev = 0;
    while prev != 999 {
        let next = *subscriber.data();
        assert!(prev <= next);
        prev = next;
    }

    let _ = producer.join();
}
```

