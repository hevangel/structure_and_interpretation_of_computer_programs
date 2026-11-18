# Environment model mutation and CSE step machine

**Book concepts:** Section 3.2 (Subsections 3.2.1–3.2.5). **Edition applicability:** 2022-javascript (with shared lexical environment concepts from 1996-scheme).

## What this demonstrates

The environment model explains how closures retain references to mutable parent frames (`make_withdraw` instances stay independent while sibling closures share outer mutations), and the Control–Stash–Environment (`C, S, E`) machine makes each step of evaluation, frame extension, and environment restoration explicit.

## Implementation walkthrough

`CseMachine` stores frames in a mutable arena (`frames: Vec<Frame>` / `list[Frame]`) indexed by `EnvId`, so closures carry an environment pointer (`env`) rather than cloning maps. Every `step()` pops one item from the **Control** stack, updates the **Stash** value stack or **Environment** register, and records a trace entry.

## Complexity and correctness

Each variable lookup or assignment traverses at most `O(depth)` enclosing frames. Extending a frame is `O(params)`, and control/stash transitions are `O(1)` per machine step.

## Run it

```sh
cargo run --bin ch03-cse-environment-machine
cargo test --bin ch03-cse-environment-machine
python3 examples/ch03/section-02/cse-environment-machine/main.py
python3 -m unittest examples/ch03/section-02/cse-environment-machine/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add `If(pred, cons, alt)` control-flow instructions to `CseMachine` and inspect how branch instructions replace only the needed branch on the Control stack.

[Section introduction](../../../../chapters/ch03/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
