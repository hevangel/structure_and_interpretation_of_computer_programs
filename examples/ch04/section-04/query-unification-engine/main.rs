use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Term {
    Atom(String),
    Num(i64),
    Var(usize, String),
    Nil,
    Pair(Box<Term>, Box<Term>),
}

impl Term {
    fn list(items: Vec<Term>) -> Term {
        let mut acc = Term::Nil;
        for item in items.into_iter().rev() {
            acc = Term::Pair(Box::new(item), Box::new(acc));
        }
        acc
    }

    fn dotted(mut items: Vec<Term>, tail: Term) -> Term {
        let mut acc = tail;
        while let Some(item) = items.pop() {
            acc = Term::Pair(Box::new(item), Box::new(acc));
        }
        acc
    }

    fn var(name: &str) -> Term {
        Term::Var(0, name.to_string())
    }

    fn atom(s: &str) -> Term {
        Term::Atom(s.to_string())
    }

    fn display(&self) -> String {
        match self {
            Term::Atom(s) => s.clone(),
            Term::Num(n) => n.to_string(),
            Term::Var(0, name) => format!("?{name}"),
            Term::Var(id, name) => format!("?{name}#{id}"),
            Term::Nil => "()".to_string(),
            Term::Pair(_, _) => {
                let mut parts = Vec::new();
                let mut cur = self;
                while let Term::Pair(a, d) = cur {
                    parts.push(a.display());
                    cur = d;
                }
                if *cur == Term::Nil {
                    format!("({})", parts.join(" "))
                } else {
                    format!("({} . {})", parts.join(" "), cur.display())
                }
            }
        }
    }
}

type VarId = (usize, String);
type Frame = BTreeMap<VarId, Term>;

#[derive(Clone, Debug)]
enum Query {
    Pattern(Term),
    And(Vec<Query>),
    Or(Vec<Query>),
    Not(Box<Query>),
    Unique(Box<Query>),
    Same(Term, Term),
    Lt(Term, Term),
    Neq(Term, Term),
}

#[derive(Clone, Debug)]
struct Rule {
    conclusion: Term,
    body: Option<Query>,
}

struct Database {
    assertions: Vec<Term>,
    rules: Vec<Rule>,
    rule_counter: usize,
}

impl Database {
    fn new() -> Self {
        Self {
            assertions: Vec::new(),
            rules: Vec::new(),
            rule_counter: 0,
        }
    }

    fn add_assertion(&mut self, term: Term) {
        self.assertions.push(term);
    }

    fn add_rule(&mut self, conclusion: Term, body: Option<Query>) {
        self.rules.push(Rule { conclusion, body });
    }

    fn instantiate(&self, term: &Term, frame: &Frame) -> Term {
        match term {
            Term::Var(id, name) => {
                if let Some(bound) = frame.get(&(*id, name.clone())) {
                    self.instantiate(bound, frame)
                } else {
                    term.clone()
                }
            }
            Term::Pair(a, d) => Term::Pair(
                Box::new(self.instantiate(a, frame)),
                Box::new(self.instantiate(d, frame)),
            ),
            other => other.clone(),
        }
    }

    fn depends_on(&self, exp: &Term, var: &VarId, frame: &Frame) -> bool {
        match exp {
            Term::Var(id, name) => {
                if (*id, name.clone()) == *var {
                    true
                } else if let Some(binding) = frame.get(&(*id, name.clone())) {
                    self.depends_on(binding, var, frame)
                } else {
                    false
                }
            }
            Term::Pair(a, d) => self.depends_on(a, var, frame) || self.depends_on(d, var, frame),
            _ => false,
        }
    }

    fn extend_if_possible(&self, var: VarId, val: &Term, frame: &Frame) -> Option<Frame> {
        if let Some(binding) = frame.get(&var) {
            return self.unify_match(binding, val, frame);
        }
        if let Term::Var(id2, name2) = val {
            let var2 = (*id2, name2.clone());
            if var == var2 {
                return Some(frame.clone());
            }
            if let Some(binding2) = frame.get(&var2) {
                return self.unify_match(&Term::Var(var.0, var.1), binding2, frame);
            }
        }
        if self.depends_on(val, &var, frame) {
            return None;
        }
        let mut next = frame.clone();
        next.insert(var, val.clone());
        Some(next)
    }

