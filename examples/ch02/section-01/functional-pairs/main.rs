fn pair(first:i64,second:i64)->impl Fn(bool)->i64 {
    // The selector is the *only* way to observe the captured pair.
    move |select_first| if select_first {first} else {second}
}
fn first<P:Fn(bool)->i64>(p:&P)->i64 {p(true)}
fn second<P:Fn(bool)->i64>(p:&P)->i64 {p(false)}
fn main() {
    let p=pair(4,9);
    println!("first={}, second={}",first(&p),second(&p));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selector_laws() {
        let p=pair(17,-5);
        assert_eq!(first(&p),17);
        assert_eq!(second(&p),-5);
    }
}
