# 5.5.4 Instruction Sequence Composition

## What this subsection teaches

How does `preserving([env, continue], seq1, seq2)` know when saving a register is redundant? Every `InstructionSequence` carries two small static register sets:
- **`needs`:** registers `seq` reads *before* assigning them
- **`modifies`:** registers `seq` overwrites.

**Algebra:**
- `append(s1, s2)` has `needs = s1.needs ∪ (s2.needs \ s1.modifies)` and `modifies = s1.modifies ∪ s2.modifies`.
- `preserving(r, s1, s2)` wraps `s1` in `save(r) ... restore(r)` **iff `r ∈ s1.modifies` AND `r ∈ s2.needs`!**

## Connecting the concept to Rust and Python

- **In Rust:** `InstructionSequence { needs: BTreeSet<Reg>, modifies: BTreeSet<Reg>, statements: Vec<Instr> }` uses `BTreeSet` set union/difference operations cleanly (`register-compiler`).
- **In Python:** Python uses `frozenset[Reg]` union (`|`) and difference (`-`) directly inside `append_instruction_sequences` and `preserving`, matching Rust set algebra 1:1.
- **Why compare both:** Testing `preserving([env, continue], compile(Const(2)), compile(Const(3)))` in both Rust and Python confirms **zero `Save`/`Restore` instructions** are emitted for non-interfering operands!

## Exercise

Compute `needs` and `modifies` for compiling `(+ x 1)` vs `(+ (f x) y)`, showing exactly which register needs saving around `(f x)`.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
