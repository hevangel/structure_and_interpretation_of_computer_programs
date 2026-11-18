use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
enum Expr {
    Num(i64),
    Bool(bool),
    Var(String),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Lambda(Vec<String>, Vec<Expr>),
    Define(String, Box<Expr>),
    Set(String, Box<Expr>),
    Let(Vec<(String, Expr)>, Vec<Expr>),
    Seq(Vec<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Clone)]
enum Value {
    Num(i64),
    Bool(bool),
    Prim(fn(&[Value]) -> Result<Value, String>),
    CompiledProc {
        params: Vec<String>,
        body: ExecFn,
        env: Env,
    },
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(n) => write!(f, "Num({n})"),
            Value::Bool(b) => write!(f, "Bool({b})"),
            Value::Prim(_) => write!(f, "<prim>"),
            Value::CompiledProc { params, .. } => write!(f, "<proc({params:?})>"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }
}

type Env = Rc<RefCell<Frame>>;

struct Frame {
    bindings: BTreeMap<String, Value>,
    parent: Option<Env>,
}

fn extend_env(parent: &Env, params: &[String], args: &[Value]) -> Result<Env, String> {
    if params.len() != args.len() {
        return Err("arity mismatch".into());
    }
    let mut bindings = BTreeMap::new();
    for (p, a) in params.iter().zip(args) {
        bindings.insert(p.clone(), a.clone());
    }
    Ok(Rc::new(RefCell::new(Frame {
        bindings,
        parent: Some(Rc::clone(parent)),
    })))
}

fn lookup(env: &Env, name: &str) -> Result<Value, String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(f) = cur {
        let b = f.borrow();
        if let Some(v) = b.bindings.get(name) {
            return Ok(v.clone());
        }
        cur = b.parent.clone();
    }
    Err(format!("unbound: {name}"))
}

fn set_var(env: &Env, name: &str, val: Value) -> Result<(), String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(f) = cur {
        let parent = {
            let mut b = f.borrow_mut();
            if b.bindings.contains_key(name) {
                b.bindings.insert(name.to_string(), val);
                return Ok(());
            }
            b.parent.clone()
        };
        cur = parent;
    }
    Err(format!("unbound set!: {name}"))
}

#[derive(Default)]
struct Counters {
    analyze_steps: Cell<usize>,
    exec_steps: Cell<usize>,
}

type ExecFn = Rc<dyn Fn(&Env, &Counters) -> Result<Value, String>>;

/// Section 4.1.7: syntactic analysis runs ONCE per AST node and returns a reusable closure.
fn analyze(expr: &Expr, counters: &Counters) -> ExecFn {
    counters.analyze_steps.set(counters.analyze_steps.get() + 1);
    match expr {
        Expr::Num(n) => {
            let v = *n;
            Rc::new(move |_, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                Ok(Value::Num(v))
            })
        }
        Expr::Bool(b) => {
            let v = *b;
            Rc::new(move |_, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                Ok(Value::Bool(v))
            })
        }
        Expr::Var(name) => {
            let n = name.clone();
            Rc::new(move |env, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                lookup(env, &n)
            })
        }
        Expr::If(pred, cons, alt) => {
            let p_fn = analyze(pred, counters);
            let c_fn = analyze(cons, counters);
            let a_fn = analyze(alt, counters);
            Rc::new(move |env, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                if matches!(p_fn(env, c)?, Value::Bool(false)) {
                    a_fn(env, c)
                } else {
                    c_fn(env, c)
                }
            })
        }
        Expr::Lambda(params, body) => {
            let ps = params.clone();
            let b_fn = analyze_sequence(body, counters);
            Rc::new(move |env, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                Ok(Value::CompiledProc {
                    params: ps.clone(),
                    body: Rc::clone(&b_fn),
                    env: Rc::clone(env),
                })
            })
        }
        Expr::Define(name, rhs) => {
            let n = name.clone();
            let v_fn = analyze(rhs, counters);
            Rc::new(move |env, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                let val = v_fn(env, c)?;
                env.borrow_mut().bindings.insert(n.clone(), val.clone());
                Ok(val)
            })
        }
        Expr::Set(name, rhs) => {
            let n = name.clone();
            let v_fn = analyze(rhs, counters);
            Rc::new(move |env, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                let val = v_fn(env, c)?;
                set_var(env, &n, val.clone())?;
                Ok(val)
            })
        }
        Expr::Let(bindings, body) => {
            // Exercise 4.22: expand `let` during analysis so execution never inspects binding syntax.
            let params: Vec<String> = bindings.iter().map(|(k, _)| k.clone()).collect();
            let arg_exprs: Vec<Expr> = bindings.iter().map(|(_, v)| v.clone()).collect();
            let lam = Expr::Call(Box::new(Expr::Lambda(params, body.clone())), arg_exprs);
            analyze(&lam, counters)
        }
        Expr::Seq(exprs) => analyze_sequence(exprs, counters),
        Expr::Call(proc_expr, arg_exprs) => {
            let f_proc = analyze(proc_expr, counters);
            let f_args: Vec<ExecFn> = arg_exprs.iter().map(|a| analyze(a, counters)).collect();
            Rc::new(move |env, c| {
                c.exec_steps.set(c.exec_steps.get() + 1);
                let proc = f_proc(env, c)?;
                let args: Vec<Value> = f_args
                    .iter()
                    .map(|af| af(env, c))
                    .collect::<Result<_, _>>()?;
                match proc {
                    Value::Prim(f) => f(&args),
                    Value::CompiledProc {
                        params,
                        body,
                        env: def_env,
                    } => {
                        let call_env = extend_env(&def_env, &params, &args)?;
                        body(&call_env, c)
                    }
                    _ => Err("cannot call non-procedure".into()),
                }
            })
        }
    }
}

