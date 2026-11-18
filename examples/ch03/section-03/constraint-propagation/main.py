from __future__ import annotations

import math
from typing import Protocol
import unittest


class Constraint(Protocol):
    def process_new_value(self) -> None: ...
    def process_forget_value(self) -> None: ...


class Connector:
    def __init__(self) -> None:
        self.value: float | None = None
        self.informant: object | None = None
        self.constraints: list[Constraint] = []

    def has_value(self) -> bool:
        return self.value is not None

    def connect(self, c: Constraint) -> None:
        self.constraints.append(c)
        if self.has_value():
            c.process_new_value()

    def set_value(self, new_val: float, setter: object) -> None:
        if self.value is None:
            self.value = new_val
            self.informant = setter
            for c in self.constraints:
                if c is not setter:
                    c.process_new_value()
        elif abs(self.value - new_val) > 1e-12:
            raise ValueError(f"contradiction: {self.value} vs {new_val}")

    def forget_value(self, retractor: object) -> None:
        if self.informant is not retractor:
            return
        self.value = None
        self.informant = None
        for c in self.constraints:
            if c is not retractor:
                c.process_forget_value()


class Adder:
    def __init__(self, a1: Connector, a2: Connector, total: Connector) -> None:
        self.a1, self.a2, self.total = a1, a2, total
        a1.connect(self)
        a2.connect(self)
        total.connect(self)

    def process_new_value(self) -> None:
        v1, v2, vs = self.a1.value, self.a2.value, self.total.value
        if v1 is not None and v2 is not None:
            self.total.set_value(v1 + v2, self)
        elif v1 is not None and vs is not None:
            self.a2.set_value(vs - v1, self)
        elif v2 is not None and vs is not None:
            self.a1.set_value(vs - v2, self)

    def process_forget_value(self) -> None:
        self.total.forget_value(self)
        self.a1.forget_value(self)
        self.a2.forget_value(self)
        self.process_new_value()


class Multiplier:
    def __init__(self, m1: Connector, m2: Connector, product: Connector) -> None:
        self.m1, self.m2, self.product = m1, m2, product
        m1.connect(self)
        m2.connect(self)
        product.connect(self)

    def process_new_value(self) -> None:
        v1, v2, vp = self.m1.value, self.m2.value, self.product.value
        if v1 == 0.0 or v2 == 0.0:
            self.product.set_value(0.0, self)
        elif v1 is not None and v2 is not None:
            self.product.set_value(v1 * v2, self)
        elif vp is not None and v1 is not None and v1 != 0.0:
            self.m2.set_value(vp / v1, self)
        elif vp is not None and v2 is not None and v2 != 0.0:
            self.m1.set_value(vp / v2, self)

    def process_forget_value(self) -> None:
        self.product.forget_value(self)
        self.m1.forget_value(self)
        self.m2.forget_value(self)
        self.process_new_value()


class Squarer:
    def __init__(self, a: Connector, b: Connector) -> None:
        self.a, self.b = a, b
        a.connect(self)
        b.connect(self)

    def process_new_value(self) -> None:
        if self.b.value is not None and self.b.value >= 0.0:
            self.a.set_value(math.sqrt(self.b.value), self)
        elif self.a.value is not None:
            self.b.set_value(self.a.value * self.a.value, self)

    def process_forget_value(self) -> None:
        self.a.forget_value(self)
        self.b.forget_value(self)
        self.process_new_value()


def cv(value: float) -> Connector:
    c = Connector()
    c.set_value(value, "constant")
    return c


def c_add(x: Connector, y: Connector) -> Connector:
    z = Connector()
    Adder(x, y, z)
    return z


def c_mul(x: Connector, y: Connector) -> Connector:
    z = Connector()
    Multiplier(x, y, z)
    return z


def c_div(x: Connector, y: Connector) -> Connector:
    z = Connector()
    Multiplier(z, y, x)
    return z


def averager(a: Connector, b: Connector, c: Connector) -> None:
    Multiplier(cv(2.0), c, c_add(a, b))


def celsius_fahrenheit_converter(c: Connector) -> Connector:
    return c_add(c_mul(c_div(cv(9.0), cv(5.0)), c), cv(32.0))


class ConstraintPropagationTest(unittest.TestCase):
    def test_forget_and_reverse_propagation(self) -> None:
        c = Connector()
        f = celsius_fahrenheit_converter(c)
        c.set_value(25.0, "user")
        self.assertEqual(f.value, 77.0)
        with self.assertRaises(ValueError):
            f.set_value(100.0, "user")
        c.forget_value("user")
        self.assertFalse(f.has_value())
        f.set_value(212.0, "user")
        self.assertEqual(c.value, 100.0)

    def test_averager_and_squarer(self) -> None:
        a, b, avg = Connector(), Connector(), Connector()
        averager(a, b, avg)
        a.set_value(10.0, "user")
        avg.set_value(20.0, "user")
        self.assertEqual(b.value, 30.0)
        avg.forget_value("user")
        self.assertIsNone(b.value)
        self.assertEqual(a.value, 10.0)
        b.set_value(40.0, "user")
        self.assertEqual(avg.value, 25.0)

        x, y = Connector(), Connector()
        Squarer(x, y)
        y.set_value(49.0, "user")
        self.assertEqual(x.value, 7.0)


if __name__ == "__main__":
    c = Connector()
    f = celsius_fahrenheit_converter(c)
    c.set_value(100.0, "user")
    print("100 C ->", f.value, "F")
    c.forget_value("user")
    f.set_value(32.0, "user")
    print("32 F ->", c.value, "C")
