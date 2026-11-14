# Power set by recursive decomposition

**Section:** 2.2 · **Edition scope:** shared

## Big idea

A set with n distinct elements has 2^n subsets. Every subset either includes the next item or excludes it.

## Rust design and invariants

Start with the empty subset. For each element, clone all existing subsets and append that element to create the second half. The two halves remain disjoint when original elements are distinct.

## Complexity

Time and total output storage are O(n·2^n), because the result actually contains that many element occurrences.

## Run

```sh
cargo run --bin ch02-powerset
cargo test --bin ch02-powerset
```

## Exercise

Rewrite the algorithm as recursive include/exclude search and compare output ordering.

[Undergraduate section guide](../../../../chapters/ch02/section-02/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
