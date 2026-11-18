# Mutable pair cycle detection and O(1) deque

**Book concepts:** Section 3.3 (Subsections 3.3.1 and 3.3.2). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Structure sharing and `set_car!`/`set_cdr!` mutations break naive tree traversal (`count_pairs` on three cells can return `3`, `4`, `7`, or diverge on cycles), requiring identity tracking or Floyd's O(1)-space cycle detector, while doubly-linked mutable nodes support O(1) operations at both ends of a `Deque`.

## Implementation walkthrough

In Rust, `PairHeap` models mutable cons cells inside an index arena (`set_car`, `set_cdr`) and compares `naive_count_pairs`, `unique_count_pairs` (via `BTreeSet<usize>`), and `has_cdr_cycle` (Floyd's tortoise-and-hare algorithm). `Deque<T>` stores doubly-linked nodes with free-list recycling for O(1) `front_insert`, `rear_insert`, `front_delete`, and `rear_delete`. Python models the exact same pointer structures with mutable `ConsCell` and doubly-linked `_DequeNode` objects.

## Complexity and correctness

Floyd's algorithm detects cycles along the `cdr` spine in O(n) time and O(1) auxiliary space. Every `Deque` insertion and deletion updates at most two neighbor pointers in O(1) worst-case time.

## Run it

```sh
cargo run --bin ch03-cycle-detection-deque
cargo test --bin ch03-cycle-detection-deque
python3 examples/ch03/section-03/cycle-detection-deque/main.py
python3 -m unittest examples/ch03/section-03/cycle-detection-deque/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Extend Floyd's cycle detector to return the exact cycle entry cell and cycle length when `has_cdr_cycle` is true.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
