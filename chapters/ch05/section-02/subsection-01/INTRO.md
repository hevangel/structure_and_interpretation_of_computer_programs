# 5.2.1 The Machine Model

## What this subsection teaches

A simulator packages four cooperating structures:
- **Registers:** named state slots holding machine values
- **Stack:** LIFO frame history (with optional instrumentation)
- **Flag:** boolean result of the latest `test` instruction
- **Instruction sequence + `pc`:** linear assembled vector of executable steps.

Decoupling assembly (`build`) from execution (`start`) lets one assembled machine run repeatedly on fresh register inputs.

## Connecting the concept to Rust and Python

- **In Rust:** Encapsulated cleanly as `struct Machine { regs, stack, flag, ops, labels, code, ... }` returning `Result` cleanly on bad registers or infinite-loop bounds (`register-simulator`).
- **In Python:** Encapsulated cleanly as `@dataclass class Machine` with dictionary-backed registers, list stack, and explicit exception reporting on invalid targets.
- **Why compare both:** Keeping machine state separated from instruction list (`code`) allows resetting registers and re-running assembled `fact`/`fib` machines seamlessly in both languages.

## Exercise

List every mutable field inside `Machine` that changes during a single `factorial` run vs stays read-only.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
