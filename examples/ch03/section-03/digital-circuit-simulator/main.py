from __future__ import annotations

from collections import deque
from typing import Callable
import unittest

Action = Callable[["Agenda"], None]


class Agenda:
    def __init__(self) -> None:
        self.current_time = 0
        self.segments: dict[int, deque[Action]] = {}

    def after_delay(self, delay: int, action: Action) -> None:
        time = self.current_time + delay
        if time not in self.segments:
            self.segments[time] = deque()
        self.segments[time].append(action)

    def propagate(self) -> None:
        while self.segments:
            next_time = min(self.segments)
            self.current_time = next_time
            bucket = self.segments[next_time]
            action = bucket.popleft()
            if not bucket:
                del self.segments[next_time]
            action(self)


class Wire:
    def __init__(self) -> None:
        self.signal = False
        self.actions: list[Action] = []

    def get(self) -> bool:
        return self.signal

    def set(self, new_val: bool, agenda: Agenda) -> None:
        if self.signal == new_val:
            return
        self.signal = new_val
        for action in list(self.actions):
            action(agenda)

    def add_action(self, action: Action, agenda: Agenda) -> None:
        self.actions.append(action)
        action(agenda)


INVERTER_DELAY = 2
AND_DELAY = 3
OR_DELAY = 5


def inverter(inp: Wire, out: Wire, agenda: Agenda) -> None:
    def act(ag: Agenda) -> None:
        sig = not inp.get()
        ag.after_delay(INVERTER_DELAY, lambda ag2: out.set(sig, ag2))

    inp.add_action(act, agenda)


def and_gate(a1: Wire, a2: Wire, out: Wire, agenda: Agenda) -> None:
    def act(ag: Agenda) -> None:
        sig = a1.get() and a2.get()
        ag.after_delay(AND_DELAY, lambda ag2: out.set(sig, ag2))

    a1.add_action(act, agenda)
    a2.add_action(act, agenda)


def or_gate(o1: Wire, o2: Wire, out: Wire, agenda: Agenda) -> None:
    def act(ag: Agenda) -> None:
        sig = o1.get() or o2.get()
        ag.after_delay(OR_DELAY, lambda ag2: out.set(sig, ag2))

    o1.add_action(act, agenda)
    o2.add_action(act, agenda)


def half_adder(a: Wire, b: Wire, s: Wire, c: Wire, agenda: Agenda) -> None:
    d, e = Wire(), Wire()
    or_gate(a, b, d, agenda)
    and_gate(a, b, c, agenda)
    inverter(c, e, agenda)
    and_gate(d, e, s, agenda)


def full_adder(a: Wire, b: Wire, c_in: Wire, s: Wire, c_out: Wire, agenda: Agenda) -> None:
    s1, c1, c2 = Wire(), Wire(), Wire()
    half_adder(b, c_in, s1, c1, agenda)
    half_adder(a, s1, s, c2, agenda)
    or_gate(c1, c2, c_out, agenda)


def ripple_carry_adder(
    a_bits: list[Wire],
    b_bits: list[Wire],
    s_bits: list[Wire],
    c_out: Wire,
    agenda: Agenda,
) -> None:
    carry_in = Wire()
    for i in range(len(a_bits)):
        next_carry = c_out if i + 1 == len(a_bits) else Wire()
        full_adder(a_bits[i], b_bits[i], carry_in, s_bits[i], next_carry, agenda)
        carry_in = next_carry


def add_4bit(a: int, b: int) -> tuple[int, bool, int]:
    agenda = Agenda()
    a_bits = [Wire() for _ in range(4)]
    b_bits = [Wire() for _ in range(4)]
    s_bits = [Wire() for _ in range(4)]
    c_out = Wire()
    ripple_carry_adder(a_bits, b_bits, s_bits, c_out, agenda)
    agenda.propagate()
    t0 = agenda.current_time
    for i in range(4):
        a_bits[i].set(bool((a >> i) & 1), agenda)
        b_bits[i].set(bool((b >> i) & 1), agenda)
    agenda.propagate()
    total_sum = sum((1 << i) for i, w in enumerate(s_bits) if w.get())
    return total_sum, c_out.get(), agenda.current_time - t0


class DigitalCircuitTest(unittest.TestCase):
    def test_half_adder_timing(self) -> None:
        ag = Agenda()
        a, b, s, c = Wire(), Wire(), Wire(), Wire()
        half_adder(a, b, s, c, ag)
        ag.propagate()
        t0 = ag.current_time
        a.set(True, ag)
        ag.propagate()
        self.assertTrue(s.get())
        self.assertFalse(c.get())
        self.assertEqual(ag.current_time - t0, 8)

    def test_four_bit_ripple_carry_exhaustive(self) -> None:
        for a in range(16):
            for b in range(16):
                s, carry, _ = add_4bit(a, b)
                self.assertEqual((16 if carry else 0) + s, a + b)


if __name__ == "__main__":
    s, carry, ticks = add_4bit(11, 7)
    print(f"4-bit ripple-carry 11 + 7 = {s} (carry={carry}) in {ticks} ticks")
