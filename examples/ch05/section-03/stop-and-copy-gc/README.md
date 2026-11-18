# Vector-memory pairs and Cheney stop-and-copy garbage collector

**Book concepts:** Section 5.3 (Subsections 5.3.1–5.3.2). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Implements SICP's vector-memory representation (`the_cars` / `the_cdrs` and `new_cars` / `new_cdrs` with tagged pointers) and Cheney's iterative stop-and-copy garbage collector using `BrokenHeart` forwarding pointers, breadth-first `scan`/`free` compaction, and `gc_flip` semispace swapping across shared DAGs and cyclic structures.

## Implementation walkthrough

Each reachable pair in old space is moved once to `new_free`, leaving behind a `BrokenHeart(target)` tag in its old `car` slot. Because the `scan` cursor iteratively relocates both `car` and `cdr` fields inside `new_cars`/`new_cdrs` without recursion, shared subgraphs preserve pointer identity and cyclic lists terminate cleanly without exhausting the call stack.

## Complexity and correctness

Time per collection is `O(live_pairs)` and auxiliary traversal stack is `O(1)` because the compacted semispace prefix between `scan` and `free` serves as the BFS queue.

## Run it

```sh
cargo run --bin ch05-stop-and-copy-gc
cargo test --bin ch05-stop-and-copy-gc
python3 examples/ch05/section-03/stop-and-copy-gc/main.py
python3 -m unittest examples/ch05/section-03/stop-and-copy-gc/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add multiple independent roots (`roots: &[Pointer]`) and inspect how `collect_all_roots` compacts disjoint reachable subgraphs contiguously.

[Section introduction](../../../../chapters/ch05/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
