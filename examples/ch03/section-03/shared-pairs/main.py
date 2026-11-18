from __future__ import annotations

from dataclasses import dataclass
import unittest
import weakref


@dataclass
class Node:
    value: int
    parent_ref: weakref.ref[Node] | None = None


class SharedPairsTests(unittest.TestCase):
    def test_weak_link_does_not_keep_parent_alive(self) -> None:
        parent: Node | None = Node(1)
        assert parent is not None
        child = Node(2, weakref.ref(parent))
        alias = parent
        self.assertEqual(alias.value, 1)
        self.assertIsNotNone(child.parent_ref and child.parent_ref())
        del alias
        del parent
        self.assertIsNone(child.parent_ref())  # type: ignore[misc]


if __name__ == "__main__":
    root = Node(10)
    child = Node(20, weakref.ref(root))
    parent = child.parent_ref() if child.parent_ref else None
    print(f"child {child.value} parent {parent.value if parent else None}")
