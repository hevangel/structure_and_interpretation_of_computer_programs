# JavaScript statement parser, block scoping, and return unwinding

**Book concepts:** Section 4.1 (Subsections 4.1.1–4.1.4). **Edition applicability:** 2022-javascript.

## What this demonstrates

The 2022 JavaScript edition of SICP replaces expression-only Scheme forms with statement-oriented syntax: recursive-descent parsing for `const`/`let`/`function` declarations, block-level lexical scoping with temporal dead zone (TDZ) scanning, `if`/`else`, and explicit `ReturnValue` control-flow unwinding.

## Implementation walkthrough

`Parser` produces statement and expression AST nodes (`ConstDecl`, `LetDecl`, `FunDecl`, `Block`, `Return`, `If`). Before executing any block, `scan_declarations` pre-allocates uninitialized binding slots for every `const`, `let`, and `function` declared in that block, enforcing TDZ checks and `const` immutability. `eval_sequence` short-circuits immediately when `Signal::ReturnValue` propagates out of nested blocks until caught at the enclosing function call boundary.

## Complexity and correctness

Block scanning takes O(S) in the number of statements in a block, and `ReturnValue` unwinding exits nested blocks in O(nesting depth) without corrupting outer frames.

## Run it

```sh
cargo run --bin ch04-javascript-parser
cargo test --bin ch04-javascript-parser
python3 examples/ch04/section-01/javascript-parser/main.py
python3 -m unittest examples/ch04/section-01/javascript-parser/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add `while (cond) { ... }` loops and `break`/`continue` control signals alongside `ReturnValue`.

[Section introduction](../../../../chapters/ch04/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
