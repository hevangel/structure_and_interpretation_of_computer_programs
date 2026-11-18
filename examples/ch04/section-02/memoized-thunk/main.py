from collections.abc import Callable
import unittest


class Thunk:
    def __init__(self, fn: Callable[[], int]) -> None:
        self._pending: Callable[[], int] | None = fn
        self._cached: int | None = None

    def force(self) -> int:
        if self._cached is not None:
            return self._cached
        assert self._pending is not None
        value = self._pending()
        self._pending = None
        self._cached = value
        return value


class MemoizedThunkTests(unittest.TestCase):
    def test_evaluate_exactly_once(self) -> None:
        calls = 0

        def compute() -> int:
            nonlocal calls
            calls += 1
            return 42

        t = Thunk(compute)
        self.assertEqual(t.force(), 42)
        self.assertEqual(t.force(), 42)
        self.assertEqual(calls, 1)


if __name__ == "__main__":
    delayed = Thunk(lambda: 21 * 2)
    print(f"first {delayed.force()}, second {delayed.force()}")
