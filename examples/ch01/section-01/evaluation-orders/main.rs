use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    EqZero(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Block {
        defs: Vec<(String, Expr)>,
        body: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
struct Proc {
    params: Vec<String>,
    body: Expr,
}

#[derive(Clone, Debug)]
enum Thunk {
    Value(i64),
    Deferred {
        expr: Expr,
        env: BTreeMap<String, Thunk>,
    },
}

#[derive(Default)]
struct Program {
    procs: BTreeMap<String, Proc>,
    steps: usize,
    step_limit: usize,
}

impl Program {
    fn new(step_limit: usize) -> Self {
        Self {
            procs: BTreeMap::new(),
            steps: 0,
            step_limit,
        }
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        self.steps += 1;
        if self.steps > self.step_limit {
            Err("evaluation exceeded step limit (non-terminating)")
        } else {
            Ok(())
        }
    }

    fn eval_applicative(
        &mut self,
        expr: &Expr,
        env: &BTreeMap<String, i64>,
    ) -> Result<i64, &'static str> {
        self.tick()?;
        match expr {
            Expr::Num(n) => Ok(*n),
            Expr::Var(name) => env.get(name).copied().ok_or("unbound variable"),
            Expr::Add(a, b) => Ok(self.eval_applicative(a, env)? + self.eval_applicative(b, env)?),
            Expr::EqZero(a) => Ok(i64::from(self.eval_applicative(a, env)? == 0)),
            Expr::If(pred, cons, alt) => {
                if self.eval_applicative(pred, env)? != 0 {
                    self.eval_applicative(cons, env)
                } else {
                    self.eval_applicative(alt, env)
                }
            }
            Expr::Call(name, args) => {
                let proc = self.procs.get(name).cloned().ok_or("unknown procedure")?;
                if proc.params.len() != args.len() {
                    return Err("arity mismatch");
                }
                // Applicative order evaluates all arguments before entering the body.
                let mut local = BTreeMap::new();
                for (param, arg) in proc.params.iter().zip(args) {
                    local.insert(param.clone(), self.eval_applicative(arg, env)?);
                }
                self.eval_applicative(&proc.body, &local)
            }
            Expr::Block { defs, body } => {
                // Block structure (1.1.8): inner definitions extend and shadow outer scope.
                let mut inner = env.clone();
                for (name, rhs) in defs {
                    let v = self.eval_applicative(rhs, &inner)?;
                    inner.insert(name.clone(), v);
                }
                self.eval_applicative(body, &inner)
            }
        }
    }

    fn force(&mut self, thunk: &Thunk) -> Result<i64, &'static str> {
        match thunk {
            Thunk::Value(v) => Ok(*v),
            Thunk::Deferred { expr, env } => self.eval_normal(expr, env),
        }
    }

    fn eval_normal(
        &mut self,
        expr: &Expr,
        env: &BTreeMap<String, Thunk>,
    ) -> Result<i64, &'static str> {
        self.tick()?;
        match expr {
            Expr::Num(n) => Ok(*n),
            Expr::Var(name) => {
                let t = env.get(name).cloned().ok_or("unbound variable")?;
                self.force(&t)
            }
            Expr::Add(a, b) => Ok(self.eval_normal(a, env)? + self.eval_normal(b, env)?),
            Expr::EqZero(a) => Ok(i64::from(self.eval_normal(a, env)? == 0)),
            Expr::If(pred, cons, alt) => {
                if self.eval_normal(pred, env)? != 0 {
                    self.eval_normal(cons, env)
                } else {
                    self.eval_normal(alt, env)
                }
            }
            Expr::Call(name, args) => {
                let proc = self.procs.get(name).cloned().ok_or("unknown procedure")?;
                if proc.params.len() != args.len() {
                    return Err("arity mismatch");
                }
                // Normal order substitutes unevaluated expressions until actually needed.
                let mut local = BTreeMap::new();
                for (param, arg) in proc.params.iter().zip(args) {
                    local.insert(
                        param.clone(),
                        Thunk::Deferred {
                            expr: arg.clone(),
                            env: env.clone(),
                        },
                    );
                }
                self.eval_normal(&proc.body, &local)
            }
            Expr::Block { defs, body } => {
                let mut inner = env.clone();
                for (name, rhs) in defs {
                    let v = self.eval_normal(rhs, &inner)?;
                    inner.insert(name.clone(), Thunk::Value(v));
                }
                self.eval_normal(body, &inner)
            }
        }
    }
}

fn ben_bitdiddle_program(limit: usize) -> (Program, Expr) {
    let mut prog = Program::new(limit);
    prog.procs.insert(
        "p".into(),
        Proc {
            params: vec![],
            body: Expr::Call("p".into(), vec![]),
        },
    );
    prog.procs.insert(
        "test".into(),
        Proc {
            params: vec!["x".into(), "y".into()],
            body: Expr::If(
                Box::new(Expr::EqZero(Box::new(Expr::Var("x".into())))),
                Box::new(Expr::Num(0)),
                Box::new(Expr::Var("y".into())),
            ),
        },
    );
    let call = Expr::Call(
        "test".into(),
        vec![Expr::Num(0), Expr::Call("p".into(), vec![])],
    );
    (prog, call)
}

fn main() {
    let (mut app, expr) = ben_bitdiddle_program(25);
    let app_res = app.eval_applicative(&expr, &BTreeMap::new());
    let (mut norm, expr2) = ben_bitdiddle_program(25);
    let norm_res = norm.eval_normal(&expr2, &BTreeMap::new());
    let mut scoped = Program::new(50);
    let blk = Expr::Block {
        defs: vec![("x".into(), Expr::Num(10))],
        body: Box::new(Expr::Add(
            Box::new(Expr::Block {
                defs: vec![("x".into(), Expr::Num(3))],
                body: Box::new(Expr::Var("x".into())),
            }),
            Box::new(Expr::Var("x".into())),
        )),
    };
    let blk_res = scoped.eval_normal(&blk, &BTreeMap::new());
    println!(
        "applicative: {:?} (steps={}), normal: {:?} (steps={}), block: {:?}",
        app_res, app.steps, norm_res, norm.steps, blk_res
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applicative_diverges_while_normal_terminates() {
        let (mut app, expr) = ben_bitdiddle_program(30);
        assert!(app.eval_applicative(&expr, &BTreeMap::new()).is_err());

        let (mut norm, expr2) = ben_bitdiddle_program(30);
        assert_eq!(norm.eval_normal(&expr2, &BTreeMap::new()), Ok(0));
        assert!(norm.steps < 10);
    }

    #[test]
    fn block_scope_shadows_without_leaking() {
        let mut prog = Program::new(50);
        let expr = Expr::Block {
            defs: vec![("x".into(), Expr::Num(10))],
            body: Box::new(Expr::Add(
                Box::new(Expr::Block {
                    defs: vec![("x".into(), Expr::Num(3))],
                    body: Box::new(Expr::Var("x".into())),
                }),
                Box::new(Expr::Var("x".into())),
            )),
        };
        assert_eq!(prog.eval_applicative(&expr, &BTreeMap::new()), Ok(13));
        assert_eq!(prog.eval_normal(&expr, &BTreeMap::new()), Ok(13));
    }
}
