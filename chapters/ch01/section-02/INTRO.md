# 1.2 Procedures and the Processes They Generate

## The big idea

Compare recursive and iterative processes by counting stack frames and steps. Use an invariant to explain each loop.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **linear recursion, iteration, orders of growth, exponentiation, fibonacci, gcd, primality**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Rust does not guarantee tail-call elimination. An iterative *process* in Scheme may still require a loop in portable Rust.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Why can two algorithms computing the same function use radically different resources?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
