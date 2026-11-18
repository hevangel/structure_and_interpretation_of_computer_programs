from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Node:
    value: int
    left: Node | None = None
    right: Node | None = None


def insert(tree: Node | None, value: int) -> Node:
    if tree is None:
        return Node(value)
    if value < tree.value:
        return Node(tree.value, insert(tree.left, value), tree.right)
    if value > tree.value:
        return Node(tree.value, tree.left, insert(tree.right, value))
    return tree


def contains(tree: Node | None, value: int) -> bool:
    if tree is None:
        return False
    if value < tree.value:
        return contains(tree.left, value)
    if value > tree.value:
        return contains(tree.right, value)
    return True


class BinarySearchTreeTests(unittest.TestCase):
    def test_membership(self) -> None:
        tree = insert(insert(insert(insert(None, 5), 2), 8), 5)
        self.assertTrue(contains(tree, 5) and contains(tree, 8))
        self.assertFalse(contains(tree, 7))


if __name__ == "__main__":
    tree = insert(insert(insert(None, 5), 2), 8)
    print(f"contains 2? {contains(tree, 2)}")
