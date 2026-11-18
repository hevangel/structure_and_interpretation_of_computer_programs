# 1.1.1 Expressions

## What this subsection teaches

An **expression** is a tree of operations and operands waiting to be reduced to a value. Grouping and operator precedence decide *which* sub-expression reduces first: `3 * (4 + 5)` builds an outer multiplication over an inner sum (`27`), whereas `3 * 4 + 5` nests multiplication inside addition (`17`).

Keeping syntax trees distinct from their final values explains both evaluation errors (type mismatches before running) and why evaluators treat expressions as structured data.

## Connecting the concept to Rust and Python

- **In Rust:** Every expression has a static type (e.g. `i64` vs `f64`), so `3 + 4.0` is rejected at compile time rather than implicitly coercing. Almost every construct (`if`, `{ ... }`) is itself an expression that yields a value.
- **In Python:** Expressions are dynamically typed and evaluated left-to-right; mixed `int + float` promotes automatically (`3 + 4.0 == 7.0`), while statements like assignments (`x = 5`) are distinct from expressions (unlike Rust blocks).
- **Why compare both:** Rust checks operand type compatibility on the syntax tree *before* execution, while Python checks types at runtime on the resulting objects.

## Exercise

Evaluate `3 * (4 + 5)` vs `3 * 4 + 5` as expression trees and trace the intermediate values in [`newton-sqrt`](../../../../examples/ch01/section-01/newton-sqrt/README.md).

[Parent section](../INTRO.md) · [Chapter 1 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