    fn unify_match(&self, p1: &Term, p2: &Term, frame: &Frame) -> Option<Frame> {
        if p1 == p2 {
            return Some(frame.clone());
        }
        if let Term::Var(id, name) = p1 {
            return self.extend_if_possible((*id, name.clone()), p2, frame);
        }
        if let Term::Var(id, name) = p2 {
            return self.extend_if_possible((*id, name.clone()), p1, frame);
        }
        match (p1, p2) {
            (Term::Pair(a1, d1), Term::Pair(a2, d2)) => {
                let f1 = self.unify_match(a1, a2, frame)?;
                self.unify_match(d1, d2, &f1)
            }
            _ => None,
        }
    }

    fn pattern_match(&self, pat: &Term, dat: &Term, frame: &Frame) -> Option<Frame> {
        if pat == dat {
            return Some(frame.clone());
        }
        if let Term::Var(id, name) = pat {
            let key = (*id, name.clone());
            if let Some(bound) = frame.get(&key) {
                return self.pattern_match(bound, dat, frame);
            }
            let mut next = frame.clone();
            next.insert(key, dat.clone());
            return Some(next);
        }
        match (pat, dat) {
            (Term::Pair(a1, d1), Term::Pair(a2, d2)) => {
                let f1 = self.pattern_match(a1, a2, frame)?;
                self.pattern_match(d1, d2, &f1)
            }
            _ => None,
        }
    }

    fn rename_term(&self, term: &Term, rule_id: usize) -> Term {
        match term {
            Term::Var(_, name) => Term::Var(rule_id, name.clone()),
            Term::Pair(a, d) => Term::Pair(
                Box::new(self.rename_term(a, rule_id)),
                Box::new(self.rename_term(d, rule_id)),
            ),
            other => other.clone(),
        }
    }

    fn rename_query(&self, query: &Query, rule_id: usize) -> Query {
        match query {
            Query::Pattern(t) => Query::Pattern(self.rename_term(t, rule_id)),
            Query::And(qs) => {
                Query::And(qs.iter().map(|q| self.rename_query(q, rule_id)).collect())
            }
            Query::Or(qs) => Query::Or(qs.iter().map(|q| self.rename_query(q, rule_id)).collect()),
            Query::Not(q) => Query::Not(Box::new(self.rename_query(q, rule_id))),
            Query::Unique(q) => Query::Unique(Box::new(self.rename_query(q, rule_id))),
            Query::Same(a, b) => {
                Query::Same(self.rename_term(a, rule_id), self.rename_term(b, rule_id))
            }
            Query::Lt(a, b) => {
                Query::Lt(self.rename_term(a, rule_id), self.rename_term(b, rule_id))
            }
            Query::Neq(a, b) => {
                Query::Neq(self.rename_term(a, rule_id), self.rename_term(b, rule_id))
            }
        }
    }

    fn simple_query(&mut self, pat: &Term, frames: Vec<Frame>) -> Vec<Frame> {
        let mut out = Vec::new();
        for frame in &frames {
            for assertion in &self.assertions {
                if let Some(f) = self.pattern_match(pat, assertion, frame) {
                    out.push(f);
                }
            }
            let rules_snapshot = self.rules.clone();
            for rule in &rules_snapshot {
                self.rule_counter += 1;
                let rid = self.rule_counter;
                let renamed_conc = self.rename_term(&rule.conclusion, rid);
                if let Some(unified) = self.unify_match(pat, &renamed_conc, frame) {
                    match &rule.body {
                        None => out.push(unified),
                        Some(body) => {
                            let renamed_body = self.rename_query(body, rid);
                            out.extend(self.qeval(&renamed_body, vec![unified]));
                        }
                    }
                }
            }
        }
        out
    }

