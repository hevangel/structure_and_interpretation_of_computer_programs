# Backtracking constraint search: five dwellings

**Section:** 4.3 · **Source concept:** shared

## Goal

Turn a logic puzzle into a set of constraints and enumerate only assignments that satisfy them.

## Implementation walkthrough

The five people occupy different floors. Reject partial assignments violating fixed-floor and adjacency requirements; after all assignments, check the relative-floor constraint. Backtracking restores the assignment list between branches.

## Complexity and correctness

At most 5! complete permutations are examined; early pruning avoids many of them. The search is generalizable to larger constraint satisfaction problems.

## Run and validate

```sh
cargo run --bin ch04-multiple-dwelling
cargo test --bin ch04-multiple-dwelling
```

## Extend the example

Add a generic predicate interface for partial assignments, then compare node counts with and without pruning.

[Section introduction](../../../../chapters/ch04/section-03/INTRO.md). Source code and explanatory text are original Rust teaching material.
