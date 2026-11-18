# 2.4.2 Tagged Data

## What this subsection teaches

If two helpers both represent numbers—a rectangular pair `(3.0, 4.0)` vs a polar pair `(5.0, 0.927)`—how do selectors tell them apart? Attach an explicit **type tag** (`"rectangular"` or `"polar"`) so generic selectors inspect the tag and strip it before computing.

Explicit tags turn raw data into **self-describing** values.

## Connecting the concept to Rust and Python

- **In Rust:** Rust's tagged union (`enum Complex { Rectangular(f64, f64), Polar(f64, f64) }`) bakes the tag cleanly into memory layout and guarantees every selector checks all variants at compile time.
- **In Python:** Python represents tagged data either as explicit `(tag, contents)` tuples/dataclasses (`Tagged(tag="polar", data=...)`) or class instances inspected via `type`/`isinstance`.
- **Why compare both:** Tagged enums give compile-time exhaustiveness in Rust, whereas explicit `(tag, payload)` records in Python mirror Scheme's dynamic tagging.

## Exercise

Trace what code has to change when adding a third coordinate variant under centralized tagged `match` dispatch.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
