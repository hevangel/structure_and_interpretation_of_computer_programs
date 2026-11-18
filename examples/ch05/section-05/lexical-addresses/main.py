from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Address:
    depth: int
    slot: int


def resolve(layouts: list[list[str]], name: str) -> Address | None:
    for depth, frame in enumerate(reversed(layouts)):
        if name in frame:
            return Address(depth=depth, slot=frame.index(name))
    return None


def access(values: list[list[int]], address: Address) -> int | None:
    index = len(values) - (address.depth + 1)
    if index < 0 or index >= len(values):
        return None
    frame = values[index]
    if address.slot < 0 or address.slot >= len(frame):
        return None
    return frame[address.slot]


def main() -> None:
    layouts = [["x", "y"], ["x"]]
    values = [[10, 42], [99]]
    addr = resolve(layouts, "y")
    assert addr is not None
    print(f"lexical address for y: {addr}, value = {access(values, addr)}")


class TestLexicalAddresses(unittest.TestCase):
    def test_shadowing_and_ancestor_frames(self) -> None:
        layout = [["x", "y"], ["x", "z"]]
        data = [[1, 2], [99, 8]]
        self.assertEqual(resolve(layout, "x"), Address(depth=0, slot=0))
        self.assertEqual(resolve(layout, "y"), Address(depth=1, slot=1))
        addr_y = resolve(layout, "y")
        addr_x = resolve(layout, "x")
        assert addr_y is not None and addr_x is not None
        self.assertEqual(access(data, addr_y), 2)
        self.assertEqual(access(data, addr_x), 99)
        self.assertIsNone(resolve(layout, "missing"))


if __name__ == "__main__":
    main()
