# Evaluator with lexical closures

**Book concepts:** Section 4.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A function is not just its source body: it must remember the environment where it was defined. That captured environment distinguishes lexical from dynamic scope.

## Implementation walkthrough

Values are numbers or closures. Evaluating a lambda captures its defining environment; applying it extends that captured frame with the argument, not with the caller's current local variables.

## Complexity and correctness

This educational interpreter uses explicit frame captures for clarity. A production interpreter shares immutable or reference-counted environment frames and handles recursive bindings carefully.

## Run it

```sh
cargo run --bin ch04-lexical-closures
cargo test --bin ch04-lexical-closures
python3 examples/ch04/section-01/lexical-closures/main.py
python3 -m unittest examples/ch04/section-01/lexical-closures/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement nested recursive functions and choose an environment representation that avoids copying whole maps.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