    fn qeval(&mut self, query: &Query, frames: Vec<Frame>) -> Vec<Frame> {
        match query {
            Query::Pattern(pat) => self.simple_query(pat, frames),
            Query::And(conjuncts) => {
                let mut cur = frames;
                for q in conjuncts {
                    cur = self.qeval(q, cur);
                }
                cur
            }
            Query::Or(disjuncts) => {
                let streams: Vec<Vec<Frame>> = disjuncts
                    .iter()
                    .map(|q| self.qeval(q, frames.clone()))
                    .collect();
                let mut out = Vec::new();
                let max_len = streams.iter().map(Vec::len).max().unwrap_or(0);
                for idx in 0..max_len {
                    for s in &streams {
                        if let Some(f) = s.get(idx) {
                            out.push(f.clone());
                        }
                    }
                }
                out
            }
            Query::Not(inner) => {
                let mut out = Vec::new();
                for frame in frames {
                    if self.qeval(inner, vec![frame.clone()]).is_empty() {
                        out.push(frame);
                    }
                }
                out
            }
            Query::Unique(inner) => {
                let mut out = Vec::new();
                for frame in frames {
                    let res = self.qeval(inner, vec![frame]);
                    if res.len() == 1 {
                        out.push(res.into_iter().next().unwrap());
                    }
                }
                out
            }
            Query::Same(a, b) => frames
                .into_iter()
                .filter(|f| self.instantiate(a, f) == self.instantiate(b, f))
                .collect(),
            Query::Neq(a, b) => frames
                .into_iter()
                .filter(|f| self.instantiate(a, f) != self.instantiate(b, f))
                .collect(),
            Query::Lt(a, b) => frames
                .into_iter()
                .filter(|f| {
                    matches!(
                        (self.instantiate(a, f), self.instantiate(b, f)),
                        (Term::Num(x), Term::Num(y)) if x < y
                    )
                })
                .collect(),
        }
    }

    fn ask(&mut self, pattern: &Term, query: &Query) -> Vec<Term> {
        let frames = self.qeval(query, vec![BTreeMap::new()]);
        frames
            .iter()
            .map(|f| self.instantiate(pattern, f))
            .collect()
    }
}

fn build_demo_db() -> Database {
    let mut db = Database::new();
    for (emp, sup) in [
        ("alyssa", "ben"),
        ("ben", "louis"),
        ("cy", "ben"),
        ("louis", "warbucks"),
    ] {
        db.add_assertion(Term::list(vec![
            Term::atom("supervisor"),
            Term::atom(emp),
            Term::atom(sup),
        ]));
    }
    // Base rule: direct supervisor outranks
    db.add_rule(
        Term::list(vec![
            Term::atom("outranked-by"),
            Term::var("staff"),
            Term::var("boss"),
        ]),
        Some(Query::Pattern(Term::list(vec![
            Term::atom("supervisor"),
            Term::var("staff"),
            Term::var("boss"),
        ]))),
    );
    // Recursive rule
    db.add_rule(
        Term::list(vec![
            Term::atom("outranked-by"),
            Term::var("staff"),
            Term::var("boss"),
        ]),
        Some(Query::And(vec![
            Query::Pattern(Term::list(vec![
                Term::atom("supervisor"),
                Term::var("staff"),
                Term::var("mid"),
            ])),
            Query::Pattern(Term::list(vec![
                Term::atom("outranked-by"),
                Term::var("mid"),
                Term::var("boss"),
            ])),
        ])),
    );
    // append-to-form rules
    db.add_rule(
        Term::list(vec![
            Term::atom("append-to-form"),
            Term::Nil,
            Term::var("y"),
            Term::var("y"),
        ]),
        None,
    );
    db.add_rule(
        Term::list(vec![
            Term::atom("append-to-form"),
            Term::dotted(vec![Term::var("u")], Term::var("v")),
            Term::var("y"),
            Term::dotted(vec![Term::var("u")], Term::var("z")),
        ]),
        Some(Query::Pattern(Term::list(vec![
            Term::atom("append-to-form"),
            Term::var("v"),
            Term::var("y"),
            Term::var("z"),
        ]))),
    );
    db
}

