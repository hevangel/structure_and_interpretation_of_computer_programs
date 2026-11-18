# 2.2 Hierarchical Data and the Closure Property · [Chapter 2](../README.md)

## The big idea

An operation has the **closure property** (in abstract algebra's sense) if combining things of a type produces another thing of that *same* type. Because pairs can hold other pairs, lists nest into arbitrary **trees**; and because picture combinators (`beside`, `below`, `rotate90`) produce new **painters**, small combinators build fractal patterns like `corner_split` and `square_limit`.

Meanwhile, normalizing traversals into a shared **sequence interface** (`map`, `filter`, `flatmap`, `fold_right`/`fold_left`) turns ad-hoc loops into composable dataflow pipelines.

## What you should focus on

Focus on ****structural closure, recursive trees, sequence pipelines (`map`/`filter`/`flatmap`), fold associativity (`fold_left` vs `fold_right`), and geometric picture frames****. Across every example, test which callers remain completely untouched when you swap the internal representation.

## Connecting the concept to Rust and Python

- **In Rust:** Because every Rust type needs a compile-time known size, recursive trees and sequences wrap subnodes in `Box<T>`, `Vec<T>`, or `Rc<T>`, and sequence pipelines map to zero-cost `Iterator` adaptors.
- **In Python:** Python lists and tuples naturally nest to arbitrary depth (`[1, [2, 3]]`), generator comprehensions express `map`/`filter`/`flatmap` concisely, and higher-order painters are plain functions `(Frame, Canvas) -> None`.
- **Key contrast:** Python mirrors Scheme's dynamically nested lists directly without heap-wrapper types; Rust forces ownership and recursion indirection (`Box`/`Rc`) to be explicit in every node definition.

## Subsections

- [2.2.1 Representing Sequences](subsection-01/INTRO.md)
- [2.2.2 Hierarchical Structures](subsection-02/INTRO.md)
- [2.2.3 Sequences as Conventional Interfaces](subsection-03/INTRO.md)
- [2.2.4 The Picture Language](subsection-04/INTRO.md)

## Runnable examples in this section

- [2.2 Recursive tree folds](../../../examples/ch02/section-02/recursive-trees/README.md)
- [2.2 Sequence processing and matrix multiplication](../../../examples/ch02/section-02/matrix-operations/README.md)
- [2.2 Picture language: compositional images](../../../examples/ch02/section-02/picture-composition/README.md)
- [2.2 Power set by recursive decomposition](../../../examples/ch02/section-02/powerset/README.md)
- [2.2 Recursive mobiles and torque balance](../../../examples/ch02/section-02/balanced-mobiles/README.md)
- [2.2 Horner's rule as a fold](../../../examples/ch02/section-02/horner-evaluation/README.md)
- [2.2 Prime-sum pairs using nested enumeration](../../../examples/ch02/section-02/prime-sum-pairs/README.md)
- [2.2 Left and right folds are not interchangeable](../../../examples/ch02/section-02/fold-direction/README.md)
- [2.2 Geometric frame-mapped picture language and painters](../../../examples/ch02/section-02/frame-painters/README.md)
- [2.2 Binary mobiles, fold direction, and nested mappings](../../../examples/ch02/section-02/nested-mappings-mobiles/README.md)

## Check your understanding

1. For what binary operations `op` does `fold_left(op, init, xs) == fold_right(op, init, xs)` always hold?
2. How does separating a **Frame** `(origin, edge1, edge2)` from unit-square normalized **Segments** make rotation and splitting pure linear algebra?

**Experiment:** Compare `fold_left` vs `fold_right` with subtraction (`-`) and non-associative division (`/`), then compose custom painters in `frame-painters`.

[Chapter 2 overview](../README.md) · [Edition notes](../../../docs/editions.md)
