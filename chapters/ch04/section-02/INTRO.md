# 4.2 Variations on a Scheme: Lazy Evaluation

## The big idea

Delay work using a thunk and optionally remember the first computed value.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **thunks, memoization, call-by-need versus call-by-value**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Closure ownership and RefCell can implement memoization; document when effects occur.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Does evaluating an expression zero, one, or multiple times change its observable result?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