fn main() {
    let mut db = build_demo_db();
    let bosses = db.ask(
        &Term::var("boss"),
        &Query::Pattern(Term::list(vec![
            Term::atom("outranked-by"),
            Term::atom("alyssa"),
            Term::var("boss"),
        ])),
    );
    println!(
        "alyssa outranked by: {:?}",
        bosses.iter().map(Term::display).collect::<Vec<_>>()
    );

    let splits = db.ask(
        &Term::list(vec![Term::var("x"), Term::var("y")]),
        &Query::Pattern(Term::list(vec![
            Term::atom("append-to-form"),
            Term::var("x"),
            Term::var("y"),
            Term::list(vec![Term::Num(1), Term::Num(2), Term::Num(3)]),
        ])),
    );
    let top = db.ask(
        &Term::var("sup"),
        &Query::And(vec![
            Query::Pattern(Term::list(vec![
                Term::atom("supervisor"),
                Term::var("e"),
                Term::var("sup"),
            ])),
            Query::Or(vec![
                Query::Same(Term::var("sup"), Term::atom("ben")),
                Query::Same(Term::var("sup"), Term::atom("louis")),
            ]),
            Query::Unique(Box::new(Query::Pattern(Term::list(vec![
                Term::atom("supervisor"),
                Term::var("e2"),
                Term::var("sup"),
            ])))),
            Query::Not(Box::new(Query::Neq(Term::var("sup"), Term::atom("louis")))),
        ]),
    );
    let _lt_ok = !db
        .qeval(
            &Query::Lt(Term::Num(1), Term::Num(2)),
            vec![BTreeMap::new()],
        )
        .is_empty();
    println!(
        "splits of (1 2 3): {:?}; unique non-ben sup: {:?}",
        splits.iter().map(Term::display).collect::<Vec<_>>(),
        top.iter().map(Term::display).collect::<Vec<_>>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_hop_rules_and_bidirectional_append_work() {
        let mut db = build_demo_db();
        let bosses: Vec<String> = db
            .ask(
                &Term::var("b"),
                &Query::Pattern(Term::list(vec![
                    Term::atom("outranked-by"),
                    Term::atom("alyssa"),
                    Term::var("b"),
                ])),
            )
            .iter()
            .map(Term::display)
            .collect();
        assert_eq!(bosses, vec!["ben", "louis", "warbucks"]);

        let splits: Vec<String> = db
            .ask(
                &Term::list(vec![Term::var("x"), Term::var("y")]),
                &Query::Pattern(Term::list(vec![
                    Term::atom("append-to-form"),
                    Term::var("x"),
                    Term::var("y"),
                    Term::list(vec![Term::Num(1), Term::Num(2), Term::Num(3)]),
                ])),
            )
            .iter()
            .map(Term::display)
            .collect();
        assert_eq!(
            splits,
            vec!["(() (1 2 3))", "((1) (2 3))", "((1 2) (3))", "((1 2 3) ())",]
        );
    }

    #[test]
    fn cyclic_occurs_check_and_negation_unique_work() {
        let db = build_demo_db();
        let self_ref = Term::list(vec![Term::atom("f"), Term::var("x")]);
        assert!(db
            .unify_match(&Term::var("x"), &self_ref, &BTreeMap::new())
            .is_none());

        let mut db = build_demo_db();
        // Who is NOT a supervisor of anyone? alyssa and cy (and warbucks is nobody's subordinate, wait supervisor(emp, sup))
        let top = db.ask(
            &Term::var("who"),
            &Query::And(vec![
                Query::Pattern(Term::list(vec![
                    Term::atom("supervisor"),
                    Term::var("any"),
                    Term::var("who"),
                ])),
                Query::Not(Box::new(Query::Pattern(Term::list(vec![
                    Term::atom("supervisor"),
                    Term::var("who"),
                    Term::var("boss"),
                ])))),
            ]),
        );
        // Distinct check: louis->warbucks means warbucks appears once as supervisor of louis and has no boss.
        assert_eq!(
            top.iter().map(Term::display).collect::<Vec<_>>(),
            vec!["warbucks"]
        );

        let single_report = db.ask(
            &Term::var("sup"),
            &Query::And(vec![
                Query::Pattern(Term::list(vec![
                    Term::atom("supervisor"),
                    Term::var("emp_seed"),
                    Term::var("sup"),
                ])),
                Query::Or(vec![
                    Query::Same(Term::var("sup"), Term::atom("ben")),
                    Query::Same(Term::var("sup"), Term::atom("louis")),
                ]),
                Query::Unique(Box::new(Query::Pattern(Term::list(vec![
                    Term::atom("supervisor"),
                    Term::var("emp"),
                    Term::var("sup"),
                ])))),
            ]),
        );
        // ben supervises both alyssa and cy (2), whereas louis supervises only ben (1).
        assert_eq!(
            single_report.iter().map(Term::display).collect::<Vec<_>>(),
            vec!["louis"]
        );

        let lt_res = db.qeval(
            &Query::Lt(Term::Num(2), Term::Num(5)),
            vec![BTreeMap::new()],
        );
        assert_eq!(lt_res.len(), 1);
        let neq_res = db.qeval(
            &Query::Neq(Term::atom("a"), Term::atom("a")),
            vec![BTreeMap::new()],
        );
        assert!(neq_res.is_empty());
    }
}
