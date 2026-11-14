fn compose<F,G>(outer:F,inner:G) -> impl Fn(i64)->i64
where F:Fn(i64)->i64,G:Fn(i64)->i64 {
    move |x| outer(inner(x))
}
fn repeated<F>(f:F,times:usize) -> impl Fn(i64)->i64
where F:Fn(i64)->i64 {
    move |mut x| {
        for _ in 0..times { x=f(x); }
        x
    }
}
fn main() {
    let transform = compose(|x|x*x,|x|x+1);
    println!("(5+1)^2 = {}", transform(5));
    println!("add ten = {}", repeated(|x|x+2,5)(0));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn composition_is_ordered() {
        assert_eq!(compose(|x|x*x,|x|x+1)(5),36);
        assert_eq!(compose(|x|x+1,|x|x*x)(5),26);
        assert_eq!(repeated(|x|x*2,0)(3),3);
        assert_eq!(repeated(|x|x*2,5)(3),96);
    }
}
