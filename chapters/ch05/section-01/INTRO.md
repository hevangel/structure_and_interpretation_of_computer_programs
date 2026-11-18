# 5.1 Designing Register Machines · [Chapter 5](../README.md)

## The big idea

High-level loops and recursive procedures compile down to two tiny pieces: a **data path** (registers holding values, operations `+`/`=`/`%`, buttons/wires between them) and a **controller** (sequential instructions branching via `test`/`branch`/`goto`).

When a procedure calls *itself*—like recursive factorial or tree-recursive Fibonacci—a finite set of registers isn't enough on its own: adding a single explicit **stack (`save`/`restore`)** records pending intermediate values and `continue` return labels cleanly.

## What you should focus on

Focus on ****data paths vs controllers, subroutines & `continue` return registers, and explicit recursion stacks (`save`/`restore`)****. Every machine model and compiler pass makes hidden runtime cost directly measurable.

## Connecting the concept to Rust and Python

- **In Rust:** Explicit structs (`struct Machine { a, b, steps }`) or `Vec<u64>` stacks (`explicit-stack-factorial`, `register-gcd`) step cleanly in a flat `loop` with zero reliance on native call stacks.
- **In Python:** `@dataclass` register banks and plain Python `list` stacks (`append`/`pop`) trace every transfer `(a, b) <- (b, a % b)` or stack push depth explicitly.
- **Key contrast:** Both Rust and Python strip away host-language recursion here—revealing the exact stack depth (`n - 1` pushes for `factorial(n)`) that ordinary nested function calls hide.

## Subsections

- [5.1.1 Register-Machine Description Language](subsection-01/INTRO.md)
- [5.1.2 Abstraction in Machine Design](subsection-02/INTRO.md)
- [5.1.3 Subroutines](subsection-03/INTRO.md)
- [5.1.4 A Stack for Recursion](subsection-04/INTRO.md)
- [5.1.5 Instruction Summary](subsection-05/INTRO.md)

## Runnable examples in this section

- [5.1 Design a register-machine controller](../../../examples/ch05/section-01/register-gcd/README.md)
- [5.1 Factorial with an explicit control stack](../../../examples/ch05/section-01/explicit-stack-factorial/README.md)

## Check your understanding

1. Why can iterative `gcd(a, b)` run with zero stack saves while recursive `factorial(n)` requires saving `n` and `continue` before descending?
2. Why does reusing one shared `gcd` controller from two call sites require a `continue` return-address register?

**Experiment:** Record `max_depth` vs `n` across `explicit-stack-factorial` and verify `factorial(n)` pushes `n - 1` frames before multiplying back up.

[Chapter 5 overview](../README.md) · [Edition notes](../../../docs/editions.md)
