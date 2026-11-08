# 5.2 A Register-Machine Simulator

## The big idea

Write a tiny interpreter for machine instructions and trace its register evolution.

## What you should already know

You should be comfortable with basic functions, conditionals, and data structures. For this section, focus on **instruction encoding, labels, execution loop, trace**. The important move is to separate the *interface* of a computational idea from the particular mechanism that realizes it.

## Translating the idea into Rust

Use bounded execution and explicit errors rather than trusting arbitrary bytecode.

Study the nearby examples, identify their data representation, and write down one invariant. Compare your prediction with the test assertions. Some original Scheme ideas are dynamically typed or depend on tail-call behavior; the Rust port deliberately highlights those differences rather than hiding them.

## Check your understanding

What separates the machine specification from the program running on it?

**Experiment:** change the data representation or evaluation strategy without changing the observable API. Which tests survive?

[Back to book and edition notes](../../../docs/editions.md)
