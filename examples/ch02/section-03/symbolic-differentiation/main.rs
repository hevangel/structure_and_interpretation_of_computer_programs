#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr { Num(i64), X, Add(Box<Expr>,Box<Expr>), Mul(Box<Expr>,Box<Expr>) }
fn add(a: Expr, b: Expr) -> Expr {
    if a == Expr::Num(0) { b } else if b == Expr::Num(0) { a }
    else { Expr::Add(Box::new(a),Box::new(b)) }
}
fn mul(a: Expr, b: Expr) -> Expr {
    if a == Expr::Num(0) || b == Expr::Num(0) { Expr::Num(0) }
    else if a == Expr::Num(1) { b }
    else if b == Expr::Num(1) { a }
    else { Expr::Mul(Box::new(a),Box::new(b)) }
}
fn derivative(e: &Expr) -> Expr {
    match e {
        Expr::Num(_) => Expr::Num(0),
        Expr::X => Expr::Num(1),
        Expr::Add(a,b) => add(derivative(a),derivative(b)),
        Expr::Mul(a,b) => add(mul(derivative(a), (**b).clone()),
                               mul((**a).clone(), derivative(b))),
    }
}
fn eval(e: &Expr, x: i64) -> i64 {
    match e {
        Expr::Num(n) => *n, Expr::X => x,
        Expr::Add(a,b) => eval(a,x)+eval(b,x),
        Expr::Mul(a,b) => eval(a,x)*eval(b,x),
    }
}
fn main() {
    let x_squared=mul(Expr::X,Expr::X);
    println!("d(x*x)/dx at 3 = {}", eval(&derivative(&x_squared),3));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn product_rule() {
        let e=mul(Expr::X,Expr::X);
        assert_eq!(eval(&derivative(&e),7),14);
        assert_eq!(derivative(&Expr::Num(5)),Expr::Num(0));
    }
}
