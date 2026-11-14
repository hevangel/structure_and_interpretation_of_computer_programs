enum Expr {
    Number(i64),Divide(Box<Expr>,Box<Expr>),
    IfZero(Box<Expr>,Box<Expr>,Box<Expr>),
}
fn eval(expr:&Expr)->Result<i64,&'static str> {
    match expr {
        Expr::Number(n)=>Ok(*n),
        Expr::Divide(a,b)=>{
            let lhs=eval(a)?;
            let rhs=eval(b)?;
            lhs.checked_div(rhs).ok_or("division by zero or overflow")
        }
        Expr::IfZero(condition,then_branch,else_branch)=>{
            // Never evaluate the unselected branch.
            if eval(condition)?==0 {eval(then_branch)} else {eval(else_branch)}
        }
    }
}
fn main() {
    let e=Expr::IfZero(
        Box::new(Expr::Number(0)),
        Box::new(Expr::Number(42)),
        Box::new(Expr::Divide(Box::new(Expr::Number(1)),Box::new(Expr::Number(0)))));
    println!("lazy branch result: {:?}",eval(&e));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn only_selected_branch_is_evaluated() {
        let bad=Expr::Divide(Box::new(Expr::Number(1)),Box::new(Expr::Number(0)));
        let safe=Expr::IfZero(Box::new(Expr::Number(0)),
            Box::new(Expr::Number(42)),Box::new(bad));
        assert_eq!(eval(&safe),Ok(42));
        assert_eq!(eval(&Expr::Divide(Box::new(Expr::Number(7)),
            Box::new(Expr::Number(0)))),Err("division by zero or overflow"));
    }
}
