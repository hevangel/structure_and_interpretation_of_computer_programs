# Chapter 2: Building Abstractions with Data

Compound data lets a program talk about rational numbers, intervals, syntax trees, geometric pictures, and polynomials as cohesive units rather than loose bags of coordinates.

This chapter introduces **abstraction barriers** (constructors and selectors separating *what* a datum means from *how* it is stored), **closure under combination** (hierarchical sequences and trees), **symbolic manipulation**, **tagged data & data-directed dispatch**, and **generic arithmetic across type towers**.

## Rust and Python perspectives

- **In Rust:** Rust enforces abstraction barriers at compile time using modules, private struct fields, recursive `enum`s with `Box`/`Rc`, and exhaustive `match` dispatch or traits (`impl Trait` / `dyn Trait`).
- **In Python:** Python relies on classes (`@dataclass`), duck typing, special dunder methods (`__add__`, `__eq__`), nested lists/tuples, and convention (`_private` attributes) plus runtime dispatch dictionaries keyed by `(op, type_tags)`.
- **Comparing both:** Rust catches missing cases and illegal field access before execution, while Python makes dynamic data-directed dispatch tables and heterogeneous containers nearly effortless.

## Sections in this chapter

- [2.1 Introduction to Data Abstraction](section-01/INTRO.md)
- [2.2 Hierarchical Data and the Closure Property](section-02/INTRO.md)
- [2.3 Symbolic Data](section-03/INTRO.md)
- [2.4 Multiple Representations for Abstract Data](section-04/INTRO.md)
- [2.5 Systems with Generic Operations](section-05/INTRO.md)

## How to study

1. Read each section and subsection note before inspecting the source files.
2. Predict the output, invariants, and time/space complexity on paper.
3. Compare the Rust (`main.rs`) and Python (`main.py`) implementations in the viewer (`index.html` Side-by-Side view) and run both test suites.
4. Change one representation detail or invariant and observe which tests break.

The commentary here is independently written educational material, not a reproduction of the textbook. · [Edition notes](../../docs/editions.md) · [Coverage ledger](../../COVERAGE.md)
