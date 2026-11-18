# Four-level numeric tower, coercion, `raise`, and `drop`

**Book concepts:** Section 2.5 (Subsections 2.5.1 and 2.5.2). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A linear type tower (`Int < Rat < Real < Complex`) replaces quadratic pairwise coercion tables with linear upward `raise` steps and canonical downward simplification via `project` and `drop`.

## Implementation walkthrough

Each `TowerNum` level defines `rank`, `raise_once`, `project`, and generic equality `equ`. Binary operations coerce the lower-ranked operand upward to match the higher-ranked operand, evaluate the result, and invoke `drop` to collapse values back down the tower whenever `project` followed by `raise_once` remains equal to the original value (for instance, `(3 + 4i) * (3 - 4i)` collapses from `Complex(25, 0)` to `Int(25)`).

## Complexity and correctness

Cross-type coercion requires at most O(L) steps for a tower of `L = 4` levels, instead of O(L^2) explicit conversion functions. Round-trip verification (`raise(project(x)) == x`) ensures `drop` never loses information.

## Run it

```sh
cargo run --bin ch02-numeric-tower-coercion
cargo test --bin ch02-numeric-tower-coercion
python3 examples/ch02/section-05/numeric-tower-coercion/main.py
python3 -m unittest examples/ch02/section-05/numeric-tower-coercion/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Extend the tower with an `ExactIrrational` or `GaussianInt` level and check whether a strict linear ordering still holds.

[Section introduction](../../../../chapters/ch02/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
