# 4.2 Variations on a Scheme: Lazy Evaluation · [Chapter 4](../README.md)

## The big idea

Why couldn't `unless(false, 1, 1/0)` guard against division by zero in Section 4.1? Because standard **applicative order** evaluates all arguments before entering a compound procedure.

Changing *one* small rule in `apply`—wrapping unevaluated argument expressions and environments in **Thunks**, and forcing **`actual_value`** only at `if` tests, primitive calls, and operator positions—turns the whole language **normal-order (call-by-need)**! Memoizing each thunk (`Unevaluated -> Evaluated`) even allows Church-encoded lazy `cons`/`car`/`cdr` to express infinite streams directly inside the guest language.

## What you should focus on

Focus on ****applicative vs normal order, `delay_it` vs `actual_value`, call-by-name vs memoized call-by-need thunks, and procedural lazy lists****. Every variation keeps the same guest syntax while systematically shifting *when* and *how* expressions reduce.

## Connecting the concept to Rust and Python

- **In Rust:** Thunks are modeled as `Value::Thunk(Rc<RefCell<ThunkState>>)` where `ThunkState` transitions once from `Delayed(Expr, Env)` to `Evaluated(Value)` inside `actual_value` (`lazy-evaluator`, `memoized-thunk`).
- **In Python:** A `@dataclass class Thunk` holds `(expr, env)` until forced once, then caches `.value` and clears `(expr, env)` cleanly—modeling call-by-need without needing `Rc<RefCell>`.
- **Key contrast:** Both implementations demonstrate why user-defined functions like `unless` and infinite `integers_from(1)` suddenly work inside the *guest* language without touching guest syntax.

## Subsections

- [4.2.1 Normal and Applicative Order](subsection-01/INTRO.md)
- [4.2.2 A Lazy Evaluator](subsection-02/INTRO.md)
- [4.2.3 Streams as Lazy Lists](subsection-03/INTRO.md)

## Runnable examples in this section

- [4.2 Call-by-need with a memoized thunk](../../../examples/ch04/section-02/memoized-thunk/README.md)
- [4.2 Conditional evaluation avoids unneeded computation](../../../examples/ch04/section-02/conditional-laziness/README.md)
- [4.2 Normal-order lazy evaluator with memoized thunks and lazy lists](../../../examples/ch04/section-02/lazy-evaluator/README.md)

## Check your understanding

1. At which three exact evaluation sites must `actual_value` force a thunk instead of passing it along unevaluated?
2. What happens to `(square (id_with_count 10))` under call-by-name (unmemoized) vs call-by-need (memoized)?

**Experiment:** Toggle thunk caching off in `memoized-thunk` or `lazy-evaluator` and measure how often nested expressions re-evaluate.

[Chapter 4 overview](../README.md) · [Edition notes](../../../docs/editions.md)
