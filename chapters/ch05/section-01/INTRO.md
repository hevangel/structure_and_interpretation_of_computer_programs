# 5.1 Designing Register Machines

## The big idea

Describe a mathematical operation as a sequence of elementary register transfers.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **data paths, controller, primitive operations, stack**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Represent machine instructions as enum variants; keep the machine state independent of the program text.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

Which temporary values must exist at every step of a computation?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
