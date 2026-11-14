# Separate syntactic analysis from execution

**Section:** 4.1 · **Source concept:** shared

## Goal

Demonstrate that a syntax tree can be analyzed once into an executable procedure, then run against many environments.

## Implementation walkthrough

The analysis step constructs closures for number literals, variable references, and addition; execution supplies a runtime environment. A missing binding produces an error rather than silently defaulting.

## Complexity and correctness

Analysis is O(n) in tree size, and each run evaluates the resulting closures. Each name lookup is expected O(1). This tiny model omits declarations and functions.

## Run and validate

```sh
cargo run --bin ch04-analysis-then-execution
cargo test --bin ch04-analysis-then-execution
```

## Extend the example

Add multiplication and compare interpreting the original AST versus running an analyzed expression repeatedly.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md). Source code and explanatory text are original Rust teaching material.