fn analyze_sequence(exprs: &[Expr], counters: &Counters) -> ExecFn {
    let fns: Vec<ExecFn> = exprs.iter().map(|e| analyze(e, counters)).collect();
    Rc::new(move |env, c| {
        let mut last = Value::Bool(false);
        for f in &fns {
            last = f(env, c)?;
        }
        Ok(last)
    })
}

fn global_env() -> Env {
    let mut bindings = BTreeMap::new();
    bindings.insert(
        "*".into(),
        Value::Prim(|args| match args {
            [Value::Num(a), Value::Num(b)] => Ok(Value::Num(a * b)),
            _ => Err("bad *".into()),
        }),
    );
    bindings.insert(
        "-".into(),
        Value::Prim(|args| match args {
            [Value::Num(a), Value::Num(b)] => Ok(Value::Num(a - b)),
            _ => Err("bad -".into()),
        }),
    );
    bindings.insert(
        "=".into(),
        Value::Prim(|args| match args {
            [Value::Num(a), Value::Num(b)] => Ok(Value::Bool(a == b)),
            _ => Err("bad =".into()),
        }),
    );
    Rc::new(RefCell::new(Frame {
        bindings,
        parent: None,
    }))
}

fn factorial_ast(n: i64) -> Expr {
    Expr::Seq(vec![
        Expr::Define(
            "fact".into(),
            Box::new(Expr::Lambda(
                vec!["n".into()],
                vec![Expr::If(
                    Box::new(Expr::Call(
                        Box::new(Expr::Var("=".into())),
                        vec![Expr::Var("n".into()), Expr::Num(0)],
                    )),
                    Box::new(Expr::Num(1)),
                    Box::new(Expr::Call(
                        Box::new(Expr::Var("*".into())),
                        vec![
                            Expr::Var("n".into()),
                            Expr::Call(
                                Box::new(Expr::Var("fact".into())),
                                vec![Expr::Call(
                                    Box::new(Expr::Var("-".into())),
                                    vec![Expr::Var("n".into()), Expr::Num(1)],
                                )],
                            ),
                        ],
                    )),
                )],
            )),
        ),
        Expr::Let(
            vec![("k".into(), Expr::Num(n))],
            vec![Expr::Call(
                Box::new(Expr::Var("fact".into())),
                vec![Expr::Var("k".into())],
            )],
        ),
    ])
}

fn main() {
    let counters = Counters::default();
    let compiled = analyze(&factorial_ast(10), &counters);
    let after_analyze = counters.analyze_steps.get();
    let env = global_env();
    let val = compiled(&env, &counters).unwrap();
    let extra = analyze(
        &Expr::Seq(vec![
            Expr::Define("flag".into(), Box::new(Expr::Bool(true))),
            Expr::Set("flag".into(), Box::new(Expr::Bool(false))),
            Expr::If(
                Box::new(Expr::Var("flag".into())),
                Box::new(Expr::Num(1)),
                Box::new(Expr::Num(2)),
            ),
        ]),
        &counters,
    );
    let extra_val = extra(&env, &counters).unwrap();
    println!(
        "fact(10)={:?}, extra={:?}, analyze_steps={} (frozen before run!), exec_steps={}",
        val,
        extra_val,
        after_analyze,
        counters.exec_steps.get()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_runs_once_even_across_repeated_loops() {
        let counters = Counters::default();
        let compiled = analyze(&factorial_ast(10), &counters);
        let static_analyses = counters.analyze_steps.get();
        assert!(static_analyses > 0);

        let env = global_env();
        let first = compiled(&env, &counters).unwrap();
        assert_eq!(first, Value::Num(3_628_800));
        // Zero additional AST analysis steps during execution.
        assert_eq!(counters.analyze_steps.get(), static_analyses);
        assert!(counters.exec_steps.get() > static_analyses);

        // Re-running compiled program still triggers 0 AST analyses.
        let mut_prog = analyze(
            &Expr::Seq(vec![
                Expr::Define("x".into(), Box::new(Expr::Num(1))),
                Expr::Set("x".into(), Box::new(Expr::Num(7))),
                Expr::Call(
                    Box::new(Expr::Var("fact".into())),
                    vec![Expr::Var("x".into())],
                ),
            ]),
            &counters,
        );
        let after_second_analyze = counters.analyze_steps.get();
        assert_eq!(mut_prog(&env, &counters).unwrap(), Value::Num(5040));
        assert_eq!(counters.analyze_steps.get(), after_second_analyze);

        let b_expr = analyze(
            &Expr::If(
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Num(1)),
                Box::new(Expr::Num(2)),
            ),
            &counters,
        );
        assert_eq!(b_expr(&env, &counters).unwrap(), Value::Num(1));
    }
}
