# 5.1.3 Subroutines

## What this subsection teaches

If two places in a large controller both need `gcd`, duplicating the instructions wastes space—sharing *one* `gcd` block introduces a classic dilemma: after `gcd` finishes, **where does `goto` jump back to?**

Storing the return label inside a dedicated **`continue` register** before jumping (`assign continue (label after-gcd-1); goto gcd-loop`), then ending the subroutine with **`goto (reg continue)`**, lets callers share one subroutine safely—as long as nested calls save `continue` on a stack!

## Connecting the concept to Rust and Python

- **In Rust:** `Target::Reg("continue")` allows indirect dynamic jumps cleanly alongside static `Target::Label("...")` branches (`register-simulator`).
- **In Python:** Indirect register jumps look up the label PC stored as `ValLabel(pc)` inside `regs["continue"]` cleanly.
- **Why compare both:** Explicitly assigning and jumping through `continue` shows what CPU `CALL`/`RET` instructions automate behind the scenes.

## Exercise

Trace two sequential calls to a shared `gcd` subroutine using `continue` (`after_1`, `after_2`) and verify neither overwrites the other.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
