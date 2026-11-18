# Picture language: compositional images

**Book concepts:** Section 2.2 (Subsection 2.2.4). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Create larger pictures from small pictures using transformations such as placing one beside another or above another.

## Implementation walkthrough

An ASCII picture is a vector of equal-width rows. Combinators return new pictures, illustrating closure under composition without needing a graphics library.

## Complexity and correctness

Combining pictures copies all output characters, O(total area). A production painter would defer drawing and transform coordinates.

## Run it

```sh
cargo run --bin ch02-picture-composition
cargo test --bin ch02-picture-composition
python3 examples/ch02/section-02/picture-composition/main.py
python3 -m unittest examples/ch02/section-02/picture-composition/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Implement a painter that mirrors a picture horizontally.

[Section introduction](../../../../chapters/ch02/section-02/INTRO.md) · [Coverage](../../../../COVERAGE.md)
