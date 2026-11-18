# Full metacircular evaluator (`quote`, `cond`, `define`, `set!`, `let*`, `scan_out_defines`)

**Book concepts:** Section 4.1 (Subsections 4.1.1–4.1.6). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A complete S-expression interpreter with linked mutable lexical frames (`Rc<RefCell<Frame>>` / `Frame`), special forms (`quote`, `if`, `cond`, `and`/`or`, `begin`, `set!`, `define`, `lambda`), derived expressions (`let`, `let*`), `scan_out_defines` for mutually recursive internal definitions, and Y-combinator recursion without `define` (Exercise 4.21).

## Implementation walkthrough

`parse_program` parses Scheme-style S-expressions. `scan_out_defines` rewrites internal `define`s inside procedure bodies into `*unassigned*` `let` bindings followed by `set!` assignments so mutually recursive helpers (`even?` and `odd?`) resolve regardless of declaration order. Derived `let` and `let*` expand syntactically into `lambda` applications before evaluation.

## Complexity and correctness

Variable lookup walks `O(depth)` enclosing frames. Scanning out internal definitions occurs once per `lambda`/`define` body construction, preventing stale lexical visibility.

## Run it

```sh
cargo run --bin ch04-metacircular-evaluator
cargo test --bin ch04-metacircular-evaluator
python3 examples/ch04/section-01/metacircular-evaluator/main.py
python3 -m unittest examples/ch04/section-01/metacircular-evaluator/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add `unless` and named `let` (`(let loop ((i 0)) ...)`) as syntactic transformations in `eval`.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
