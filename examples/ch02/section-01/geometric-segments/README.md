# Geometric segments and abstract rectangles

**Section:** 2.1 · **Edition scope:** shared

## Big idea

A point and a line segment should be manipulated through geometric operations, not by peeking into their coordinates in client code.

## Rust design and invariants

Midpoint and length are methods on Segment; Rectangle validates corner order and uses its dimensions for area and perimeter. Both structures preserve a simple input invariant.

## Complexity

All operations use constant work and memory. Floating-point comparisons need a tolerance when results involve square roots.

## Run

```sh
cargo run --bin ch02-geometric-segments
cargo test --bin ch02-geometric-segments
```

## Exercise

Represent a rectangle by a center point, width, and height without changing its area and perimeter API.

[Undergraduate section guide](../../../../chapters/ch02/section-01/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
