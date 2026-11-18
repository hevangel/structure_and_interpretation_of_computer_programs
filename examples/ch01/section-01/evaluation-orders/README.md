# Evaluation orders and lexical block scope

**Book concepts:** Section 1.1 (Subsections 1.1.5 and 1.1.8). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Applicative-order evaluation ("evaluate arguments then apply") and normal-order evaluation ("fully expand then reduce") produce different termination behavior when an unused argument diverges, and block-structured internal declarations preserve outer bindings via lexical shadowing.

## Implementation walkthrough

A compact AST evaluator tracks step counts under a bounded step budget. `eval_applicative` evaluates every argument expression before binding parameters; `eval_normal` binds deferred argument thunks carrying their lexical environment and forces them only when inspected by primitives or conditionals. `Block` nodes extend a cloned frame so inner definitions shadow outer names without mutating them.

## Complexity and correctness

When arguments are reused multiple times without memoization, normal order may re-evaluate expressions; when an argument is unused and non-terminating (`test(0, p())`), normal order terminates in O(1) steps while applicative order diverges.

## Run it

```sh
cargo run --bin ch01-evaluation-orders
cargo test --bin ch01-evaluation-orders
python3 examples/ch01/section-01/evaluation-orders/main.py
python3 -m unittest examples/ch01/section-01/evaluation-orders/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Count how many extra steps normal order performs when evaluating `square(add(1, 2))` where `square(x) = x + x`, and compare that with memoized call-by-need.

[Section introduction](../../../../chapters/ch01/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
