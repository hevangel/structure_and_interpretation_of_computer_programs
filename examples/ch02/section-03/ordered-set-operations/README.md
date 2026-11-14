# Set union and intersection by merging

**Section:** 2.3 · **Edition scope:** shared

## Big idea

Sets stored in sorted order can be combined without nested membership searches.

## Rust design and invariants

Use two indices to walk sorted unique sequences. When elements match, advance both; otherwise advance only the smaller. An invariant guarantees all visited items are already in sorted output order.

## Complexity

O(m+n) time, O(m+n) output space for sets of sizes m and n.

## Run

```sh
cargo run --bin ch02-ordered-set-operations
cargo test --bin ch02-ordered-set-operations
```

## Exercise

Implement set difference with the same two-pointer approach.

[Undergraduate section guide](../../../../chapters/ch02/section-03/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
