type Church = Box<dyn Fn(&dyn Fn(u64) -> u64, u64) -> u64>;
fn zero() -> Church { Box::new(|_, x| x) }
fn succ(n: Church) -> Church {
    // The successor applies f once more than its predecessor.
    Box::new(move |f, x| f(n(f, x)))
}
fn as_number(n: &Church) -> u64 { n(&|x| x+1, 0) }
fn main() {
    let two = succ(succ(zero()));
    println!("church two = {}", as_number(&two));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn function_is_number() {
        let three = succ(succ(succ(zero())));
        assert_eq!(as_number(&three), 3);
        assert_eq!(three(&|x| x*2, 1), 8);
    }
}
