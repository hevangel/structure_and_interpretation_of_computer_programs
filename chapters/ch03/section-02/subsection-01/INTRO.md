# 3.2.1 Rules for Evaluation

## What this subsection teaches

The **environment model** replaces textual substitution with two explicit mechanical rules:
1. **Evaluate a name** by walking outward from the innermost environment frame along parent links until the first frame containing that name is found.
2. **Evaluate a function definition** by packing *(parameter list, body, current environment pointer)* into a closure.

## Connecting the concept to Rust and Python

- **In Rust:** Modeled cleanly as an explicit `Vec<Frame>` arena (`Frame { bindings: BTreeMap<String, Val>, parent: Option<usize> }`) where `lookup(env_id, name)` walks `parent` links.
- **In Python:** A dictionary-backed `Frame(bindings, parent)` class—and Python's own execution model literally follows this rule via frame dicts and `LEGB` lookup.
- **Why compare both:** See [`lexical-environments`](../../../../examples/ch03/section-02/lexical-environments/README.md): representing frames explicitly shows how lookup terminates on the nearest enclosing binding.

## Exercise

Trace looking up `(x, y)` through three nested frames where `y` is bound in frame 0 and shadowed in frame 2.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
