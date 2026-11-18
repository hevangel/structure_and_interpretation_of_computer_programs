# 5.3.2 Garbage Collection

## What this subsection teaches

Once `free` hits the end of vector memory, **garbage collection** identifies what is still reachable from machine roots (`root`, registers, stack):
- **Mark–Sweep (`mark-sweep-gc`):** marks live graph nodes via DFS/pointer reversal, then sweeps unmarked slots.
- **Cheney Stop-and-Copy (`stop-and-copy-gc`):** uses two pointers (`free` and `scan`) in working half-space `new_cars`/`new_cdrs`! Relocate roots to `new[0..]`, advance `scan` up toward `free` relocating `new_cars[scan]` and `new_cdrs[scan]`, and write **`BrokenHeart(new_idx)`** into copied old cells so shared pointers and **cycles** (`set_cdr(a, a)`) redirect cleanly in `O(live_memory)` time without recursion!

## Connecting the concept to Rust and Python

- **In Rust:** Implemented cleanly in both [`mark-sweep-gc`](../../../../examples/ch05/section-03/mark-sweep-gc/README.md) and [`stop-and-copy-gc`](../../../../examples/ch05/section-03/stop-and-copy-gc/README.md) with multi-cycle GC flip tests verifying shared DAGs and cycles.
- **In Python:** Uses the exact same iterative `scan < free` Cheney queue loop over Python lists—showing how non-recursive copying collectors operate with `O(1)` auxiliary space.
- **Why compare both:** Watching Cheney's `scan` pointer chase `free` across consecutive `gc_flip` cycles in both Rust and Python demystifies how production copying GCs eliminate fragmentation and survive cyclic graphs.

## Exercise

Trace `scan` and `free` step by step while copying a 2-node cyclic pair `[a -> b -> a]` plus one unreachable cell `dead`.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
