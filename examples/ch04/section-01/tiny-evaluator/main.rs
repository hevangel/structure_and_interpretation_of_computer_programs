use std::collections::HashMap;
#[derive(Clone)]
enum Expr {
    Int(i64), Var(&'static str),
    Add(Box<Expr>,Box<Expr>),
    Let(&'static str,Box<Expr>,Box<Expr>),
}
fn eval(e:&Expr,env:&HashMap<&'static str,i64>)->Result<i64,String> {
    match e {
        Expr::Int(v)=>Ok(*v),
        Expr::Var(name)=>env.get(name).copied().ok_or_else(||format!("unbound: {name}")),
        Expr::Add(a,b)=>Ok(eval(a,env)?+eval(b,env)?),
        Expr::Let(name,value,body)=>{
            let v=eval(value,env)?;
            let mut inner=env.clone();inner.insert(name,v);
            eval(body,&inner)
        }
    }
}
fn main() {
    let e=Expr::Let("x",Box::new(Expr::Int(4)),
        Box::new(Expr::Add(Box::new(Expr::Var("x")),Box::new(Expr::Int(3)))));
    println!("evaluated: {:?}",eval(&e,&HashMap::new()));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn lexical_binding() {
        let e=Expr::Let("x",Box::new(Expr::Int(2)),Box::new(Expr::Var("x")));
        assert_eq!(eval(&e,&HashMap::new()).unwrap(),2);
        assert!(eval(&Expr::Var("missing"),&HashMap::new()).is_err());
    }
}
