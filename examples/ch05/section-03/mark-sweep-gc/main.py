from __future__ import annotations

from dataclasses import dataclass, field
import unittest


@dataclass
class Node:
    children: list[int]
    marked: bool = False


@dataclass
class Heap:
    slots: list[Node | None] = field(default_factory=list)

    def alloc(self, children: list[int]) -> int:
        node_id = len(self.slots)
        self.slots.append(Node(children=list(children), marked=False))
        return node_id

    def mark(self, node_id: int) -> None:
        if not (0 <= node_id < len(self.slots)):
            return
        node = self.slots[node_id]
        if node is None or node.marked:
            return
        node.marked = True
        for child in list(node.children):
            self.mark(child)

    def collect(self, roots: list[int]) -> int:
        for root_id in roots:
            self.mark(root_id)
        freed = 0
        for idx, node in enumerate(self.slots):
            if node is not None:
                if node.marked:
                    node.marked = False
                else:
                    self.slots[idx] = None
                    freed += 1
        return freed


def main() -> None:
    heap = Heap()
    leaf = heap.alloc([])
    root = heap.alloc([leaf])
    heap.alloc([])
    print(f"reclaimed {heap.collect([root])} unreachable node(s)")


class TestMarkSweepGc(unittest.TestCase):
    def test_graph_reachability(self) -> None:
        heap = Heap()
        a = heap.alloc([])
        b = heap.alloc([a])
        dead = heap.alloc([])
        self.assertEqual(heap.collect([b]), 1)
        self.assertIsNone(heap.slots[dead])
        self.assertIsNotNone(heap.slots[a])
        self.assertEqual(heap.collect([]), 2)


if __name__ == "__main__":
    main()
