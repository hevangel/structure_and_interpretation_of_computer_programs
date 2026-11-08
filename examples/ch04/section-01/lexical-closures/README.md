# Evaluator with lexical closures

**Book section:** 4.1. **Edition:** shared.

## Concept

A function is not just its source body: it must remember the environment where it was defined. That captured environment distinguishes lexical from dynamic scope.

## Rust implementation

Values are numbers or closures. Evaluating a lambda copies its defining environment; applying it extends that captured frame with the argument, not with the caller's current local variables.

## Complexity and invariants

This educational interpreter uses whole-map clones for clarity. A real interpreter would share environment frames and handle recursive bindings more carefully.

## Run and test

```sh
cargo run --bin ch04-lexical-closures
cargo test --bin ch04-lexical-closures
```

## Extension exercise

Implement nested recursive functions and choose an environment representation that avoids copying whole maps.

This is independently authored Rust teaching code, not a verbatim textbook translation.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md)
