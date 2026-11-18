# 5.1.1 Register-Machine Description Language

## What this subsection teaches

A register-machine design splits into two explicit views:
1. **Data paths:** named registers (`a`, `b`, `t`), constant inputs, primitive tests (`= b 0`), and operations (`rem a b`).
2. **Controller:** linear list of `assign`, `test`, `branch`, and `goto` instructions stepped sequentially by a program counter.

## Connecting the concept to Rust and Python

- **In Rust:** Controller steps are modeled as `enum Instr` variants reading/writing named `RegisterTable` cells cleanly.
- **In Python:** Mirrors instructions with `@dataclass(frozen=True)` records mutating a `dict[str, Value]` register table (`register-gcd`, `register-simulator`).
- **Why compare both:** Representing the data path cleanly as plain data structs makes register transfers identical and inspectable in both Rust and Python (`(a, b) <- (b, a % b)`).

## Exercise

Translate `gcd(a, b)` into explicit `test`, `branch`, `assign`, and `goto` instructions without using a `while` keyword.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
