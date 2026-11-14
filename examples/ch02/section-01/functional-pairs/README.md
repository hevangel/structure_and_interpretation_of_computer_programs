# Pairs represented as functions

**Section:** 2.1 · **Edition scope:** shared

## Big idea

A pair does not have to be stored in two fields: it can be represented by a function whose behavior selects one of two components.

## Rust design and invariants

A closure captures both component values and chooses one when called with a selector. Clients interact through first and second selectors rather than inspecting closure internals.

## Complexity

Construction and selection are constant-time for Copy components. This illustrates the idea of a data abstraction defined by operations.

## Run

```sh
cargo run --bin ch02-functional-pairs
cargo test --bin ch02-functional-pairs
```

## Exercise

Generalize this representation to different component types, and explain why a single closure's return type becomes complicated.

[Undergraduate section guide](../../../../chapters/ch02/section-01/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
