# 5.5 Compilation · [Chapter 5](../README.md)

## The big idea

`eceval` interprets every AST node repeatedly, saving and restoring registers *just in case* a sub-expression might clobber them. A **compiler** inspects the source AST **once up front**, emitting register-machine instructions parameterized by **target register** and **linkage (`Next`, `Return`, `Label`)**.

By tracking static sets **`needs`** (registers read) and **`modifies`** (registers overwritten) on every instruction sequence, **`preserving(regs, seq1, seq2)`** inserts `save`/`restore` pairs **only when a register is modified by `seq1` AND actually read by `seq2`**—eliminating ~80% of runtime stack traffic! Adding compile-time **lexical addressing (`(depth, slot)`)** replaces string hash-map lookups with direct array indexing.

## What you should focus on

Focus on ****target/linkage compilation (`Next`, `Return`, `Label`), `needs`/`modifies` static analysis, `preserving` register saving, lexical `(depth, slot)` addressing, and compiled vs interpreted performance****. Every machine model and compiler pass makes hidden runtime cost directly measurable.

## Connecting the concept to Rust and Python

- **In Rust:** Moves smoothly from toy stack VMs (`bytecode-compiler`, `constant-folding-compiler`, `lexical-addresses`) up to the full target/linkage compiler (`register-compiler`) storing persistent code vectors on `CompiledVM` cleanly.
- **In Python:** Pairs the same `InstructionSequence(needs: frozenset, modifies: frozenset, instrs: list)` algebra and `preserving` optimizer with a Python register VM and tail-call trampoline (`javascript-tail-calls`).
- **Key contrast:** Both `main.rs` and `main.py` in `register-compiler` prove identical exact counts: compiled `factorial(6)` uses `32` total pushes / `17` max depth compared to `176` pushes / `33` max depth in `eceval`!

## Subsections

- [5.5.1 Compiler Structure](subsection-01/INTRO.md)
- [5.5.2 Compiling Expressions](subsection-02/INTRO.md)
- [5.5.3 Applications and Return Statements](subsection-03/INTRO.md)
- [5.5.4 Instruction Sequence Composition](subsection-04/INTRO.md)
- [5.5.5 Example of Compiled Code](subsection-05/INTRO.md)
- [5.5.6 Lexical Addressing](subsection-06/INTRO.md)
- [5.5.7 Compiler-Evaluator Interface](subsection-07/INTRO.md)

## Runnable examples in this section

- [5.5 Compile expressions to stack-machine bytecode](../../../examples/ch05/section-05/bytecode-compiler/README.md)
- [5.5 Tail calls through a trampoline](../../../examples/ch05/section-05/javascript-tail-calls/README.md)
- [5.5 Compile variable names to lexical addresses](../../../examples/ch05/section-05/lexical-addresses/README.md)
- [5.5 Constant folding before bytecode generation](../../../examples/ch05/section-05/constant-folding-compiler/README.md)
- [5.5 Register compiler with preserving, linkages, and lexical addressing](../../../examples/ch05/section-05/register-compiler/README.md)

## Check your understanding

1. Why can compiling `(+ x 1)` preserve `env` around `1` **without** emitting a single `save env` / `restore env`?
2. Why does compiling `define (factorial n) ...` across two top-level calls require persistent code/label memory on the VM instead of resetting PC `0`?

**Experiment:** Inspect the instructions emitted by `register-compiler` for `preserving([env, continue], compile(op), compile(operands))` and compare push counts against `explicit-control`.

[Chapter 5 overview](../README.md) · [Edition notes](../../../docs/editions.md)
