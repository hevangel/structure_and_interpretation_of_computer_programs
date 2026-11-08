use std::collections::HashMap;
#[derive(Clone)]
enum Expr {
    Int(i64),Var(&'static str),Add(Box<Expr>,Box<Expr>),
    Let(&'static str,Box<Expr>,Box<Expr>),
    Lambda(&'static str,Box<Expr>),Call(Box<Expr>,Box<Expr>),
}
#[derive(Clone)]
enum Value {
    Num(i64),
    Closure {param:&'static str,body:Box<Expr>,env:HashMap<&'static str,Value>},
}
fn number(value:Value)->Result<i64,String> {
    match value {Value::Num(n)=>Ok(n),Value::Closure{..}=>Err("expected number".into())}
}
fn evaluate(e:&Expr,env:&HashMap<&'static str,Value>)->Result<Value,String> {
    match e {
        Expr::Int(n)=>Ok(Value::Num(*n)),
        Expr::Var(name)=>env.get(name).cloned().ok_or_else(||format!("unbound: {name}")),
        Expr::Add(a,b)=>Ok(Value::Num(number(evaluate(a,env)?)?+number(evaluate(b,env)?)?)),
        Expr::Let(name,value,body)=>{
            let v=evaluate(value,env)?;
            let mut inner=env.clone();inner.insert(name,v);
            evaluate(body,&inner)
        }
        Expr::Lambda(param,body)=>Ok(Value::Closure{
            param,body:body.clone(),env:env.clone(),
        }),
        Expr::Call(function,argument)=>{
            let f=evaluate(function,env)?;
            let arg=evaluate(argument,env)?;
            match f {
                Value::Num(_)=>Err("cannot call a number".into()),
                Value::Closure{param,body,mut env}=>{
                    // Extend the definition environment, not the call-site environment.
                    env.insert(param,arg);
                    evaluate(&body,&env)
                }
            }
        }
    }
}
fn example()->Expr {
    // let x=10; let f=(lambda y: x+y); let x=100; f(3)
    Expr::Let("x",Box::new(Expr::Int(10)),
        Box::new(Expr::Let("f",
            Box::new(Expr::Lambda("y",
                Box::new(Expr::Add(Box::new(Expr::Var("x")),Box::new(Expr::Var("y")))))),
            Box::new(Expr::Let("x",Box::new(Expr::Int(100)),
                Box::new(Expr::Call(Box::new(Expr::Var("f")),Box::new(Expr::Int(3)))))))))
}
fn main() {
    println!("captured lexical x gives {:?}",number(evaluate(&example(),&HashMap::new()).unwrap()));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn uses_definition_environment() {
        let result=evaluate(&example(),&HashMap::new()).unwrap();
        assert_eq!(number(result).unwrap(),13);
    }
}
