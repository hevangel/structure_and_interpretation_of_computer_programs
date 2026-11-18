# 5.5.7 Compiler-Evaluator Interface

## What this subsection teaches

How do **compiled procedures** (machine-code `entry` labels) and **interpreted procedures** (AST lambdas in `eceval`) call **each other** inside one running system?

They agree on a **shared runtime value and calling contract**:
1. Both represent environments as the same frame structure (`ctenv`-compatible indexed frames + global frame).
2. When a compiled call hits an interpreted `Closure`, it passes `argl` and jumps into `eceval`'s **`apply_dispatch`** with `continue` pointing to `after_call`.
3. When `eceval` applies a `CompiledProc(entry, env)`, it extends `env` with `argl` and jumps directly to `entry` with `continue = return_addr`!

## Connecting the concept to Rust and Python

- **In Rust:** Represented cleanly in `Value::Compiled(entry, env)` alongside `Value::Primitive(...)` and `eval_compiled` running persistent multi-definition code on `CompiledVM` (`register-compiler`).
- **In Python:** Mirrors the same unified `CompiledProc` / `Primitive` representation so top-level calls invoke compiled helper definitions across repeated `eval_compiled` calls cleanly.
- **Why compare both:** Sharing one value & frame ABI bridges Chapter 4 (interpretation) and Chapter 5 (compilation)—the exact pattern used by real hybrid VMs (JIT + interpreter).

## Exercise

Design the shared calling convention allowing compiled `map(f, xs)` to invoke an interpreted user lambda `f` passed at runtime.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
