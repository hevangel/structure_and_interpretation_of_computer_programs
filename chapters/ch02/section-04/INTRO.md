# 2.4 Multiple Representations for Abstract Data · [Chapter 2](../README.md)

## The big idea

Sometimes *two* valid internal representations coexist in the same program—like Cartesian `(real, imag)` complex numbers (fast addition) alongside Polar `(magnitude, angle)` (fast multiplication).

Three architectures manage multiple representations differently: (1) **explicit tagged `enum` dispatch**, (2) **data-directed dispatch** storing `(operation, type_tag) -> function` in an extensible registry (`put`/`get`/`apply_generic`), and (3) **message passing**, where each data value itself is an object/function answering operation names.

## What you should focus on

Focus on ****tagged data, open vs closed extensibility (the expression problem), data-directed `(op, tag)` registries, and message passing****. Across every example, test which callers remain completely untouched when you swap the internal representation.

## Connecting the concept to Rust and Python

- **In Rust:** `enum`s give closed exhaustive dispatch (easy to add new operations, hard to add new external types), whereas **traits** (`trait Complex`) and trait-object `BTreeMap<(Op, Tag), Rc<dyn Fn...>>` tables support open extension of new representations.
- **In Python:** Dictionaries keyed by `(op, type_tag)` implement `put`/`get`/`apply_generic` directly, while classes (`def magnitude(self)`) and closures natively express message passing via duck typing.
- **Key contrast:** Look at `data-directed-dispatch`: Rust traits check new implementations statically at compile time, while Python's dispatch dictionaries allow hot-plugging new types and operations dynamically at runtime.

## Subsections

- [2.4.1 Representing Complex Numbers](subsection-01/INTRO.md)
- [2.4.2 Tagged Data](subsection-02/INTRO.md)
- [2.4.3 Data-Directed Programming](subsection-03/INTRO.md)

## Runnable examples in this section

- [2.4 Complex numbers with two representations](../../../examples/ch02/section-04/complex-representations/README.md)
- [2.4 Data-directed dispatch table (put, get, apply_generic)](../../../examples/ch02/section-04/data-directed-dispatch/README.md)

## Check your understanding

1. Which axis is easy vs hard to extend under (a) tagged `match`, (b) data-directed table, and (c) message passing?
2. Why does `apply_generic` need a list of type tags `(tag1, tag2)` when dispatching binary operations?

**Experiment:** Register a new differentiation operator (e.g., division or exponentiation) into `data-directed-dispatch` without editing the core evaluator loop.

[Chapter 2 overview](../README.md) · [Edition notes](../../../docs/editions.md)
