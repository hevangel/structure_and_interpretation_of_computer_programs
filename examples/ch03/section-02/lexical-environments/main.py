from __future__ import annotations

import unittest


class Environment:
    def __init__(self, parent: Environment | None = None) -> None:
        self.values: dict[str, int] = {}
        self.parent = parent

    def child(self) -> Environment:
        return Environment(parent=self)

    def define(self, name: str, value: int) -> None:
        self.values[name] = value

    def lookup(self, name: str) -> int | None:
        # Closest frame shadows any enclosing binding of the same name.
        if name in self.values:
            return self.values[name]
        if self.parent is not None:
            return self.parent.lookup(name)
        return None


class LexicalEnvironmentsTests(unittest.TestCase):
    def test_shadowing_and_free_variables(self) -> None:
        global_env = Environment()
        global_env.define("x", 1)
        global_env.define("y", 2)
        local_env = global_env.child()
        local_env.define("x", 99)
        self.assertEqual(local_env.lookup("x"), 99)
        self.assertEqual(local_env.lookup("y"), 2)
        self.assertIsNone(local_env.lookup("z"))


if __name__ == "__main__":
    outer = Environment()
    outer.define("x", 10)
    inner = outer.child()
    inner.define("x", 20)
    print(f"inner x = {inner.lookup('x')}")
