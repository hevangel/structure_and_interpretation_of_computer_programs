from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass
class Machine:
    a: int
    b: int
    steps: int = 0

    def run(self) -> None:
        while self.b != 0:
            # A register transfer: (a, b) <- (b, a mod b).
            remainder = self.a % self.b
            self.a = self.b
            self.b = remainder
            self.steps += 1


def main() -> None:
    m = Machine(a=206, b=40)
    m.run()
    print(f"gcd={m.a} steps={m.steps}")


class TestRegisterGcd(unittest.TestCase):
    def test_machine_state(self) -> None:
        m = Machine(a=206, b=40)
        m.run()
        self.assertEqual((m.a, m.b), (2, 0))
        self.assertGreater(m.steps, 0)


if __name__ == "__main__":
    main()
