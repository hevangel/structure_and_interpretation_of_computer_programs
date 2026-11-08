# Bidirectional constraint propagation

**Book section:** 3.3. **Edition:** shared.

## Concept

A relation such as x+y=z can infer any missing quantity if two values are known. A network of connected constraints propagates newly discovered values.

## Rust implementation

Store connectors as optional numbers and encode add and multiply constraints. Iterate to a fixed point, inferring missing values and rejecting conflicting assignments. A Celsius/Fahrenheit network demonstrates propagation in both directions.

## Complexity and invariants

For a small finite network, each pass scans O(C) constraints; at most O(V) previously unknown connectors gain values. Contradictions should be reported explicitly.

## Run and test

```sh
cargo run --bin ch03-constraint-propagation
cargo test --bin ch03-constraint-propagation
```

## Extension exercise

Add a nonlinear or ambiguous constraint and decide when multiple solutions must be represented.

This is independently authored Rust teaching code, not a verbatim textbook translation.

[Section introduction](../../../../chapters/ch03/section-03/INTRO.md)
