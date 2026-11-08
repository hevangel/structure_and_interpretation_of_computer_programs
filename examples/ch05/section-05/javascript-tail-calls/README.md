# Tail calls through a trampoline

**Book concepts:** Section 5.5. **Edition applicability:** 2022-javascript.

## What this demonstrates

Model explicit tail calls without letting recursive host-language calls accumulate stack frames.

## Implementation walkthrough

A Bounce is either a finished value or a boxed continuation. The trampoline repeatedly invokes one continuation and discards its predecessor.

## Complexity and correctness

O(n) computation, O(1) nested call-stack depth, and one small heap allocation per bounce in this teaching implementation.

## Run it

```sh
cargo run --bin ch05-javascript-tail-calls
cargo test --bin ch05-javascript-tail-calls
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add a tail-recursive mutual recursion test with even and odd predicates and discuss return-stack discipline in the JavaScript adaptation.

[Section introduction](../../../../chapters/ch05/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
