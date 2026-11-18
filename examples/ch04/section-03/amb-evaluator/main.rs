use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Num(i64),
    Bool(bool),
    Sym(String),
    List(Vec<Expr>),
}

fn tokenize(src: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == ';' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if c == '(' || c == ')' || c == '\'' {
            tokens.push(c.to_string());
            i += 1;
        } else {
            let mut j = i;
            while j < chars.len() && !chars[j].is_whitespace() && chars[j] != '(' && chars[j] != ')'
            {
                j += 1;
            }
            tokens.push(chars[i..j].iter().collect());
            i = j;
        }
    }
    tokens
}

fn parse_tokens(tokens: &[String], pos: &mut usize) -> Result<Expr, String> {
    let tok = tokens
        .get(*pos)
        .ok_or_else(|| "unexpected EOF".to_string())?
        .clone();
    *pos += 1;
    match tok.as_str() {
        "(" => {
            let mut items = Vec::new();
            while tokens.get(*pos).map(String::as_str) != Some(")") {
                if *pos >= tokens.len() {
                    return Err("unclosed parenthesis".to_string());
                }
                items.push(parse_tokens(tokens, pos)?);
            }
            *pos += 1;
            Ok(Expr::List(items))
        }
        "'" => Ok(Expr::List(vec![
            Expr::Sym("quote".into()),
            parse_tokens(tokens, pos)?,
        ])),
        "#t" | "true" => Ok(Expr::Bool(true)),
        "#f" | "false" => Ok(Expr::Bool(false)),
        other => {
            if let Ok(n) = other.parse::<i64>() {
                Ok(Expr::Num(n))
            } else {
                Ok(Expr::Sym(other.to_string()))
            }
        }
    }
}

fn parse_program(src: &str) -> Result<Vec<Expr>, String> {
    let tokens = tokenize(src);
    let mut pos = 0;
    let mut out = Vec::new();
    while pos < tokens.len() {
        out.push(parse_tokens(&tokens, &mut pos)?);
    }
    Ok(out)
}

#[derive(Clone, Debug)]
struct Frame {
    bindings: BTreeMap<String, Value>,
    parent: Option<Rc<RefCell<Frame>>>,
}

type Env = Rc<RefCell<Frame>>;

#[derive(Clone)]
enum Value {
    Num(i64),
    Bool(bool),
    Sym(String),
    Nil,
    Pair(Box<Value>, Box<Value>),
    Primitive(String),
    Closure {
        params: Vec<String>,
        body: AmbExec,
        env: Env,
    },
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Sym(s) => write!(f, "{s}"),
            Value::Nil => write!(f, "()"),
            Value::Pair(a, d) => write!(f, "({a:?} . {d:?})"),
            Value::Primitive(p) => write!(f, "<prim:{p}>"),
            Value::Closure { .. } => write!(f, "<closure>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Sym(a), Value::Sym(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Pair(a1, d1), Value::Pair(a2, d2)) => a1 == a2 && d1 == d2,
            _ => false,
        }
    }
}

impl Value {
    fn as_num(&self) -> Result<i64, String> {
        match self {
            Value::Num(n) => Ok(*n),
            other => Err(format!("expected number, got {other:?}")),
        }
    }

    fn is_truthy(&self) -> bool {
        !matches!(self, Value::Bool(false))
    }

    fn format_sexp(&self) -> String {
        match self {
            Value::Pair(_, _) => {
                let mut parts = Vec::new();
                let mut cur = self;
                while let Value::Pair(car, cdr) = cur {
                    parts.push(car.format_sexp());
                    cur = cdr;
                }
                if *cur == Value::Nil {
                    format!("({})", parts.join(" "))
                } else {
                    format!("({} . {})", parts.join(" "), cur.format_sexp())
                }
            }
            _ => format!("{self:?}"),
        }
    }
}

enum Bounce {
    Done,
    Step(Box<dyn FnOnce() -> Bounce>),
}

