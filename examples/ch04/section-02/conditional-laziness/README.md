# Conditional evaluation avoids unneeded computation

**Section:** 4.2 · **Source concept:** shared

## Goal

An expression language may evaluate only the selected branch of a conditional, making it possible to avoid errors or nonterminating calculations in unused branches.

## Implementation walkthrough

Implement an AST containing literals, division and an if-zero expression. The evaluator computes the condition first and evaluates exactly one branch.

## Complexity and correctness

Execution cost depends on the selected branch, not the total expression tree. Errors in an unselected branch do not affect the result.

## Run and validate

```sh
cargo run --bin ch04-conditional-laziness
cargo test --bin ch04-conditional-laziness
```

## Extend the example

Add a counter expression and compare evaluation counts with an eager conditional.

[Section introduction](../../../../chapters/ch04/section-02/INTRO.md). Source code and explanatory text are original Rust teaching material.
