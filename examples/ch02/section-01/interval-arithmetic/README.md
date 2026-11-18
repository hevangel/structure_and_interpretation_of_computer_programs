# Interval arithmetic and algebraic dependency

**Book concepts:** Section 2.1 (Subsection 2.1.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Interval arithmetic tracks uncertainty bounds through addition, subtraction, multiplication, and division, and exposes the dependency problem: algebraically equivalent formulas (`par1` vs `par2`) yield different interval widths when an uncertain variable appears more than once.

## Implementation walkthrough

`Interval` stores `[low, high]` bounds with `make_center_percent` constructors, `add`/`sub`/`mul`/`div`, a zero-straddling guard in `reciprocal`, and a 9-case endpoint sign analysis `mul_fast` (Exercise 2.11). `par1(r1, r2) = (r1 * r2) / (r1 + r2)` treats multiple occurrences of `r1` and `r2` as independent variables, whereas `par2(r1, r2) = 1 / (1/r1 + 1/r2)` references each resistor once and achieves tighter bounds.

## Complexity and correctness

All arithmetic operations run in O(1) time. `mul_fast` matches the 4-product exhaustive `mul` across all sign combinations while performing at most two multiplications when neither operand spans zero.

## Run it

```sh
cargo run --bin ch02-interval-arithmetic
cargo test --bin ch02-interval-arithmetic
python3 examples/ch02/section-01/interval-arithmetic/main.py
python3 -m unittest examples/ch02/section-01/interval-arithmetic/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Compute `r1 / r1` and `r1 / r2` for narrow vs. wide tolerances and measure how repeated appearances widen the result interval.

[Section introduction](../../../../chapters/ch02/section-01/INTRO.md) · [Coverage](../../../../COVERAGE.md)
