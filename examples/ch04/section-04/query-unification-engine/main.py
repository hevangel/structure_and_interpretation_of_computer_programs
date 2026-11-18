from __future__ import annotations

from dataclasses import dataclass
from typing import Any
import unittest


@dataclass(frozen=True)
class Var:
    id: int
    name: str


def lst(*items: Any) -> Any:
    acc: Any = ()
    for item in reversed(items):
        acc = (item, acc)
    return acc


def dotted(items: list[Any], tail: Any) -> Any:
    acc = tail
    for item in reversed(items):
        acc = (item, acc)
    return acc


def format_term(term: Any) -> str:
    if isinstance(term, Var):
        return f"?{term.name}" if term.id == 0 else f"?{term.name}#{term.id}"
    if isinstance(term, tuple) and len(term) == 2:
        parts = []
        cur: Any = term
        while isinstance(cur, tuple) and len(cur) == 2:
            parts.append(format_term(cur[0]))
            cur = cur[1]
        if cur == ():
            return f"({' '.join(parts)})"
        return f"({' '.join(parts)} . {format_term(cur)})"
    if term == ():
        return "()"
    return str(term)


class QueryEngine:
    def __init__(self) -> None:
        self.assertions: list[Any] = []
        self.rules: list[tuple[Any, Any]] = []
        self.rule_counter = 0

    def add_assertion(self, term: Any) -> None:
        self.assertions.append(term)

    def add_rule(self, conclusion: Any, body: Any = None) -> None:
        self.rules.append((conclusion, body))

    def instantiate(self, term: Any, frame: dict[Var, Any]) -> Any:
        if isinstance(term, Var):
            if term in frame:
                return self.instantiate(frame[term], frame)
            return term
        if isinstance(term, tuple) and len(term) == 2:
            return (self.instantiate(term[0], frame), self.instantiate(term[1], frame))
        return term

    def depends_on(self, exp: Any, var: Var, frame: dict[Var, Any]) -> bool:
        if isinstance(exp, Var):
            if exp == var:
                return True
            if exp in frame:
                return self.depends_on(frame[exp], var, frame)
            return False
        if isinstance(exp, tuple) and len(exp) == 2:
            return self.depends_on(exp[0], var, frame) or self.depends_on(exp[1], var, frame)
        return False

    def extend_if_possible(
        self, var: Var, val: Any, frame: dict[Var, Any]
    ) -> dict[Var, Any] | None:
        if var in frame:
            return self.unify_match(frame[var], val, frame)
        if isinstance(val, Var):
            if var == val:
                return frame
            if val in frame:
                return self.unify_match(var, frame[val], frame)
        if self.depends_on(val, var, frame):
            return None
        next_frame = dict(frame)
        next_frame[var] = val
        return next_frame

    def unify_match(self, p1: Any, p2: Any, frame: dict[Var, Any]) -> dict[Var, Any] | None:
        if p1 == p2:
            return frame
        if isinstance(p1, Var):
            return self.extend_if_possible(p1, p2, frame)
        if isinstance(p2, Var):
            return self.extend_if_possible(p2, p1, frame)
        if (
            isinstance(p1, tuple)
            and len(p1) == 2
            and isinstance(p2, tuple)
            and len(p2) == 2
        ):
            f1 = self.unify_match(p1[0], p2[0], frame)
            if f1 is None:
                return None
            return self.unify_match(p1[1], p2[1], f1)
        return None

    def pattern_match(self, pat: Any, dat: Any, frame: dict[Var, Any]) -> dict[Var, Any] | None:
        if pat == dat:
            return frame
        if isinstance(pat, Var):
            if pat in frame:
                return self.pattern_match(frame[pat], dat, frame)
            next_frame = dict(frame)
            next_frame[pat] = dat
            return next_frame
        if (
            isinstance(pat, tuple)
            and len(pat) == 2
            and isinstance(dat, tuple)
            and len(dat) == 2
        ):
            f1 = self.pattern_match(pat[0], dat[0], frame)
            if f1 is None:
                return None
            return self.pattern_match(pat[1], dat[1], f1)
        return None

    def rename(self, obj: Any, rule_id: int) -> Any:
        if isinstance(obj, Var):
            return Var(rule_id, obj.name)
        if isinstance(obj, tuple) and len(obj) == 2:
            return (self.rename(obj[0], rule_id), self.rename(obj[1], rule_id))
        if isinstance(obj, list):
            return [self.rename(x, rule_id) for x in obj]
        return obj

    def simple_query(self, pat: Any, frames: list[dict[Var, Any]]) -> list[dict[Var, Any]]:
        out: list[dict[Var, Any]] = []
        for frame in frames:
            for assertion in self.assertions:
                matched = self.pattern_match(pat, assertion, frame)
                if matched is not None:
                    out.append(matched)
            for conclusion, body in list(self.rules):
                self.rule_counter += 1
                rid = self.rule_counter
                r_conc = self.rename(conclusion, rid)
                unified = self.unify_match(pat, r_conc, frame)
                if unified is not None:
                    if body is None:
                        out.append(unified)
                    else:
                        out.extend(self.qeval(self.rename(body, rid), [unified]))
        return out

    def qeval(self, query: Any, frames: list[dict[Var, Any]]) -> list[dict[Var, Any]]:
        if isinstance(query, list) and query:
            op = query[0]
            if op == "and":
                cur = frames
                for q in query[1:]:
                    cur = self.qeval(q, cur)
                return cur
            if op == "or":
                streams = [self.qeval(q, list(frames)) for q in query[1:]]
                out = []
                max_len = max((len(s) for s in streams), default=0)
                for i in range(max_len):
                    for s in streams:
                        if i < len(s):
                            out.append(s[i])
                return out
            if op == "not":
                return [f for f in frames if not self.qeval(query[1], [f])]
            if op == "unique":
                out = []
                for f in frames:
                    res = self.qeval(query[1], [f])
                    if len(res) == 1:
                        out.append(res[0])
                return out
            if op == "same":
                return [
                    f
                    for f in frames
                    if self.instantiate(query[1], f) == self.instantiate(query[2], f)
                ]
        return self.simple_query(query, frames)

    def ask(self, pattern: Any, query: Any) -> list[Any]:
        return [self.instantiate(pattern, f) for f in self.qeval(query, [{}])]


