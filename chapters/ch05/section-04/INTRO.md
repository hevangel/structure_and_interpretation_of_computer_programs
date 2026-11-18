# 5.4 The Explicit-Control Evaluator · [Chapter 5](../README.md)

## The big idea

Where did recursion come from in Chapter 4's `eval`/`apply`? From the *host* language's call stack! The **explicit-control evaluator (`eceval`)** strips away host recursion completely by rewriting `eval`/`apply` as a controller program on a **7-register machine**:
`exp` (expression), `env` (environment), `val` (result), `continue` (return label), `proc` (evaluated operator), `argl` (evaluated arguments), and `unev` (pending expressions).

Look at **`ev_sequence_last_exp`**: before evaluating the final expression in a procedure body, `eceval` jumps *without* saving `continue` or `env`—proving **proper tail recursion executes unbounded loops in O(1) stack depth!**

## What you should focus on

Focus on ****the 7 ECEVAL registers (`exp, env, val, continue, proc, argl, unev`), caller vs callee register saving, tail-call `ev_sequence`, and exact stack depth formulas****. Every machine model and compiler pass makes hidden runtime cost directly measurable.

## Connecting the concept to Rust and Python

- **In Rust:** Implemented as an explicit controller state machine driving named `RegisterTable` + `Stack` registers inside a flat `loop` (`explicit-control`)—running tail-recursive loops without touching Rust's call stack.
- **In Python:** Runs the exact same 7-register controller loop in Python, allowing `tail_sum(500)` to run cleanly at constant stack depth (`max_depth == 8`) even though recursive Python functions normally blow `RecursionError` cleanly around depth 1,000!
- **Key contrast:** Measuring `total_pushes = 32n - 16` and `max_depth = 5n + 3` for non-tail `fact(n)` vs constant depth for `tail_sum(n)` yields **identical numbers** in Rust and Python.

## Subsections

- [5.4.1 Explicit-Control Evaluator Core](subsection-01/INTRO.md)
- [5.4.2 Sequence and Tail Recursion](subsection-02/INTRO.md)
- [5.4.3 Conditionals and Declarations](subsection-03/INTRO.md)
- [5.4.4 Running Explicit Control](subsection-04/INTRO.md)

## Runnable examples in this section

- [5.4 Explicit-control expression evaluator](../../../examples/ch05/section-04/explicit-control/README.md)

## Check your understanding

1. Which single `save` you would *omit* before evaluating the last expression in a sequence turns a `O(n)`-space tail recursion into `O(1)` space?
2. Why does evaluating arguments `(f e1 e2 e3)` save `argl`, `proc`, and `unev` between operand evaluations?

**Experiment:** Verify in `explicit-control` that `tail_sum(10)` and `tail_sum(500)` record the **exact same `max_depth`**, while recursive `fact(n)` obeys `total_pushes = 32n - 16`.

[Chapter 5 overview](../README.md) · [Edition notes](../../../docs/editions.md)
