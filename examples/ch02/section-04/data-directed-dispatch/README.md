# Data-directed dispatch table (`put` / `get` / `apply_generic`)

**Book concepts:** Section 2.4 (Subsections 2.4.2 and 2.4.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Data-directed programming decouples generic operations from concrete data representations using a two-dimensional `(operation, type_tags)` dispatch table, and contrasts additive type-package registration with message-passing objects.

## Implementation walkthrough

`DispatchTable` stores `(op, type_tags) -> handler` mappings populated independently by `install_rectangular_package` and `install_polar_package`. `apply_generic` extracts runtime tags from `TaggedDatum` arguments and invokes the registered procedure without editing any central `match`/`if` chain. `DerivRegistry` illustrates Exercise 2.73's additive symbolic differentiation plugins, while `make_from_mag_ang_mp` models message-passing dispatch where the object itself closes over its fields and accepts operation names.

## Complexity and correctness

Table lookup takes O(log K) in Rust (`BTreeMap`) and O(1) expected time in Python (`dict`) for `K` registered `(op, tags)` pairs. Adding a new representation requires only registering new table entries, whereas message passing makes adding new operations trivial without modifying existing constructors.

## Run it

```sh
cargo run --bin ch02-data-directed-dispatch
cargo test --bin ch02-data-directed-dispatch
python3 examples/ch02/section-04/data-directed-dispatch/main.py
python3 -m unittest examples/ch02/section-04/data-directed-dispatch/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add an `exponentiation` (`**`) rule to `DerivRegistry` without altering `DerivRegistry::deriv`, and add a third complex representation.

[Section introduction](../../../../chapters/ch02/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
