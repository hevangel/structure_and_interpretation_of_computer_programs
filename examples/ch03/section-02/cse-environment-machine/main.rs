use std::collections::BTreeMap;

type EnvId = usize;

#[derive(Clone, Debug)]
struct Frame {
    bindings: BTreeMap<String, Value>,
    parent: Option<EnvId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Stmt {
    Num(i64),
    Var(String),
    Add(Box<Stmt>, Box<Stmt>),
    Assign(String, Box<Stmt>),
    Declare(String, Box<Stmt>),
    Lambda {
        params: Vec<String>,
        body: Box<Stmt>,
    },
    Call(Box<Stmt>, Vec<Stmt>),
    Seq(Vec<Stmt>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Int(i64),
    Closure {
        params: Vec<String>,
        body: Stmt,
        env: EnvId,
    },
}

#[derive(Clone, Debug)]
enum ControlItem {
    Eval(Stmt),
    AddOp,
    AssignOp(String),
    DeclareOp(String),
    CallOp(usize),
    RestoreEnv(EnvId),
    PopDiscard,
}

#[derive(Clone, Debug)]
struct TraceStep {
    control_len: usize,
    stash_len: usize,
    env_id: EnvId,
    action: String,
}

struct CseMachine {
    frames: Vec<Frame>,
    control: Vec<ControlItem>,
    stash: Vec<Value>,
    env: EnvId,
    trace: Vec<TraceStep>,
}

impl CseMachine {
    fn new() -> Self {
        Self {
            frames: vec![Frame {
                bindings: BTreeMap::new(),
                parent: None,
            }],
            control: Vec::new(),
            stash: Vec::new(),
            env: 0,
            trace: Vec::new(),
        }
    }

    fn extend_env(&mut self, parent: EnvId, pairs: Vec<(String, Value)>) -> EnvId {
        let mut bindings = BTreeMap::new();
        for (k, v) in pairs {
            bindings.insert(k, v);
        }
        let id = self.frames.len();
        self.frames.push(Frame {
            bindings,
            parent: Some(parent),
        });
        id
    }

    fn lookup(&self, mut env: EnvId, name: &str) -> Result<Value, String> {
        loop {
            if let Some(val) = self.frames[env].bindings.get(name) {
                return Ok(val.clone());
            }
            match self.frames[env].parent {
                Some(parent) => env = parent,
                None => return Err(format!("unbound name: {name}")),
            }
        }
    }

    fn assign(&mut self, mut env: EnvId, name: &str, val: Value) -> Result<(), String> {
        loop {
            if self.frames[env].bindings.contains_key(name) {
                self.frames[env].bindings.insert(name.to_string(), val);
                return Ok(());
            }
            match self.frames[env].parent {
                Some(parent) => env = parent,
                None => return Err(format!("cannot assign unbound name: {name}")),
            }
        }
    }

    fn step(&mut self) -> Result<bool, String> {
        let Some(item) = self.control.pop() else {
            return Ok(false);
        };
        let action = match item {
            ControlItem::Eval(stmt) => match stmt {
                Stmt::Num(n) => {
                    self.stash.push(Value::Int(n));
                    format!("push {n}")
                }
                Stmt::Var(name) => {
                    let v = self.lookup(self.env, &name)?;
                    self.stash.push(v);
                    format!("lookup {name}")
                }
                Stmt::Add(a, b) => {
                    self.control.push(ControlItem::AddOp);
                    self.control.push(ControlItem::Eval(*b));
                    self.control.push(ControlItem::Eval(*a));
                    "expand add".to_string()
                }
                Stmt::Assign(name, rhs) => {
                    self.control.push(ControlItem::AssignOp(name.clone()));
                    self.control.push(ControlItem::Eval(*rhs));
                    format!("schedule assign {name}")
                }
                Stmt::Declare(name, rhs) => {
                    self.control.push(ControlItem::DeclareOp(name.clone()));
                    self.control.push(ControlItem::Eval(*rhs));
                    format!("schedule declare {name}")
                }
                Stmt::Lambda { params, body } => {
                    self.stash.push(Value::Closure {
                        params,
                        body: *body,
                        env: self.env,
                    });
                    format!("make closure in env#{}", self.env)
                }
                Stmt::Call(callee, args) => {
                    let argc = args.len();
                    self.control.push(ControlItem::CallOp(argc));
                    for arg in args.into_iter().rev() {
                        self.control.push(ControlItem::Eval(arg));
                    }
                    self.control.push(ControlItem::Eval(*callee));
                    format!("schedule call ({argc} args)")
                }
                Stmt::Seq(stmts) => {
                    let len = stmts.len();
                    for (idx, s) in stmts.into_iter().enumerate().rev() {
                        if idx + 1 < len {
                            self.control.push(ControlItem::PopDiscard);
                        }
                        self.control.push(ControlItem::Eval(s));
                    }
                    "expand sequence".to_string()
                }
            },
            ControlItem::AddOp => {
                let Value::Int(b) = self.stash.pop().ok_or("stash underflow")? else {
                    return Err("expected int".into());
                };
                let Value::Int(a) = self.stash.pop().ok_or("stash underflow")? else {
                    return Err("expected int".into());
                };
                self.stash.push(Value::Int(a + b));
                "apply +".to_string()
            }
            ControlItem::AssignOp(name) => {
                let val = self.stash.last().cloned().ok_or("stash underflow")?;
                self.assign(self.env, &name, val)?;
                format!("assign {name}")
            }
            ControlItem::DeclareOp(name) => {
                let val = self.stash.last().cloned().ok_or("stash underflow")?;
                self.frames[self.env].bindings.insert(name.clone(), val);
                format!("declare {name} in env#{}", self.env)
            }
            ControlItem::CallOp(argc) => {
                let mut args = Vec::with_capacity(argc);
                for _ in 0..argc {
                    args.push(self.stash.pop().ok_or("stash underflow")?);
                }
                args.reverse();
                let callee = self.stash.pop().ok_or("stash underflow")?;
                match callee {
                    Value::Closure { params, body, env } => {
                        if params.len() != args.len() {
                            return Err("arity mismatch".into());
                        }
                        let saved_env = self.env;
                        let pairs = params.into_iter().zip(args).collect();
                        let call_env = self.extend_env(env, pairs);
                        self.control.push(ControlItem::RestoreEnv(saved_env));
                        self.control.push(ControlItem::Eval(body));
                        self.env = call_env;
                        format!("enter closure env#{call_env} (parent env#{env})")
                    }
                    Value::Int(_) => return Err("cannot call int".into()),
                }
            }
            ControlItem::RestoreEnv(prev) => {
                self.env = prev;
                format!("restore env#{prev}")
            }
            ControlItem::PopDiscard => {
                self.stash.pop();
                "pop discarded statement value".to_string()
            }
        };
        self.trace.push(TraceStep {
            control_len: self.control.len(),
            stash_len: self.stash.len(),
            env_id: self.env,
            action,
        });
        Ok(true)
    }

    fn eval(&mut self, stmt: Stmt) -> Result<Value, String> {
        self.control.push(ControlItem::Eval(stmt));
        while self.step()? {}
        self.stash.pop().ok_or_else(|| "empty stash".to_string())
    }
}

fn make_withdraw_program() -> Stmt {
    Stmt::Seq(vec![
        Stmt::Declare(
            "make_withdraw".into(),
            Box::new(Stmt::Lambda {
                params: vec!["balance".into()],
                body: Box::new(Stmt::Lambda {
                    params: vec!["amount".into()],
                    body: Box::new(Stmt::Seq(vec![
                        Stmt::Assign(
                            "balance".into(),
                            Box::new(Stmt::Add(
                                Box::new(Stmt::Var("balance".into())),
                                Box::new(Stmt::Var("amount".into())),
                            )),
                        ),
                        Stmt::Var("balance".into()),
                    ])),
                }),
            }),
        ),
        Stmt::Declare(
            "w1".into(),
            Box::new(Stmt::Call(
                Box::new(Stmt::Var("make_withdraw".into())),
                vec![Stmt::Num(100)],
            )),
        ),
        Stmt::Declare(
            "w2".into(),
            Box::new(Stmt::Call(
                Box::new(Stmt::Var("make_withdraw".into())),
                vec![Stmt::Num(100)],
            )),
        ),
        Stmt::Call(Box::new(Stmt::Var("w1".into())), vec![Stmt::Num(-25)]),
        Stmt::Call(Box::new(Stmt::Var("w1".into())), vec![Stmt::Num(-10)]),
        Stmt::Add(
            Box::new(Stmt::Call(
                Box::new(Stmt::Var("w2".into())),
                vec![Stmt::Num(-5)],
            )),
            Box::new(Stmt::Call(
                Box::new(Stmt::Var("w1".into())),
                vec![Stmt::Num(0)],
            )),
        ),
    ])
}

fn main() {
    let mut machine = CseMachine::new();
    let result = machine.eval(make_withdraw_program()).unwrap();
    println!(
        "CSE result={:?}, frames={}, steps={}",
        result,
        machine.frames.len(),
        machine.trace.len()
    );
    for step in machine.trace.iter().take(6) {
        println!(
            "  C={} S={} E=env#{}: {}",
            step.control_len, step.stash_len, step.env_id, step.action
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_closures_mutate_their_own_frames() {
        let mut machine = CseMachine::new();
        // w1: 100 - 25 - 10 = 65; w2: 100 - 5 = 95; w2(-5) + w1(0) = 160
        assert_eq!(machine.eval(make_withdraw_program()), Ok(Value::Int(160)));
        assert!(machine.frames.len() >= 5);
        assert!(!machine.trace.is_empty());
    }

    #[test]
    fn sibling_closures_share_outer_mutable_frame() {
        let mut machine = CseMachine::new();
        let prog = Stmt::Seq(vec![
            Stmt::Declare("shared".into(), Box::new(Stmt::Num(10))),
            Stmt::Declare(
                "inc".into(),
                Box::new(Stmt::Lambda {
                    params: vec![],
                    body: Box::new(Stmt::Assign(
                        "shared".into(),
                        Box::new(Stmt::Add(
                            Box::new(Stmt::Var("shared".into())),
                            Box::new(Stmt::Num(1)),
                        )),
                    )),
                }),
            ),
            Stmt::Declare(
                "read".into(),
                Box::new(Stmt::Lambda {
                    params: vec![],
                    body: Box::new(Stmt::Var("shared".into())),
                }),
            ),
            Stmt::Call(Box::new(Stmt::Var("inc".into())), vec![]),
            Stmt::Call(Box::new(Stmt::Var("inc".into())), vec![]),
            Stmt::Call(Box::new(Stmt::Var("read".into())), vec![]),
        ]);
        assert_eq!(machine.eval(prog), Ok(Value::Int(12)));
    }
}