type FailFn = Box<dyn FnOnce() -> Bounce>;
type SucceedFn = Rc<dyn Fn(Value, FailFn) -> Bounce>;
type AmbExec = Rc<dyn Fn(Env, SucceedFn, FailFn) -> Bounce>;

fn lookup_var(env: &Env, name: &str) -> Result<Value, String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(frame_rc) = cur {
        let frame = frame_rc.borrow();
        if let Some(val) = frame.bindings.get(name) {
            return Ok(val.clone());
        }
        cur = frame.parent.clone();
    }
    Err(format!("unbound variable: {name}"))
}

fn set_var(env: &Env, name: &str, val: Value) -> Result<Value, String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(frame_rc) = cur {
        let mut frame = frame_rc.borrow_mut();
        if frame.bindings.contains_key(name) {
            let old = frame.bindings.insert(name.to_string(), val).unwrap();
            return Ok(old);
        }
        cur = frame.parent.clone();
    }
    Err(format!("unbound variable for set!: {name}"))
}

fn text_of_quotation(expr: &Expr) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Sym(s) => Value::Sym(s.clone()),
        Expr::List(items) => {
            let mut acc = Value::Nil;
            for item in items.iter().rev() {
                acc = Value::Pair(Box::new(text_of_quotation(item)), Box::new(acc));
            }
            acc
        }
    }
}

fn analyze_sequence(exprs: &[Expr]) -> Result<AmbExec, String> {
    if exprs.is_empty() {
        return Err("empty sequence".to_string());
    }
    let mut procs = Vec::with_capacity(exprs.len());
    for e in exprs {
        procs.push(analyze(e)?);
    }
    let mut acc = procs.remove(0);
    for next in procs {
        let first = acc;
        acc = Rc::new(move |env, succeed, fail| {
            let next_c = Rc::clone(&next);
            let env_c = Rc::clone(&env);
            first(
                env,
                Rc::new(move |_, fail2| {
                    let next_inner = Rc::clone(&next_c);
                    let env_inner = Rc::clone(&env_c);
                    let succeed_c = Rc::clone(&succeed);
                    Bounce::Step(Box::new(move || next_inner(env_inner, succeed_c, fail2)))
                }),
                fail,
            )
        });
    }
    Ok(acc)
}

fn try_choices(
    choices: Vec<AmbExec>,
    idx: usize,
    env: Env,
    succeed: SucceedFn,
    fail: FailFn,
) -> Bounce {
    if idx >= choices.len() {
        return Bounce::Step(fail);
    }
    let choice = Rc::clone(&choices[idx]);
    let rest_choices = choices;
    let env_c = Rc::clone(&env);
    let succeed_c = Rc::clone(&succeed);
    choice(
        env,
        succeed,
        Box::new(move || {
            Bounce::Step(Box::new(move || {
                try_choices(rest_choices, idx + 1, env_c, succeed_c, fail)
            }))
        }),
    )
}

fn eval_args(
    args: &[AmbExec],
    idx: usize,
    env: Env,
    collected: Vec<Value>,
    succeed: Rc<dyn Fn(Vec<Value>, FailFn) -> Bounce>,
    fail: FailFn,
) -> Bounce {
    if idx >= args.len() {
        return succeed(collected, fail);
    }
    let arg_exec = Rc::clone(&args[idx]);
    let rest = args.to_vec();
    let env_c = Rc::clone(&env);
    arg_exec(
        env,
        Rc::new(move |val, fail2| {
            let mut next_vals = collected.clone();
            next_vals.push(val);
            eval_args(
                &rest,
                idx + 1,
                Rc::clone(&env_c),
                next_vals,
                Rc::clone(&succeed),
                fail2,
            )
        }),
        fail,
    )
}

