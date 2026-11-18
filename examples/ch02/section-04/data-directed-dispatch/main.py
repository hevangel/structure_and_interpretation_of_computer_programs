from __future__ import annotations

from dataclasses import dataclass
import math
from typing import Any, Callable
import unittest


@dataclass(frozen=True)
class TaggedDatum:
    tag: str
    contents: Any


class DispatchTable:
    def __init__(self) -> None:
        self._table: dict[tuple[str, tuple[str, ...]], Callable[..., Any]] = {}

    def put(self, op: str, type_tags: tuple[str, ...], proc: Callable[..., Any]) -> None:
        self._table[(op, type_tags)] = proc

    def get(self, op: str, type_tags: tuple[str, ...]) -> Callable[..., Any] | None:
        return self._table.get((op, type_tags))

    def apply_generic(self, op: str, *args: TaggedDatum) -> Any:
        tags = tuple(a.tag for a in args)
        proc = self.get(op, tags)
        if proc is None:
            raise KeyError(f"no method for ({op}, {tags})")
        return proc(*(a.contents for a in args))


def install_rectangular_package(tbl: DispatchTable) -> None:
    tbl.put("real_part", ("rectangular",), lambda z: z[0])
    tbl.put("imag_part", ("rectangular",), lambda z: z[1])
    tbl.put("magnitude", ("rectangular",), lambda z: math.hypot(z[0], z[1]))
    tbl.put("angle", ("rectangular",), lambda z: math.atan2(z[1], z[0]))


def install_polar_package(tbl: DispatchTable) -> None:
    tbl.put("real_part", ("polar",), lambda z: z[0] * math.cos(z[1]))
    tbl.put("imag_part", ("polar",), lambda z: z[0] * math.sin(z[1]))
    tbl.put("magnitude", ("polar",), lambda z: z[0])
    tbl.put("angle", ("polar",), lambda z: z[1])


class DerivRegistry:
    def __init__(self) -> None:
        self.rules: dict[str, Callable[[Any, Any, str, DerivRegistry], Any]] = {}

    def put(self, op: str, rule: Callable[[Any, Any, str, DerivRegistry], Any]) -> None:
        self.rules[op] = rule

    def deriv(self, expr: Any, var: str) -> Any:
        if isinstance(expr, (int, float)):
            return 0
        if isinstance(expr, str):
            return 1 if expr == var else 0
        op, u, v = expr
        return self.rules[op](u, v, var, self)


def make_from_mag_ang_mp(r: float, a: float) -> Callable[[str], float]:
    def dispatch(op: str) -> float:
        if op == "real_part":
            return r * math.cos(a)
        if op == "imag_part":
            return r * math.sin(a)
        if op == "magnitude":
            return r
        if op == "angle":
            return a
        raise KeyError(f"unknown message: {op}")

    return dispatch


class DataDirectedDispatchTest(unittest.TestCase):
    def test_complex_dispatch_table(self) -> None:
        tbl = DispatchTable()
        install_rectangular_package(tbl)
        install_polar_package(tbl)

        z_rect = TaggedDatum("rectangular", (3.0, 4.0))
        z_pol = TaggedDatum("polar", (2.0, math.pi / 2.0))
        self.assertAlmostEqual(tbl.apply_generic("magnitude", z_rect), 5.0)
        self.assertAlmostEqual(tbl.apply_generic("real_part", z_rect), 3.0)
        self.assertAlmostEqual(tbl.apply_generic("imag_part", z_pol), 2.0)

    def test_additive_deriv_rules(self) -> None:
        reg = DerivRegistry()
        reg.put("+", lambda u, v, var, r: ("+", r.deriv(u, var), r.deriv(v, var)))
        reg.put(
            "*",
            lambda u, v, var, r: ("+", ("*", u, r.deriv(v, var)), ("*", r.deriv(u, var), v)),
        )
        self.assertEqual(
            reg.deriv(("*", "x", "x"), "x"),
            ("+", ("*", "x", 1), ("*", 1, "x")),
        )

    def test_message_passing(self) -> None:
        mp = make_from_mag_ang_mp(4.0, 0.0)
        self.assertAlmostEqual(mp("real_part"), 4.0)
        self.assertAlmostEqual(mp("magnitude"), 4.0)


if __name__ == "__main__":
    tbl = DispatchTable()
    install_rectangular_package(tbl)
    install_polar_package(tbl)
    print("magnitude(3+4i) =", tbl.apply_generic("magnitude", TaggedDatum("rectangular", (3.0, 4.0))))
    print("mp(real_part) =", make_from_mag_ang_mp(2.0, math.pi / 2.0)("imag_part"))
