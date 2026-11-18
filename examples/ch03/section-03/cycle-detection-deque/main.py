from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar
import unittest

T = TypeVar("T")


@dataclass
class ConsCell:
    car: object
    cdr: object


def naive_count_pairs(cell: object, budget: list[int]) -> int | None:
    if not isinstance(cell, ConsCell):
        return 0
    if budget[0] == 0:
        return None
    budget[0] -= 1
    left = naive_count_pairs(cell.car, budget)
    right = naive_count_pairs(cell.cdr, budget)
    if left is None or right is None:
        return None
    return 1 + left + right


def unique_count_pairs(cell: object) -> int:
    seen: set[int] = set()

    def walk(node: object) -> None:
        if isinstance(node, ConsCell) and id(node) not in seen:
            seen.add(id(node))
            walk(node.car)
            walk(node.cdr)

    walk(cell)
    return len(seen)


def has_cdr_cycle(head: object) -> bool:
    """Floyd's O(1)-space tortoise-and-hare cycle detector along cdr."""
    slow = head
    fast = head
    while isinstance(fast, ConsCell) and isinstance(fast.cdr, ConsCell):
        assert isinstance(slow, ConsCell)
        slow = slow.cdr
        fast = fast.cdr.cdr
        if slow is fast:
            return True
    return False


@dataclass
class _DequeNode(Generic[T]):
    value: T
    prev: _DequeNode[T] | None = None
    nxt: _DequeNode[T] | None = None


class Deque(Generic[T]):
    def __init__(self) -> None:
        self.front: _DequeNode[T] | None = None
        self.rear: _DequeNode[T] | None = None
        self.size = 0

    def is_empty(self) -> bool:
        return self.size == 0

    def front_insert(self, value: T) -> None:
        node = _DequeNode(value, prev=None, nxt=self.front)
        if self.front is not None:
            self.front.prev = node
        else:
            self.rear = node
        self.front = node
        self.size += 1

    def rear_insert(self, value: T) -> None:
        node = _DequeNode(value, prev=self.rear, nxt=None)
        if self.rear is not None:
            self.rear.nxt = node
        else:
            self.front = node
        self.rear = node
        self.size += 1

    def front_delete(self) -> T | None:
        if self.front is None:
            return None
        val = self.front.value
        self.front = self.front.nxt
        if self.front is not None:
            self.front.prev = None
        else:
            self.rear = None
        self.size -= 1
        return val

    def rear_delete(self) -> T | None:
        if self.rear is None:
            return None
        val = self.rear.value
        self.rear = self.rear.prev
        if self.rear is not None:
            self.rear.nxt = None
        else:
            self.front = None
        self.size -= 1
        return val


class CycleDetectionDequeTest(unittest.TestCase):
    def test_three_cell_counts_and_floyd_cycle(self) -> None:
        linear = ConsCell(1, ConsCell(2, ConsCell(3, None)))
        self.assertEqual(naive_count_pairs(linear, [50]), 3)
        self.assertEqual(unique_count_pairs(linear), 3)

        leaf = ConsCell(1, None)
        mid = ConsCell(leaf, leaf)
        top = ConsCell(mid, None)
        self.assertEqual(naive_count_pairs(top, [50]), 4)
        self.assertEqual(unique_count_pairs(top), 3)

        x = ConsCell(1, None)
        y = ConsCell(x, x)
        z = ConsCell(y, y)
        self.assertEqual(naive_count_pairs(z, [50]), 7)
        self.assertEqual(unique_count_pairs(z), 3)
        self.assertFalse(has_cdr_cycle(z))

        x.cdr = z
        self.assertIsNone(naive_count_pairs(z, [50]))
        self.assertEqual(unique_count_pairs(z), 3)
        self.assertTrue(has_cdr_cycle(z))

    def test_deque_operations(self) -> None:
        dq: Deque[str] = Deque()
        self.assertTrue(dq.is_empty())
        dq.rear_insert("b")
        dq.front_insert("a")
        dq.rear_insert("c")
        self.assertEqual(dq.front_delete(), "a")
        self.assertEqual(dq.rear_delete(), "c")
        self.assertEqual(dq.rear_delete(), "b")
        self.assertTrue(dq.is_empty())


if __name__ == "__main__":
    x = ConsCell(1, None)
    y = ConsCell(x, x)
    z = ConsCell(y, y)
    print(f"naive={naive_count_pairs(z, [50])}, unique={unique_count_pairs(z)}, cycle={has_cdr_cycle(z)}")
    x.cdr = z
    print(f"after mutation: cycle={has_cdr_cycle(z)}")
