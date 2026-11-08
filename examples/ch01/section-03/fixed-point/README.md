# Fixed-point iteration

**Book concepts:** Section 1.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Search for a value x satisfying f(x) = x by repeatedly applying f.

## Implementation walkthrough

Pass a function as an argument; stop when successive estimates are sufficiently close. A maximum iteration count makes divergence reportable.

## Complexity and correctness

Each iteration evaluates f once, so complexity depends on the contraction rate. Convergence is not guaranteed for arbitrary functions.

## Run it

```sh
cargo run --bin ch01-fixed-point
cargo test --bin ch01-fixed-point
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Try a noncontractive function and explain why a maximum iteration count is essential.

[Section introduction](../../../../chapters/ch01/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
