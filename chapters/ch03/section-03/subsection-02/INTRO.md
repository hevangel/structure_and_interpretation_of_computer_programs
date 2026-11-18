# 3.3.2 Representing Queues

## What this subsection teaches

Scanning to the end of a linked list to append makes `enqueue` take `O(n)` time. Maintaining **both** a `front` pointer and a `rear` pointer gives `O(1)` insertion at the rear and `O(1)` removal at the front—provided every mutation preserves the invariant that `front` and `rear` agree on empty vs single-element boundaries.

Extending to a doubly-linked **Deque** adds `O(1)` push/pop at both ends.

## Connecting the concept to Rust and Python

- **In Rust:** Standard code uses `std::collections::VecDeque`; implementing an educational queue or doubly-linked `Deque` with arena indices (`Option<usize>`) avoids `Weak`/`RefCell` boilerplate while preserving `O(1)` bounds.
- **In Python:** Standard code uses `collections.deque`; implementing an explicit front/rear pointer queue or arena `Deque` demonstrates why empty-queue and one-element edge cases must update both ends cleanly.
- **Why compare both:** Compare [`mutable-queue`](../../../../examples/ch03/section-03/mutable-queue/README.md) and [`cycle-detection-deque`](../../../../examples/ch03/section-03/cycle-detection-deque/README.md) across both languages to inspect the exact boundary invariants.

## Exercise

Trace `push_back(1)`, `push_front(2)`, `pop_back()`, `pop_front()` on an empty deque verifying `front`/`rear` stay consistent after emptying.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