fn apply_primitive(op: &str, args: &[Value]) -> Result<Value, String> {
    match op {
        "+" => Ok(Value::Num(args.iter().map(|v| v.as_num().unwrap()).sum())),
        "*" => Ok(Value::Num(
            args.iter().map(|v| v.as_num().unwrap()).product(),
        )),
        "-" => Ok(Value::Num(args[0].as_num()? - args[1].as_num()?)),
        "=" => Ok(Value::Bool(args[0] == args[1])),
        "<" => Ok(Value::Bool(args[0].as_num()? < args[1].as_num()?)),
        ">" => Ok(Value::Bool(args[0].as_num()? > args[1].as_num()?)),
        "not" => Ok(Value::Bool(!args[0].is_truthy())),
        "null?" => Ok(Value::Bool(args[0] == Value::Nil)),
        "pair?" => Ok(Value::Bool(matches!(args[0], Value::Pair(_, _)))),
        "eq?" => Ok(Value::Bool(args[0] == args[1])),
        "cons" => Ok(Value::Pair(
            Box::new(args[0].clone()),
            Box::new(args[1].clone()),
        )),
        "car" => match &args[0] {
            Value::Pair(a, _) => Ok((**a).clone()),
            other => Err(format!("car on non-pair: {other:?}")),
        },
        "cdr" => match &args[0] {
            Value::Pair(_, d) => Ok((**d).clone()),
            other => Err(format!("cdr on non-pair: {other:?}")),
        },
        "list" => {
            let mut acc = Value::Nil;
            for item in args.iter().rev() {
                acc = Value::Pair(Box::new(item.clone()), Box::new(acc));
            }
            Ok(acc)
        }
        "abs" => Ok(Value::Num(args[0].as_num()?.abs())),
        _ => Err(format!("unknown primitive {op}")),
    }
}

fn execute_application(
    proc_val: Value,
    args: Vec<Value>,
    succeed: SucceedFn,
    fail: FailFn,
) -> Bounce {
    match proc_val {
        Value::Primitive(op) => match apply_primitive(&op, &args) {
            Ok(v) => succeed(v, fail),
            Err(_) => Bounce::Step(fail),
        },
        Value::Closure { params, body, env } => {
            if params.len() != args.len() {
                return Bounce::Step(fail);
            }
            let mut bindings = BTreeMap::new();
            for (p, a) in params.into_iter().zip(args) {
                bindings.insert(p, a);
            }
            let frame = Rc::new(RefCell::new(Frame {
                bindings,
                parent: Some(env),
            }));
            Bounce::Step(Box::new(move || body(frame, succeed, fail)))
        }
        _ => Bounce::Step(fail),
    }
}

