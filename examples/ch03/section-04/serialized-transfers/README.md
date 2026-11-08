# Concurrency and serialized transfers

**Book concepts:** Section 3.4. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Two account balances share a conserved total even when multiple threads attempt transfers.

## Implementation walkthrough

Each transfer acquires one mutex covering both balances, which makes checking and updating a single atomic critical section. Cloning Arc transfers shared ownership into threads.

## Complexity and correctness

Each transfer performs O(1) protected work; serialization trades potential throughput for a simple invariant. Thread scheduling is nondeterministic.

## Run it

```sh
cargo run --bin ch03-serialized-transfers
cargo test --bin ch03-serialized-transfers
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Try separate account locks. Define one global lock order to avoid deadlock and test the conserved total.

[Section introduction](../../../../chapters/ch03/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
