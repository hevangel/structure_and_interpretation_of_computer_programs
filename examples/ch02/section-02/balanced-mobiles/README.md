# Recursive mobiles and torque balance

**Section:** 2.2 · **Edition scope:** shared

## Big idea

A mobile is balanced only if its two branch torques match and each suspended submobile is balanced. This requires recursive structural reasoning.

## Rust design and invariants

Define weights and mobiles as separate enum variants. A branch multiplies its length by the total weight beneath it; recursive balance tests descend only into mobile variants.

## Complexity

O(nodes) traversal time and O(height) recursion stack, where a node is a branch or weight. No mutable state is needed.

## Run

```sh
cargo run --bin ch02-balanced-mobiles
cargo test --bin ch02-balanced-mobiles
```

## Exercise

Construct a mobile whose top-level torques match but an inner submobile is unbalanced.

[Undergraduate section guide](../../../../chapters/ch02/section-02/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