fn analyze(expr: &Expr) -> Result<AmbExec, String> {
    match expr {
        Expr::Num(n) => {
            let val = Value::Num(*n);
            Ok(Rc::new(move |_, succeed, fail| succeed(val.clone(), fail)))
        }
        Expr::Bool(b) => {
            let val = Value::Bool(*b);
            Ok(Rc::new(move |_, succeed, fail| succeed(val.clone(), fail)))
        }
        Expr::Sym(s) => {
            let name = s.clone();
            Ok(Rc::new(move |env, succeed, fail| {
                match lookup_var(&env, &name) {
                    Ok(v) => succeed(v, fail),
                    Err(_) => Bounce::Step(fail),
                }
            }))
        }
        Expr::List(items) if items.is_empty() => {
            Ok(Rc::new(|_, succeed, fail| succeed(Value::Nil, fail)))
        }
        Expr::List(items) => {
            if let Expr::Sym(head) = &items[0] {
                match head.as_str() {
                    "quote" => {
                        let quoted = text_of_quotation(&items[1]);
                        return Ok(Rc::new(move |_, succeed, fail| {
                            succeed(quoted.clone(), fail)
                        }));
                    }
                    "amb" => {
                        let choices = items[1..]
                            .iter()
                            .map(analyze)
                            .collect::<Result<Vec<_>, _>>()?;
                        return Ok(Rc::new(move |env, succeed, fail| {
                            try_choices(choices.clone(), 0, env, succeed, fail)
                        }));
                    }
                    "require" => {
                        let pred = analyze(&items[1])?;
                        return Ok(Rc::new(move |env, succeed, fail| {
                            pred(
                                env,
                                Rc::new(move |v, fail2| {
                                    if v.is_truthy() {
                                        succeed(Value::Sym("ok".into()), fail2)
                                    } else {
                                        Bounce::Step(fail2)
                                    }
                                }),
                                fail,
                            )
                        }));
                    }
                    "if" => {
                        let pred = analyze(&items[1])?;
                        let cseq = analyze(&items[2])?;
                        let alt: AmbExec = if items.len() > 3 {
                            analyze(&items[3])?
                        } else {
                            Rc::new(|_, s, f| s(Value::Bool(false), f))
                        };
                        return Ok(Rc::new(move |env, succeed, fail| {
                            let cseq_c = Rc::clone(&cseq);
                            let alt_c = Rc::clone(&alt);
                            let env_c = Rc::clone(&env);
                            pred(
                                env,
                                Rc::new(move |pv, fail2| {
                                    if pv.is_truthy() {
                                        cseq_c(Rc::clone(&env_c), Rc::clone(&succeed), fail2)
                                    } else {
                                        alt_c(Rc::clone(&env_c), Rc::clone(&succeed), fail2)
                                    }
                                }),
                                fail,
                            )
                        }));
                    }
                    "if-fail" => {
                        let first = analyze(&items[1])?;
                        let second = analyze(&items[2])?;
                        return Ok(Rc::new(move |env, succeed, fail| {
                            let env_c = Rc::clone(&env);
                            let succeed_c = Rc::clone(&succeed);
                            let second_c = Rc::clone(&second);
                            first(
                                env,
                                succeed,
                                Box::new(move || second_c(env_c, succeed_c, fail)),
                            )
                        }));
                    }
                    "begin" => return analyze_sequence(&items[1..]),
                    "lambda" => {
                        let params = match &items[1] {
                            Expr::List(ps) => ps
                                .iter()
                                .map(|p| match p {
                                    Expr::Sym(s) => Ok(s.clone()),
                                    _ => Err("lambda param must be symbol".to_string()),
                                })
                                .collect::<Result<Vec<_>, String>>()?,
                            _ => return Err("malformed lambda params".into()),
                        };
                        let body = analyze_sequence(&items[2..])?;
                        return Ok(Rc::new(move |env, succeed, fail| {
                            succeed(
                                Value::Closure {
                                    params: params.clone(),
                                    body: Rc::clone(&body),
                                    env,
                                },
                                fail,
                            )
                        }));
                    }
                    "define" => {
                        let (name, rhs_exec) = match &items[1] {
                            Expr::Sym(n) => (n.clone(), analyze(&items[2])?),
                            Expr::List(sig) => {
                                let n = match &sig[0] {
                                    Expr::Sym(s) => s.clone(),
                                    _ => return Err("proc name must be symbol".into()),
                                };
                                let params = sig[1..]
                                    .iter()
                                    .map(|p| match p {
                                        Expr::Sym(s) => Ok(s.clone()),
                                        _ => Err("param must be symbol".to_string()),
                                    })
                                    .collect::<Result<Vec<_>, String>>()?;
                                let body = analyze_sequence(&items[2..])?;
                                let lam: AmbExec = Rc::new(move |env, succeed, fail| {
                                    succeed(
                                        Value::Closure {
                                            params: params.clone(),
                                            body: Rc::clone(&body),
                                            env,
                                        },
                                        fail,
                                    )
                                });
                                (n, lam)
                            }
                            _ => return Err("malformed define".into()),
                        };
                        return Ok(Rc::new(move |env, succeed, fail| {
                            let name_c = name.clone();
                            let env_c = Rc::clone(&env);
                            rhs_exec(
                                env,
                                Rc::new(move |val, fail2| {
                                    env_c.borrow_mut().bindings.insert(name_c.clone(), val);
                                    succeed(Value::Sym("ok".into()), fail2)
                                }),
                                fail,
                            )
                        }));
                    }
                    "set!" => {
                        let name = match &items[1] {
                            Expr::Sym(s) => s.clone(),
                            _ => return Err("set! target must be symbol".into()),
                        };
                        let val_exec = analyze(&items[2])?;
                        return Ok(Rc::new(move |env, succeed, fail| {
                            let name_c = name.clone();
                            let env_c = Rc::clone(&env);
                            val_exec(
                                env,
                                Rc::new(move |new_val, fail2| {
                                    let Ok(old_val) = set_var(&env_c, &name_c, new_val) else {
                                        return Bounce::Step(fail2);
                                    };
                                    let env_undo = Rc::clone(&env_c);
                                    let name_undo = name_c.clone();
                                    succeed(
                                        Value::Sym("ok".into()),
                                        Box::new(move || {
                                            let _ = set_var(&env_undo, &name_undo, old_val);
                                            Bounce::Step(fail2)
                                        }),
                                    )
                                }),
                                fail,
                            )
                        }));
                    }
                    "permanent-set!" => {
                        let name = match &items[1] {
                            Expr::Sym(s) => s.clone(),
                            _ => return Err("permanent-set! target must be symbol".into()),
                        };
                        let val_exec = analyze(&items[2])?;
                        return Ok(Rc::new(move |env, succeed, fail| {
                            let name_c = name.clone();
                            let env_c = Rc::clone(&env);
                            val_exec(
                                env,
                                Rc::new(move |new_val, fail2| {
                                    if set_var(&env_c, &name_c, new_val).is_err() {
                                        return Bounce::Step(fail2);
                                    }
                                    succeed(Value::Sym("ok".into()), fail2)
                                }),
                                fail,
                            )
                        }));
                    }
                    _ => {}
                }
            }
            let fproc = analyze(&items[0])?;
            let aprocs = items[1..]
                .iter()
                .map(analyze)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Rc::new(move |env, succeed, fail| {
                let aprocs_c = aprocs.clone();
                let env_c = Rc::clone(&env);
                fproc(
                    env,
                    Rc::new(move |proc_val, fail2| {
                        let succeed_c = Rc::clone(&succeed);
                        eval_args(
                            &aprocs_c,
                            0,
                            Rc::clone(&env_c),
                            Vec::new(),
                            Rc::new(move |args, fail3| {
                                execute_application(
                                    proc_val.clone(),
                                    args,
                                    Rc::clone(&succeed_c),
                                    fail3,
                                )
                            }),
                            fail2,
                        )
                    }),
                    fail,
                )
            }))
        }
    }
}

