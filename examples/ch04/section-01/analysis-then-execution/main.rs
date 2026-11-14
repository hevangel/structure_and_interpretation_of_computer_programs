use std::collections::HashMap;
#[derive(Clone)]
enum Expr {
    Number(i64),Variable(&'static str),
    Add(Box<Expr>,Box<Expr>),
}
type Executor = Box<dyn Fn(&HashMap<&'static str,i64>)->Result<i64,String>>;
fn analyze(expr:&Expr)->Executor {
    match expr {
        Expr::Number(n)=>{
            let value=*n;
            Box::new(move |_|Ok(value))
        }
        Expr::Variable(name)=>{
            let name=*name;
            Box::new(move |env|env.get(name).copied().ok_or_else(||format!("unbound: {name}")))
        }
        Expr::Add(left,right)=>{
            let a=analyze(left);
            let b=analyze(right);
            // Reuse pre-analyzed child computations for each environment.
            Box::new(move |env|Ok(a(env)?+b(env)?))
        }
    }
}
fn main() {
    let expr=Expr::Add(Box::new(Expr::Variable("x")),Box::new(Expr::Number(3)));
    let program=analyze(&expr);
    println!("x=7 => {:?}",program(&HashMap::from([("x",7)])));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn analysis_is_reusable() {
        let expr=Expr::Add(Box::new(Expr::Variable("x")),Box::new(Expr::Number(3)));
        let program=analyze(&expr);
        assert_eq!(program(&HashMap::from([("x",7)])).unwrap(),10);
        assert_eq!(program(&HashMap::from([("x",20)])).unwrap(),23);
        assert!(program(&HashMap::new()).is_err());
    }
}
