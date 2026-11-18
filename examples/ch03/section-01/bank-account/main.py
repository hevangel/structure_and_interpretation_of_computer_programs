import unittest


class Account:
    def __init__(self, balance: int) -> None:
        self._balance = balance

    def deposit(self, amount: int) -> None:
        assert amount >= 0
        self._balance += amount

    def withdraw(self, amount: int) -> None:
        if amount > self._balance:
            raise ValueError("insufficient funds")
        self._balance -= amount

    def balance(self) -> int:
        return self._balance


class BankAccountTests(unittest.TestCase):
    def test_history_changes_results(self) -> None:
        a = Account(20)
        a.deposit(5)
        with self.assertRaises(ValueError):
            a.withdraw(30)
        self.assertEqual(a.balance(), 25)
        a.withdraw(10)
        self.assertEqual(a.balance(), 15)


if __name__ == "__main__":
    a = Account(100)
    a.deposit(25)
    a.withdraw(30)
    print(f"remaining balance: {a.balance()}")
