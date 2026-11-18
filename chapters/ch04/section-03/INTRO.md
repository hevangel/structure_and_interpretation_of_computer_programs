# 4.3 Variations on a Scheme: Nondeterministic Computing · [Chapter 4](../README.md)

## The big idea

In **nondeterministic computing (`amb`)**, programs specify *constraints* rather than nested search loops: `(let ((a (amb 1 2 3))) (require (> a 1)) a)`.

Behind the scenes, an `amb` evaluator compiles every expression to take **two continuations**: `(succeed(value, next_fail), fail())`. When `require(false)` fails, it invokes the most recent `fail` continuation—undoing temporary `set!` mutations on its way back—to try the next branch cleanly (`try-again`).

## What you should focus on

Focus on ****`amb` & `require`, chronological backtracking, reversible `set!` vs `permanent-set!`, `if-fail`, logic puzzles, and continuation-passing evaluators (`succeed`/`fail`)****. Every variation keeps the same guest syntax while systematically shifting *when* and *how* expressions reduce.

## Connecting the concept to Rust and Python

- **In Rust:** Each analyzed node returns `Rc<dyn Fn(&Env, SucceedFn, FailFn)>`, and assignments push an undo closure onto the failure continuation chain so failing backtracks reset variables automatically (`amb-evaluator`).
- **In Python:** Closures `(env, succeed, fail)` capture backtracking state naturally with nested `def next_fail(): ...` functions—and also invite comparison with Python generator backtracking (`n-queens-search`).
- **Key contrast:** Watching `amb-evaluator` run Pythagorean triples, the 5-dwelling puzzle, and ambiguous grammar parsing shows how two small callbacks (`succeed`, `fail`) replace complex search loops in both Rust and Python.

## Subsections

- [4.3.1 Amb and Search](subsection-01/INTRO.md)
- [4.3.2 Nondeterministic Examples](subsection-02/INTRO.md)
- [4.3.3 Implementing Amb](subsection-03/INTRO.md)

## Runnable examples in this section

- [4.3 N-queens nondeterministic search](../../../examples/ch04/section-03/n-queens-search/README.md)
- [4.3 Backtracking constraint search: five dwellings](../../../examples/ch04/section-03/multiple-dwelling/README.md)
- [4.3 Nondeterministic continuation-passing amb evaluator](../../../examples/ch04/section-03/amb-evaluator/README.md)

## Check your understanding

1. Why must ordinary `set!` save its old value and restore it inside the `fail` continuation, while `permanent-set!` does not?
2. How does `amb` natural-language parsing reject dead-end parses and enumerate multiple valid parse trees on `try-again`?

**Experiment:** Add `permanent-set!` counter tracking across backtracking failures in `amb-evaluator` and collect every valid Pythagorean triple.

[Chapter 4 overview](../README.md) · [Edition notes](../../../docs/editions.md)
