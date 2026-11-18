# 3.2.5 The CSE Machine

## What this subsection teaches

The **Control–Stash–Environment (CSE) machine** turns evaluation into three explicit runtime data structures:
- **Control (`C`):** stack of remaining expressions/instructions to process (top = next).
- **Stash (`S`):** stack of intermediate computed values waiting for operators or function calls.
- **Environment (`E`):** current lexical frame pointer (restored via `ResetEnv` instructions when leaving blocks or calls).

Walking `(C, S, E)` step by step reveals tail calls cleanly: when a call sits at the end of a block, no extra `ResetEnv` marker needs to stay on `C`.

*(Note: `3.2.5 The CSE Machine` is a Source Academy web-comparison supplement accompanying the 2022 JavaScript adaptation, rather than one of the 94 numbered subsections `1.1.1`–`5.5.7` in the printed 1996 Scheme or 2022 JavaScript volumes.)*

## Connecting the concept to Rust and Python

- **In Rust:** Implemented with three explicit vectors (`control: Vec<ControlItem>`, `stash: Vec<Val>`, `env: usize`) stepping inside a deterministic `loop` with trace logs.
- **In Python:** Mirrored with `control: list[...]`, `stash: list[...]`, and `env_id: int`—making each instruction step (`BinOpMarker`, `CallMarker`, `ResetEnv`) inspectable as plain Python objects.
- **Why compare both:** Run [`cse-environment-machine`](../../../../examples/ch03/section-02/cse-environment-machine/README.md): the exact same `(C, S, E)` state machine transitions run in both Rust and Python without relying on host call stacks.

## Exercise

Trace the `(C, S, E)` triple step by step while evaluating `(1 + 2) * 3` and calling a two-step closure.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
