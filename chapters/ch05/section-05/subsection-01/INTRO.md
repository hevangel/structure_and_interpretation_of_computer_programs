# 5.5.1 Compiler Structure

## What this subsection teaches

Instead of interpreting AST syntax at runtime, `compile(exp, target, linkage)` translates AST nodes directly into register-machine instructions parameterized by:
- **`target`:** which register (`Val`, `Proc`, `Argl`, ...) receives the expression's value
- **`linkage`:** what control does afterward—**`Next`** (fall through), **`Return`** (jump to `reg(continue)`), or **`Label(L)`** (jump unconditionally to `L`).

Returning `{ needs, modifies, instructions }` alongside the emitted code drives register-saving optimizations cleanly (`register-compiler`).

## Connecting the concept to Rust and Python

- **In Rust:** Combines AST expression enums, target/linkage enums, and `InstructionSequence { needs, modifies, statements }` (`bytecode-compiler`, `register-compiler`).
- **In Python:** Mirrors `compile_expr(expr, target, linkage) -> InstructionSequence` cleanly with frozen dataclasses and `frozenset[Reg]` usage metadata.
- **Why compare both:** Notice how `compile` decouples **AST traversal** (`compile_if`, `compile_lambda`) from **optimization/linking** (`preserving`, `tack_on_seq`) cleanly in both Rust and Python.

## Exercise

Explain the three linkage modes (`Next`, `Return`, `Label`) and how `Return` linkage compiles tail calls directly to `goto (reg proc)` without saving `continue`!

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
