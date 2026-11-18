# 2.5.2 Combining Different Types

## What this subsection teaches

Real programs mix types constantly (`Int(3) + Rat(1, 2)` or `Rat(3, 2) + Complex(1, 0)`). Writing explicit converters between every `(A, B)` pair would take `O(N^2)` rules for `N` types.

Instead, arrange types in a linear **tower** (`Int -> Rat -> Real -> Complex`): define one upward step **`raise`** (`O(N)` converters) to align mixed arguments to the higher type, plus downward **`project`** and **`drop`** so exact results like `Complex(5.0, 0.0)` simplify cleanly back down to `Int(5)` whenever `raise(project(x)) == x`!

## Connecting the concept to Rust and Python

- **In Rust:** An explicit `Level` rank (`Int < Rat < Real < Complex`) makes `raise_to(val, target_level)` loop safely until levels match, while `drop(val)` tests lossless projection step by step.
- **In Python:** Python mirrors the `raise`/`project`/`drop` loop with clean dataclasses (`Int`, `Rat`, `Real`, `Complex`)—and illustrates how Python's own `bool -> int -> float -> complex` numeric tower works underneath.
- **Why compare both:** Both implementations in [`numeric-tower-coercion`](../../../../examples/ch02/section-05/numeric-tower-coercion/README.md) prove `drop(Complex(Real(6.0), Real(0.0))) == Int(6)` while preserving genuine complex parts.

## Exercise

Trace raising `Int(2)` + `Complex(Rat(3, 2), Int(4))`, then dropping `Complex(Real(5.0), Real(0.0))`.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
