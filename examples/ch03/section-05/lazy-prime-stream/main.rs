struct Primes { candidate:u64,found:Vec<u64> }
impl Primes {
    fn new()->Self {Self{candidate:2,found:Vec::new()}}
}
impl Iterator for Primes {
    type Item=u64;
    fn next(&mut self)->Option<Self::Item> {
        loop {
            let n=self.candidate;
            self.candidate=self.candidate.checked_add(1)?;
            // A composite n has some prime factor <= sqrt(n).
            let prime=self.found.iter().take_while(|&&p|p<=n/p).all(|p|n%p!=0);
            if prime {
                self.found.push(n);
                return Some(n);
            }
        }
    }
}
fn main() {println!("first 10 primes: {:?}",Primes::new().take(10).collect::<Vec<_>>());}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn prefix_matches_known_primes() {
        assert_eq!(Primes::new().take(10).collect::<Vec<_>>(),
            vec![2,3,5,7,11,13,17,19,23,29]);
        assert_eq!(Primes::new().take(0).count(),0);
    }
}
