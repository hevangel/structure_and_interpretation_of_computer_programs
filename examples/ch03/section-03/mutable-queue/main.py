from collections import deque
from typing import Generic, TypeVar
import unittest


T = TypeVar("T")


class Queue(Generic[T]):
    def __init__(self) -> None:
        self._items: deque[T] = deque()

    def push(self, item: T) -> None:
        self._items.append(item)

    def pop(self) -> T | None:
        return self._items.popleft() if self._items else None

    def __len__(self) -> int:
        return len(self._items)


class MutableQueueTests(unittest.TestCase):
    def test_fifo_and_empty(self) -> None:
        q: Queue[int] = Queue()
        self.assertIsNone(q.pop())
        q.push(1)
        q.push(2)
        self.assertEqual(len(q), 2)
        self.assertEqual(q.pop(), 1)
        self.assertEqual(q.pop(), 2)
        self.assertIsNone(q.pop())


if __name__ == "__main__":
    q: Queue[str] = Queue()
    q.push("first")
    q.push("second")
    print(f"len={len(q)}, head={q.pop()}, remaining={len(q)}")