def build_demo_db() -> QueryEngine:
    db = QueryEngine()
    for emp, sup in [("alyssa", "ben"), ("ben", "louis"), ("cy", "ben"), ("louis", "warbucks")]:
        db.add_assertion(lst("supervisor", emp, sup))
    v = lambda n: Var(0, n)
    db.add_rule(lst("outranked-by", v("staff"), v("boss")), lst("supervisor", v("staff"), v("boss")))
    db.add_rule(
        lst("outranked-by", v("staff"), v("boss")),
        [
            "and",
            lst("supervisor", v("staff"), v("mid")),
            lst("outranked-by", v("mid"), v("boss")),
        ],
    )
    db.add_rule(lst("append-to-form", (), v("y"), v("y")), None)
    db.add_rule(
        lst("append-to-form", dotted([v("u")], v("v")), v("y"), dotted([v("u")], v("z"))),
        lst("append-to-form", v("v"), v("y"), v("z")),
    )
    return db


class QueryUnificationTests(unittest.TestCase):
    def test_multi_hop_and_bidirectional_append(self) -> None:
        db = build_demo_db()
        bosses = db.ask(Var(0, "b"), lst("outranked-by", "alyssa", Var(0, "b")))
        self.assertEqual([format_term(b) for b in bosses], ["ben", "louis", "warbucks"])

        splits = db.ask(
            lst(Var(0, "x"), Var(0, "y")),
            lst("append-to-form", Var(0, "x"), Var(0, "y"), lst(1, 2, 3)),
        )
        self.assertEqual(
            [format_term(s) for s in splits],
            ["(() (1 2 3))", "((1) (2 3))", "((1 2) (3))", "((1 2 3) ())"],
        )

    def test_occurs_check_and_negation_unique(self) -> None:
        db = build_demo_db()
        self.assertIsNone(db.unify_match(Var(0, "x"), lst("f", Var(0, "x")), {}))
        top = db.ask(
            Var(0, "who"),
            [
                "and",
                lst("supervisor", Var(0, "any"), Var(0, "who")),
                ["not", lst("supervisor", Var(0, "who"), Var(0, "boss"))],
            ],
        )
        self.assertEqual([format_term(t) for t in top], ["warbucks"])


if __name__ == "__main__":
    db = build_demo_db()
    bosses = db.ask(Var(0, "b"), lst("outranked-by", "alyssa", Var(0, "b")))
    splits = db.ask(
        lst(Var(0, "x"), Var(0, "y")),
        lst("append-to-form", Var(0, "x"), Var(0, "y"), lst(1, 2, 3)),
    )
    print("alyssa outranked by:", [format_term(b) for b in bosses])
    print("splits of (1 2 3):", [format_term(s) for s in splits])
