# Lazy streams with Rust iterators

**Book concepts:** Section 3.5. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Produce only as many members of a potentially infinite sequence as the consumer requests.

## Implementation walkthrough

Use std::iter::successors to generate integers on demand, filter them by a primality predicate, and stop with take.

## Complexity and correctness

The iterator uses O(1) intermediate memory for finite prefix consumption; each new prime still costs trial division.

## Run it

```sh
cargo run --bin ch03-lazy-streams
cargo test --bin ch03-lazy-streams
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Write a memoizing stream with shared tails and contrast it with a single-pass Iterator.

[Section introduction](../../../../chapters/ch03/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
