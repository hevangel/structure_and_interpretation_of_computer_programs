# 5.2 A Register-Machine Simulator · [Chapter 5](../README.md)

## The big idea

Designing controller diagrams on paper is great—running them is better! A **register-machine simulator** parses controller text (`assign`, `test`, `branch`, `goto`, `save`, `restore`, `perform`), resolves symbolic branch labels into instruction indices via a **two-pass assembler**, wires primitive operations (`op`), and instruments every execution with **instruction counts**, **stack `total_pushes` / `max_depth`**, and tracing hooks.

## What you should focus on

Focus on ****named `RegisterTable`, two-pass label assembly, `pc` dispatch & `goto(reg)`, stack statistics (`total_pushes`, `max_depth`), and execution tracing****. Every machine model and compiler pass makes hidden runtime cost directly measurable.

## Connecting the concept to Rust and Python

- **In Rust:** Uses strongly typed `Instr` / `Source` enums, `BTreeMap<String, usize>` label tables, `Vec<Val>` stacks with push/depth counters, and bounded step execution (`register-simulator`, `two-pass-assembler`).
- **In Python:** Mirrors the exact same machine model (`Register`, `Stack` statistics, two-pass label table, `BTreeMap`-ordered primitives) in readable dataclasses.
- **Key contrast:** Both Rust and Python simulators compute identical push/depth counts for recursive `fact(n)` (`2n - 2` pushes and `max_depth` linear in `n`) and tree-recursive `fib(n)`.

## Subsections

- [5.2.1 The Machine Model](subsection-01/INTRO.md)
- [5.2.2 The Assembler](subsection-02/INTRO.md)
- [5.2.3 Instruction Execution Functions](subsection-03/INTRO.md)
- [5.2.4 Monitoring Machine Performance](subsection-04/INTRO.md)

## Runnable examples in this section

- [5.2 Small register-machine simulator](../../../examples/ch05/section-02/register-simulator/README.md)
- [5.2 Two-pass assembler with symbolic jump labels](../../../examples/ch05/section-02/two-pass-assembler/README.md)

## Check your understanding

1. Why does assembling jumps (`goto label`) require two passes (or backpatching) over the controller list?
2. How do `total_pushes` and `max_depth` expose algorithmic differences between two controllers computing the same output?

**Experiment:** Verify the analytical `fact` and `fib` push counts printed by `register-simulator` and trigger the unknown-label/step-limit guards in `two-pass-assembler`.

[Chapter 5 overview](../README.md) · [Edition notes](../../../docs/editions.md)
