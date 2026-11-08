# Square roots by iterative improvement

**Book concepts:** Section 1.1. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Find an approximation to a square root without using the library square-root function. A local improvement rule is more useful than a giant formula.

## Implementation walkthrough

Newton's update averages the current guess with x / guess. The loop stops based on the residual and scales the tolerance with the input.

## Complexity and correctness

Each update needs constant work. Near the root, the number of correct digits typically grows quadratically; the bounded loop protects against nontermination.

## Run it

```sh
cargo run --bin ch01-newton-sqrt
cargo test --bin ch01-newton-sqrt
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Compare an absolute error threshold with a relative error threshold for x = 1e-12 and x = 1e12.

[Section introduction](../../../..//chapters/ch01/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
