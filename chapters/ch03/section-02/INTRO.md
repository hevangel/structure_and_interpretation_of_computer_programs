# 3.2 The Environment Model of Evaluation · [Chapter 3](../README.md)

## The big idea

When names can be reassigned, evaluation needs a mechanical model of *where* bindings live: the **environment model**. An **environment** is a linked chain of **frames**, each holding a table of local bindings and a pointer to its enclosing parent frame.

A function object (closure) is simply a pair: **(code + pointer to the environment where it was created)**. Calling it allocates a new frame whose parent is the closure's *definition* environment—not the caller's environment!

## What you should focus on

Focus on ****linked environment frames, closure capture rules, variable shadowing vs mutation (`set!`), mutual internal definitions, and the explicit CSE step machine****. Always ask which parts of a system depend on *when* operations run versus *what* values they compute.

## Connecting the concept to Rust and Python

- **In Rust:** Rust's own lexical scopes compile down to stack slots and captures at compile time; to study runtime environment models (or build an interpreter), we allocate frames explicitly in `Vec<Frame>` arenas or `Rc<RefCell<Frame>>` chains.
- **In Python:** Python's runtime literally *is* an environment model: functions carry `__closure__` cells pointing to enclosing frames, `x = val` binds locally while `nonlocal x` mutates the enclosing frame slot cleanly.
- **Key contrast:** Comparing `lexical-environments` and `cse-environment-machine` in Rust (explicit frame indices) vs Python clarifies the exact difference between shadowing (`let`) and updating (`set!`).

## Subsections

- [3.2.1 Rules for Evaluation](subsection-01/INTRO.md)
- [3.2.2 Applying Functions](subsection-02/INTRO.md)
- [3.2.3 Frames and Local State](subsection-03/INTRO.md)
- [3.2.4 Internal Definitions](subsection-04/INTRO.md)
- [3.2.5 The CSE Machine](subsection-05/INTRO.md)

## Runnable examples in this section

- [3.2 Lexical scope and environment frames](../../../examples/ch03/section-02/lexical-environments/README.md)
- [3.2 Environment model mutation and CSE step machine](../../../examples/ch03/section-02/cse-environment-machine/README.md)

## Check your understanding

1. Draw the frames created by calling `acc1 = make_account(100)` and `acc2 = make_account(100)`, then calling `acc1(40)`.
2. Why can mutually recursive inner definitions not simply bind sequentially without an uninitialized placeholder or TDZ check?

**Experiment:** Trace `cse-environment-machine` step by step to watch Control, Stash, and Environment frames push and pop across a closure call.

[Chapter 3 overview](../README.md) · [Edition notes](../../../docs/editions.md)
