# Constant folding before bytecode generation

**Section:** 5.5 · **Source concept:** shared

## Goal

Optimize an expression AST by evaluating subexpressions whose inputs are all known constants, then compile the simplified tree to a virtual machine.

## Implementation walkthrough

Keep runtime variables explicit. The optimizer returns a new AST and never removes a variable's evaluation; compiling after folding generates a shorter instruction sequence.

## Complexity and correctness

Each AST transformation visits O(n) nodes. This pure arithmetic language makes folding semantics-preserving; a language with side effects needs more care.

## Run and validate

```sh
cargo run --bin ch05-constant-folding-compiler
cargo test --bin ch05-constant-folding-compiler
```

## Extend the example

Add algebraic simplifications like x+0=x, then describe cases where floating-point identities are unsafe.

[Section introduction](../../../../chapters/ch05/section-05/INTRO.md). Source code and explanatory text are original Rust teaching material.
