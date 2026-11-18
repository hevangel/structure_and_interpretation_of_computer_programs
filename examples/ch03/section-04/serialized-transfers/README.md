# Concurrency interleavings and deadlock-free serialized exchange

**Book concepts:** Section 3.4 (Subsections 3.4.1 and 3.4.2). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Unserialized concurrent read-modify-write operations yield anomalous intermediate states beyond sequential permutations (Exercise 3.38), while ordering serializer acquisition by unique account IDs eliminates deadlocks when opposing threads call `serialized_exchange(A, B)` and `serialized_exchange(B, A)` simultaneously (Exercise 3.48).

## Implementation walkthrough

`enumerate_unserialized_ex3_38` performs a depth-first exploration over all valid read/write interleavings of three concurrent account updates, finding all 8 reachable outcomes (`{35, 40, 45, 50, 55, 80, 90, 110}`). `SerializedAccount` pairs a balance mutex with a unique numeric `id`; `serialized_exchange` always locks the lower-ID account before the higher-ID account, breaking circular wait across multi-threaded exchanges.

## Complexity and correctness

Lock ordering establishes a strict global partial order on mutex acquisition, guaranteeing deadlock freedom for arbitrary numbers of concurrent account swaps while preserving total balance conservation.

## Run it

```sh
cargo run --bin ch03-serialized-transfers
cargo test --bin ch03-serialized-transfers
python3 examples/ch03/section-04/serialized-transfers/main.py
python3 -m unittest examples/ch03/section-04/serialized-transfers/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Simulate a naive `serialized_exchange` without ID ordering using bounded timeouts and measure how quickly opposing threads hit a deadlock cycle.

[Section introduction](../../../../chapters/ch03/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
