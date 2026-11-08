# Mutable FIFO queue abstraction

**Book concepts:** Section 3.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Separating a queue's interface from its storage gives clients a meaningful FIFO contract.

## Implementation walkthrough

VecDeque provides efficient insertion at the rear and removal from the front. A thin teaching wrapper records the invariant and API surface.

## Complexity and correctness

Push and pop are amortized O(1), and checking length is O(1). A linked-pair implementation could use interior mutability but is unnecessary here.

## Run it

```sh
cargo run --bin ch03-mutable-queue
cargo test --bin ch03-mutable-queue
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement a queue using two stacks and explain how amortized analysis proves O(1) per operation.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
