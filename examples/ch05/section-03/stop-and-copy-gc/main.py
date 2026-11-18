from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class PairPtr:
    index: int


@dataclass(frozen=True)
class BrokenHeart:
    forwarded: int


Pointer = None | int | str | PairPtr | BrokenHeart


class StopAndCopyMemory:
    def __init__(self, capacity: int) -> None:
        self.capacity = capacity
        self.the_cars: list[Pointer] = [None] * capacity
        self.the_cdrs: list[Pointer] = [None] * capacity
        self.new_cars: list[Pointer] = [None] * capacity
        self.new_cdrs: list[Pointer] = [None] * capacity
        self.free = 0
        self.gc_cycles = 0

    def cons_raw(self, car: Pointer, cdr: Pointer) -> PairPtr:
        if self.free >= self.capacity:
            raise MemoryError("out of raw pair memory")
        idx = self.free
        self.the_cars[idx] = car
        self.the_cdrs[idx] = cdr
        self.free += 1
        return PairPtr(idx)

    def car(self, ptr: Pointer) -> Pointer:
        assert isinstance(ptr, PairPtr)
        return self.the_cars[ptr.index]

    def cdr(self, ptr: Pointer) -> Pointer:
        assert isinstance(ptr, PairPtr)
        return self.the_cdrs[ptr.index]

    def set_cdr(self, ptr: PairPtr, val: Pointer) -> None:
        self.the_cdrs[ptr.index] = val

    def _relocate(self, old: Pointer, new_free: list[int]) -> Pointer:
        if isinstance(old, PairPtr):
            head = self.the_cars[old.index]
            if isinstance(head, BrokenHeart):
                return PairPtr(head.forwarded)
            target = new_free[0]
            new_free[0] += 1
            self.new_cars[target] = self.the_cars[old.index]
            self.new_cdrs[target] = self.the_cdrs[old.index]
            self.the_cars[old.index] = BrokenHeart(target)
            self.the_cdrs[old.index] = None
            return PairPtr(target)
        return old

    def collect_with_roots(self, roots: list[Pointer]) -> list[Pointer]:
        new_free = [0]
        scan = 0
        relocated = [self._relocate(r, new_free) for r in roots]
        while scan < new_free[0]:
            self.new_cars[scan] = self._relocate(self.new_cars[scan], new_free)
            self.new_cdrs[scan] = self._relocate(self.new_cdrs[scan], new_free)
            scan += 1
        self.the_cars, self.new_cars = self.new_cars, [None] * self.capacity
        self.the_cdrs, self.new_cdrs = self.new_cdrs, [None] * self.capacity
        self.free = new_free[0]
        self.gc_cycles += 1
        return relocated


class StopAndCopyGcTests(unittest.TestCase):
    def test_compacts_shared_dag_without_duplicating(self) -> None:
        mem = StopAndCopyMemory(16)
        for i in range(8):
            mem.cons_raw(i, None)
        shared = mem.cons_raw(10, 20)
        root = mem.cons_raw(shared, shared)
        self.assertEqual(mem.free, 10)

        [new_root] = mem.collect_with_roots([root])
        self.assertEqual(mem.free, 2)
        self.assertEqual(mem.car(new_root), mem.cdr(new_root))
        self.assertEqual(mem.car(mem.car(new_root)), 10)
        self.assertEqual(mem.cdr(mem.car(new_root)), 20)

    def test_cyclic_list_across_multiple_gc_flips(self) -> None:
        mem = StopAndCopyMemory(8)
        tail = mem.cons_raw(2, None)
        head = mem.cons_raw(1, tail)
        mem.set_cdr(tail, head)
        root: Pointer = head
        for cycle in range(1, 5):
            mem.cons_raw(999, None)
            [root] = mem.collect_with_roots([root])
            self.assertEqual(mem.free, 2)
            self.assertEqual(mem.gc_cycles, cycle)
            self.assertEqual(mem.car(root), 1)
            second = mem.cdr(root)
            self.assertEqual(mem.car(second), 2)
            self.assertEqual(mem.cdr(second), root)


if __name__ == "__main__":
    mem = StopAndCopyMemory(16)
    for i in range(6):
        mem.cons_raw(i, None)
    shared = mem.cons_raw("leaf", None)
    root = mem.cons_raw(shared, shared)
    before_free = mem.free
    [new_root] = mem.collect_with_roots([root])
    print(
        f"compacted free: {before_free} -> {mem.free}, cycles = {mem.gc_cycles}, "
        f"shared_same = {mem.car(new_root) == mem.cdr(new_root)}"
    )
