from __future__ import annotations

import threading
import unittest


def enumerate_unserialized_ex3_38(initial: int) -> set[int]:
    """Enumerate all balance outcomes when Peter (+10), Paul (-20), Mary (/2) interleave."""
    outcomes: set[int] = set()

    def dfs(bal: int, p: int, q: int, m: int, pv: int, qv: int, mv: int) -> None:
        if p == 2 and q == 2 and m == 2:
            outcomes.add(bal)
            return
        if p == 0:
            dfs(bal, 1, q, m, bal, qv, mv)
        elif p == 1:
            dfs(pv + 10, 2, q, m, pv, qv, mv)
        if q == 0:
            dfs(bal, p, 1, m, pv, bal, mv)
        elif q == 1:
            dfs(qv - 20, p, 2, m, pv, qv, mv)
        if m == 0:
            dfs(bal, p, q, 1, pv, qv, bal)
        elif m == 1:
            dfs(mv - mv // 2, p, q, 2, pv, qv, mv)

    dfs(initial, 0, 0, 0, 0, 0, 0)
    return outcomes


class SerializedAccount:
    def __init__(self, account_id: int, balance: int) -> None:
        self.id = account_id
        self.balance = balance
        self.lock = threading.Lock()


def serialized_exchange(a: SerializedAccount, b: SerializedAccount) -> None:
    if a.id == b.id:
        return
    first, second = (a, b) if a.id < b.id else (b, a)
    with first.lock:
        with second.lock:
            diff = first.balance - second.balance
            first.balance -= diff
            second.balance += diff


def run_opposing_exchanges(rounds: int) -> tuple[int, int]:
    acc_a = SerializedAccount(1, 100)
    acc_b = SerializedAccount(2, 250)
    t1 = threading.Thread(target=lambda: [serialized_exchange(acc_a, acc_b) for _ in range(rounds)])
    t2 = threading.Thread(target=lambda: [serialized_exchange(acc_b, acc_a) for _ in range(rounds)])
    t1.start()
    t2.start()
    t1.join()
    t2.join()
    return acc_a.balance, acc_b.balance


class SerializedTransfersTest(unittest.TestCase):
    def test_interleavings(self) -> None:
        self.assertEqual(
            enumerate_unserialized_ex3_38(100), {30, 35, 40, 45, 50, 55, 60, 80, 90, 110}
        )

    def test_deadlock_free_exchanges(self) -> None:
        a, b = run_opposing_exchanges(1_000)
        self.assertEqual(a + b, 350)
        self.assertIn((a, b), {(100, 250), (250, 100)})


if __name__ == "__main__":
    print("Ex 3.38 outcomes from 100:", sorted(enumerate_unserialized_ex3_38(100)))
    a, b = run_opposing_exchanges(200)
    print(f"deadlock-free exchange balances: ({a}, {b})")
