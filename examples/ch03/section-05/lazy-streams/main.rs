fn prime(n:u64)->bool {
    if n<2{return false;}
    (2..).take_while(|d| *d<=n / d).all(|d| n%d!=0)
}
fn first_primes(count:usize)->Vec<u64> {
    // A consumer decides how many elements of this unbounded iterator exist.
    std::iter::successors(Some(2u64),|n|n.checked_add(1))
        .filter(|&n|prime(n)).take(count).collect()
}
fn main() { println!("first ten primes: {:?}",first_primes(10)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn finite_prefix() {
        assert_eq!(first_primes(7),vec![2,3,5,7,11,13,17]);
        assert!(first_primes(0).is_empty());
    }
}
