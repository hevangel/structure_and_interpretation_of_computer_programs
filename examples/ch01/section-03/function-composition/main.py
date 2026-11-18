from collections.abc import Callable
import unittest


def compose(outer: Callable[[int], int], inner: Callable[[int], int]) -> Callable[[int], int]:
    return lambda x: outer(inner(x))


def repeated(f: Callable[[int], int], times: int) -> Callable[[int], int]:
    def apply(x: int) -> int:
        for _ in range(times):
            x = f(x)
        return x

    return apply


class FunctionCompositionTests(unittest.TestCase):
    def test_composition_is_ordered(self) -> None:
        self.assertEqual(compose(lambda x: x * x, lambda x: x + 1)(5), 36)
        self.assertEqual(compose(lambda x: x + 1, lambda x: x * x)(5), 26)
        self.assertEqual(repeated(lambda x: x * 2, 0)(3), 3)
        self.assertEqual(repeated(lambda x: x * 2, 5)(3), 96)


if __name__ == "__main__":
    transform = compose(lambda x: x * x, lambda x: x + 1)
    print(f"(5+1)^2 = {transform(5)}")
    print(f"add ten = {repeated(lambda x: x + 2, 5)(0)}")
