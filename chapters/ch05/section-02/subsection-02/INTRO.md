# 5.2.2 The Assembler

## What this subsection teaches

Controllers are written with symbolic labels (`fact-loop:`, `goto base-case`), yet execution advances an integer index `pc = 0, 1, 2, ...`. Because `goto` often references labels declared *further down* the file (**forward references**), an assembler works in **two passes**:
1. Pass 1 strips labels while mapping each `label_name -> current instruction index` (catching duplicate labels).
2. Pass 2 translates instructions, replacing every symbolic label with its resolved `usize`/`int` address (catching undefined labels).

## Connecting the concept to Rust and Python

- **In Rust:** Implemented cleanly in [`two-pass-assembler`](../../../../examples/ch05/section-02/two-pass-assembler/README.md) and [`register-simulator`](../../../../examples/ch05/section-02/register-simulator/README.md) using `BTreeMap`/`HashMap` label lookup.
- **In Python:** Python passes over the source instruction list identically: first building `labels: dict[str, int]`, then resolving jumps cleanly in pass 2.
- **Why compare both:** Both Rust and Python two-pass assemblers turn symbolic branches (`branch(label)`) into `O(1)` array jumps at runtime.

## Exercise

Trace Pass 1 (label table) and Pass 2 (`Op` vector) on the loop controller in [`two-pass-assembler`](../../../../examples/ch05/section-02/two-pass-assembler/README.md).

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
