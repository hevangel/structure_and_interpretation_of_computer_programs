fn accumulate<F, G>(start: i32, end: i32, identity: i64, term: F, combine: G) -> i64
where F: Fn(i32) -> i64, G: Fn(i64, i64) -> i64 {
    let mut result = identity;
    for x in start..=end {
        result = combine(result, term(x));
    }
    result
}
fn main() {
    let sum_of_squares = accumulate(1, 5, 0, |x| i64::from(x * x), |a, b| a + b);
    println!("sum of squares 1..5 = {sum_of_squares}");
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn two_combiners() {
        assert_eq!(accumulate(1, 5, 0, i64::from, |a,b| a+b), 15);
        assert_eq!(accumulate(1, 4, 1, i64::from, |a,b| a*b), 24);
    }
}
