# Nondeterministic continuation-passing `amb` evaluator

**Book concepts:** Section 4.3 (Subsections 4.3.1–4.3.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A continuation-passing backtracking evaluator (`analyze` returning `(env, succeed, fail)`) implementing `amb`, `require`, undo-on-backtrack `set!`, `permanent-set!`, and `if-fail`, applied to Pythagorean triples, the multiple-dwelling constraint puzzle, and ambiguous prepositional-phrase natural-language parsing.

## Implementation walkthrough

Every analyzed AST node threads two continuations: `succeed(val, fail2)` receives both the chosen value and the current backtracking thunk `fail2`. `set!` saves the previous binding inside the new failure continuation so assignments automatically roll back when backtracking passes them, whereas `permanent-set!` passes `fail2` unchanged to count failed search attempts.

## Complexity and correctness

Depth-first chronological backtracking explores choice trees in $O(1)$ auxiliary heap per node apart from saved environments and undo closures.

## Run it

```sh
cargo run --bin ch04-amb-evaluator
cargo test --bin ch04-amb-evaluator
python3 examples/ch04/section-03/amb-evaluator/main.py
python3 -m unittest examples/ch04/section-03/amb-evaluator/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add `ramb` (random choice ordering) or an `amb`-based N-queens constraint solver and enumerate all valid placements.

[Section introduction](../../../../chapters/ch04/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
