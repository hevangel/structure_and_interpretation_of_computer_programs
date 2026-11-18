# Normal-order lazy evaluator with memoized thunks and lazy lists

**Book concepts:** Section 4.2 (Subsections 4.2.1–4.2.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A full call-by-need interpreter that wraps compound-procedure arguments in memoized `Thunk` cells (`Unevaluated(expr, env) -> Evaluated(val)`) and forces values with `actual_value` only at primitive applications, `if` predicates, and operator positions. Even `unless` guards against division by zero and Church-style lazy `cons`/`car`/`cdr` support self-referential infinite `ones`, `integers`, and `fibs` streams inside the interpreted language.

## Implementation walkthrough

In `apply`, compound closures bind each parameter to a delayed `Thunk` in the caller's environment without evaluating operands up front. `force_it` caches the forced result inside the `Thunk` cell on first demand so repeated references (`(* w w)`) increment `memoized_hits` without re-running side effects in `(id 7)`.

## Complexity and correctness

Call-by-need avoids duplicate work of call-by-name while preserving non-strict termination semantics: shared subexpressions are evaluated at most once per thunk.

## Run it

```sh
cargo run --bin ch04-lazy-evaluator
cargo test --bin ch04-lazy-evaluator
python3 examples/ch04/section-02/lazy-evaluator/main.py
python3 -m unittest examples/ch04/section-02/lazy-evaluator/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement a lazy `map` procedure inside the interpreter and compute the 12th Fibonacci number or partial sums of infinite streams.

[Section introduction](../../../../chapters/ch04/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
