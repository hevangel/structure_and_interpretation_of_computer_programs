# Register compiler with `preserving`, linkages, and lexical addressing

**Book concepts:** Section 5.5 (Subsections 5.5.1–5.5.7). **Edition applicability:** shared conceptual material across editions *(the 2022 JavaScript edition additionally threads explicit `return` linkage around function bodies)*.

## What this demonstrates

An SICP register-machine compiler turning AST expressions into targeted `InstructionSequence` blocks annotated with `needs` and `modifies` register sets. Using `preserving`, linkage descriptors (`Next`, `Return`, `Label`), and compile-time lexical addressing `(frame_idx, disp_idx)`, compiled code eliminates unnecessary stack saves and symbol table lookups, executing `factorial(6)` with a fraction of the stack pushes of the explicit-control interpreter.

## Implementation walkthrough

`preserving(regs, seq1, seq2)` inserts `save`/`restore` around `seq1` only for registers modified by `seq1` that `seq2` actually needs. When compiling nested `lambda` bodies, `find_variable` resolves parameters at compile time into `(frame, displacement)` lexical coordinates so runtime variable lookup indexes vectors directly.

## Complexity and correctness

Lexical lookup drops from `O(depth * vars)` associative scanning to `O(depth)` pointer indexing, and register preservation eliminates stack traffic around non-interfering operand evaluations such as constants or lexical loads.

## Run it

```sh
cargo run --bin ch05-register-compiler
cargo test --bin ch05-register-compiler
python3 examples/ch05/section-05/register-compiler/main.py
python3 -m unittest examples/ch05/section-05/register-compiler/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add open-coding support for `+` and `*` when arguments are not shadowed in the compile-time environment, reducing stack pushes on `factorial` even further.

[Section introduction](../../../../chapters/ch05/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
