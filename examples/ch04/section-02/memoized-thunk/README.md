# Call-by-need with a memoized thunk

**Book concepts:** Section 4.2. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Delay the computation of a value until it is explicitly demanded, then reuse the first result.

## Implementation walkthrough

The thunk owns a FnOnce closure, consumes it on the first force call, and retains the resulting value for subsequent calls.

## Complexity and correctness

Each thunk body runs at most once. This is a small model of memoization, not a complete lazy language evaluator.

## Run it

```sh
cargo run --bin ch04-memoized-thunk
cargo test --bin ch04-memoized-thunk
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Compare memoized call-by-need against nonmemoized call-by-name for a thunk with observable side effects.

[Section introduction](../../../../chapters/ch04/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
