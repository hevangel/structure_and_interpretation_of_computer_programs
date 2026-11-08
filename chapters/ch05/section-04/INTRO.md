# 5.4 The Explicit-Control Evaluator

## The big idea

Replace implicit host recursion with an explicit machine stack and control state.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **explicit stack, continuation states, evaluation control**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

This reveals the mechanism behind function calls and expression evaluation.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

What information must be saved before entering a nested computation?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
