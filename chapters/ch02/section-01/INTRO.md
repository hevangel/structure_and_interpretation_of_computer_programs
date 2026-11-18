# 2.1 Introduction to Data Abstraction · [Chapter 2](../README.md)

## The big idea

Compound data isolates *how* something is represented from *what operations* callers perform on it. Just as black-box functions hide their internal steps behind inputs and outputs, **data abstraction** separates code using constructors and selectors (`make_rat`, `numer`, `denom`) cleanly across an **abstraction barrier**.

If a constructor normalizes signs and divides out `gcd`, or if a rectangle switches from `(width, height)` to diagonal endpoints, callers above the barrier continue working unchanged.

## What you should focus on

Focus on ****abstraction barriers, representation invariants (canonical rationals), procedural pairs (`What Is Data?`), and interval error dependency****. Across every example, test which callers remain completely untouched when you swap the internal representation.

## Connecting the concept to Rust and Python

- **In Rust:** Private struct fields inside a module (`struct Rat { n: i64, d: i64 }`) make bypassing the constructor a compile-time error; returning `impl Fn` or traits shows how data can be encoded purely as functions.
- **In Python:** Uses `@dataclass(frozen=True)` with `__post_init__` normalization or convention-based `_numer` attributes, plus closures returning `(car, cdr)` selectors for Church numerals and functional pairs.
- **Key contrast:** Rust enforces module privacy boundaries statically at compile time, while Python relies on properties, frozen dataclasses, and constructor invariants enforced at initialization.

## Subsections

- [2.1.1 Rational Arithmetic](subsection-01/INTRO.md)
- [2.1.2 Abstraction Barriers](subsection-02/INTRO.md)
- [2.1.3 What Is Data?](subsection-03/INTRO.md)
- [2.1.4 Interval Arithmetic](subsection-04/INTRO.md)

## Runnable examples in this section

- [2.1 Rational numbers as an abstract data type](../../../examples/ch02/section-01/rational-numbers/README.md)
- [2.1 Arithmetic on intervals](../../../examples/ch02/section-01/interval-arithmetic/README.md)
- [2.1 Church numerals as higher-order functions](../../../examples/ch02/section-01/church-numerals/README.md)
- [2.1 Geometric segments and abstract rectangles](../../../examples/ch02/section-01/geometric-segments/README.md)
- [2.1 Pairs represented as functions](../../../examples/ch02/section-01/functional-pairs/README.md)

## Check your understanding

1. Which lines of `rational-numbers` must change if we decide to normalize during `numer()` instead of `make_rat()`?
2. Why can `R_1 * R_2 / (R_1 + R_2)` and `1 / (1/R_1 + 1/R_2)` disagree in interval arithmetic?

**Experiment:** Move `gcd` reduction from `make_rat` into the accessors `numer`/`denom` and confirm no caller test fails.

[Chapter 2 overview](../README.md) · [Edition notes](../../../docs/editions.md)
