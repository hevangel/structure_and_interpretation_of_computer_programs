# 1.1 The Elements of Programming · [Chapter 1](../README.md)

## The big idea

Every programming language gives you primitive values and operations, ways to combine them into compound expressions and functions, and ways to name abstractions so details stay hidden. Putting these pieces together lets small local rules—like Newton's iterative tangent approximation for square and cube roots—solve continuous mathematical problems cleanly.

Just as important, two expressions that look nearly identical can behave differently depending on **evaluation order** (eager applicative order vs lazy normal order) and **lexical scope** (whether inner helper definitions shadow or capture outer parameters cleanly).

## What you should focus on

Focus on ****expressions vs bindings, evaluation order (applicative vs normal), conditional short-circuiting, Newton iteration, and lexical block scope****. The core discipline is separating *what* a computational method guarantees (its contract and invariant) from *how* any one helper is coded.

## Connecting the concept to Rust and Python

- **In Rust:** `let` introduces immutable value bindings by default, `fn` defines statically typed functions, `if`/`else` is a value-producing expression that evaluates only the chosen branch, and block scopes (`{ ... }` or nested `fn`s) let helpers keep internal state private.
- **In Python:** Names are bound dynamically with `=` (`def` creates a function object bound in the current frame), conditional expressions (`a if pred else b`) short-circuit identically, and nested `def` closures capture enclosing parameters lexically.
- **Key contrast:** Both Rust and Python evaluate function arguments **eagerly (applicative order)** before entering the function body, so an unused diverging argument hangs unless wrapped in a conditional, closure, or thunk.

## Subsections

- [1.1.1 Expressions](subsection-01/INTRO.md)
- [1.1.2 Naming and the Environment](subsection-02/INTRO.md)
- [1.1.3 Evaluating Combinations](subsection-03/INTRO.md)
- [1.1.4 Compound Procedures and Functions](subsection-04/INTRO.md)
- [1.1.5 The Substitution Model](subsection-05/INTRO.md)
- [1.1.6 Conditionals and Predicates](subsection-06/INTRO.md)
- [1.1.7 Square Roots by Newton's Method](subsection-07/INTRO.md)
- [1.1.8 Black-Box Abstractions](subsection-08/INTRO.md)

## Runnable examples in this section

- [1.1 Square roots by iterative improvement](../../../examples/ch01/section-01/newton-sqrt/README.md)
- [1.1 Cube roots by Newton iteration](../../../examples/ch01/section-01/cube-root-newton/README.md)
- [1.1 Evaluation orders and lexical block scope](../../../examples/ch01/section-01/evaluation-orders/README.md)

## Check your understanding

1. How do you guarantee a Newton iteration stopping rule works for both tiny inputs (`1e-12`) and huge inputs (`1e12`)?
2. Why does `if p() == 0` terminate in normal order when passing an infinite loop `loop_forever()` as the second argument, yet hang in default Rust/Python calls?

**Experiment:** Tighten or loosen the relative tolerance in `newton-sqrt` or toggle between `eval_applicative` and `eval_normal` in `evaluation-orders` and check which assertions fail.

[Chapter 1 overview](../README.md) · [Edition notes](../../../docs/editions.md)
