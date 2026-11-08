# 1.3 Formulating Abstractions with Higher-Order Procedures

## The big idea

Extract a repeated computation pattern into a function that takes another function, such as a sum or fixed-point operator.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **functions as arguments/results, accumulation, fixed points, numerical methods**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Closures can borrow, move, or mutate captures; Fn, FnMut, and FnOnce encode the permitted uses.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

What is gained by naming a computation pattern rather than another numeric result?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
