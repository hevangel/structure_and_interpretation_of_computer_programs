import unittest


class SharedAccount:
    def __init__(self, balance: int) -> None:
        self.balance = balance


class Handle:
    def __init__(self, shared: SharedAccount, password: str) -> None:
        self._shared = shared
        self._password = password

    @classmethod
    def create(cls, balance: int, password: str) -> "Handle":
        return cls(SharedAccount(balance), password)

    def joint(self, current: str, new_password: str) -> "Handle":
        if self._password != current:
            raise PermissionError("authentication failed")
        return Handle(self._shared, new_password)

    def withdraw(self, provided: str, amount: int) -> int:
        if provided != self._password:
            raise PermissionError("authentication failed")
        if amount > self._shared.balance:
            raise ValueError("insufficient funds")
        self._shared.balance -= amount
        return self._shared.balance

    def balance(self, provided: str) -> int:
        if provided != self._password:
            raise PermissionError("authentication failed")
        return self._shared.balance


class JointAccountTests(unittest.TestCase):
    def test_independent_passwords_shared_balance(self) -> None:
        a = Handle.create(100, "a")
        b = a.joint("a", "b")
        with self.assertRaises(PermissionError):
            a.joint("wrong", "c")
        with self.assertRaises(PermissionError):
            b.withdraw("a", 10)
        self.assertEqual(b.withdraw("b", 40), 60)
        self.assertEqual(a.balance("a"), 60)
        with self.assertRaises(ValueError):
            a.withdraw("a", 70)
        self.assertEqual(b.balance("b"), 60)


if __name__ == "__main__":
    alice = Handle.create(100, "alice")
    bob = alice.joint("alice", "bob")
    bob.withdraw("bob", 30)
    print(f"shared balance: {alice.balance('alice')}")