fn default_env() -> Env {
    let mut bindings = BTreeMap::new();
    for p in [
        "+", "-", "*", "=", "<", ">", "not", "null?", "pair?", "eq?", "cons", "car", "cdr", "list",
        "abs",
    ] {
        bindings.insert(p.to_string(), Value::Primitive(p.to_string()));
    }
    Rc::new(RefCell::new(Frame {
        bindings,
        parent: None,
    }))
}

fn collect_all(src: &str) -> Result<Vec<Value>, String> {
    let exprs = parse_program(src)?;
    let program = analyze_sequence(&exprs)?;
    let env = default_env();
    let results = Rc::new(RefCell::new(Vec::new()));
    let results_c = Rc::clone(&results);
    let mut step = program(
        env,
        Rc::new(move |val, fail| {
            results_c.borrow_mut().push(val);
            Bounce::Step(fail)
        }),
        Box::new(|| Bounce::Done),
    );
    while let Bounce::Step(thunk) = step {
        step = thunk();
    }
    let out = results.borrow().clone();
    Ok(out)
}

fn main() {
    let triples = collect_all(
        "
        (define (an-integer-between low high)
          (require (not (> low high)))
          (amb low (an-integer-between (+ low 1) high)))
        (define a (an-integer-between 1 13))
        (define b (an-integer-between a 13))
        (define c (an-integer-between b 13))
        (require (= (+ (* a a) (* b b)) (* c c)))
        (list a b c)
        ",
    )
    .unwrap();
    for t in &triples {
        println!("triple: {}", t.format_sexp());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerates_pythagorean_triples_and_multiple_dwelling() {
        let triples = collect_all(
            "
            (define (an-integer-between low high)
              (require (not (> low high)))
              (amb low (an-integer-between (+ low 1) high)))
            (define a (an-integer-between 1 13))
            (define b (an-integer-between a 13))
            (define c (an-integer-between b 13))
            (require (= (+ (* a a) (* b b)) (* c c)))
            (list a b c)
            ",
        )
        .unwrap();
        let formatted: Vec<String> = triples.iter().map(Value::format_sexp).collect();
        assert_eq!(formatted, vec!["(3 4 5)", "(5 12 13)", "(6 8 10)"]);

        let dwelling = collect_all(
            "
            (define (distinct? items)
              (if (null? items) #t
                  (if (null? (cdr items)) #t
                      (if (member? (car items) (cdr items)) #f
                          (distinct? (cdr items))))))
            (define (member? x xs)
              (if (null? xs) #f
                  (if (= x (car xs)) #t (member? x (cdr xs)))))
            (define baker (amb 1 2 3 4 5))
            (define cooper (amb 1 2 3 4 5))
            (define fletcher (amb 1 2 3 4 5))
            (define miller (amb 1 2 3 4 5))
            (define smith (amb 1 2 3 4 5))
            (require (distinct? (list baker cooper fletcher miller smith)))
            (require (not (= baker 5)))
            (require (not (= cooper 1)))
            (require (not (= fletcher 5)))
            (require (not (= fletcher 1)))
            (require (> miller cooper))
            (require (not (= (abs (- smith fletcher)) 1)))
            (require (not (= (abs (- fletcher cooper)) 1)))
            (list baker cooper fletcher miller smith)
            ",
        )
        .unwrap();
        assert_eq!(dwelling.len(), 1);
        assert_eq!(dwelling[0].format_sexp(), "(3 2 4 5 1)");
    }

    #[test]
    fn reversible_set_and_permanent_set_with_if_fail() {
        let out = collect_all(
            "
            (define count 0)
            (define temp 0)
            (if-fail
              (begin
                (define x (amb 1 3 5))
                (permanent-set! count (+ count 1))
                (set! temp (+ temp 10))
                (require (= x 9))
                x)
              (list count temp))
            ",
        )
        .unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].format_sexp(), "(3 0)");
    }

    #[test]
    fn parses_ambiguous_natural_language_sentences() {
        let parses = collect_all(
            "
            (define (memq item x)
              (if (null? x) #f
                  (if (eq? item (car x)) #t (memq item (cdr x)))))
            (define *unparsed* '())
            (define (parse-word word-list)
              (require (not (null? *unparsed*)))
              (require (memq (car *unparsed*) (cdr word-list)))
              (define found (car *unparsed*))
              (set! *unparsed* (cdr *unparsed*))
              (list (car word-list) found))
            (define nouns '(noun student professor cat class))
            (define verbs '(verb studies lectures eats sleeps))
            (define articles '(article the a))
            (define prepositions '(prep for to in by with))
            (define (parse-prepositional-phrase)
              (list 'prep-phrase
                    (parse-word prepositions)
                    (parse-noun-phrase)))
            (define (parse-simple-noun-phrase)
              (list 'simple-noun-phrase
                    (parse-word articles)
                    (parse-word nouns)))
            (define (parse-noun-phrase)
              (define (maybe-extend np)
                (amb np
                     (maybe-extend (list 'noun-phrase np (parse-prepositional-phrase)))))
              (maybe-extend (parse-simple-noun-phrase)))
            (define (parse-verb-phrase)
              (define (maybe-extend vp)
                (amb vp
                     (maybe-extend (list 'verb-phrase vp (parse-prepositional-phrase)))))
              (maybe-extend (parse-word verbs)))
            (define (parse input)
              (set! *unparsed* input)
              (define sent (list 'sentence (parse-noun-phrase) (parse-verb-phrase)))
              (require (null? *unparsed*))
              sent)
            (parse '(the cat eats in the class with the student))
            ",
        )
        .unwrap();
        assert_eq!(parses.len(), 2);
    }
}
