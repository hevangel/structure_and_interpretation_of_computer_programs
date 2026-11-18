# 5.5.6 Lexical Addressing

## What this subsection teaches

In an interpreted environment, looking up variable `y` walks frames comparing string names (`"y" == key`) on every execution—slow `O(frame_depth * bindings)` string hashing/scanning.

Since variable scopes are fixed **statically at compile time**, a compiler tracks compile-time environment layouts `ctenv = [["x", "y"], ["x"]]` and resolves `y` once into a direct **`LexicalAddress(depth=1, slot=1)`**! At runtime, `lexical_lookup(env, 1, 1)` jumps one parent pointer and reads array index `1` in `O(1)` time per hop with zero string comparisons.

## Connecting the concept to Rust and Python

- **In Rust:** Demonstrated cleanly standalone in [`lexical-addresses`](../../../../examples/ch05/section-05/lexical-addresses/README.md) and integrated right into [`register-compiler`](../../../../examples/ch05/section-05/register-compiler/README.md) (`Op::LookupLexical(depth, slot)`).
- **In Python:** Python mirrors the exact compile-time frame-layout resolver (`Address(depth, slot)`) and fast slot access without `dict` string lookup at runtime.
- **Why compare both:** Static `(depth, slot)` addressing turns variable lookup from dynamic name search into fixed pointer+offset arithmetic—precisely how CPython's `LOAD_FAST`/`LOAD_DEREF` bytecode works under the hood!

## Exercise

Given compile-time layouts `[["a", "b"], ["b", "c"], ["d"]]`, compute `(depth, slot)` addresses for `"a"`, `"b"`, `"c"`, and `"d"` from the innermost body.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
