# 3.3 Modeling with Mutable Data

## The big idea

Implement a mutable FIFO queue and state its representation invariants.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **queues, tables, cycles, mutation, sharing**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Prefer VecDeque for a real application; implement a queue here to study interface boundaries.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

How can two names for the same mutable object make local reasoning fail?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
