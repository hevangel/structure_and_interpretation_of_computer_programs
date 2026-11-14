fn multiply(mut a: u64, mut b: u64) -> u64 {
    let mut result = 0;
    while b != 0 {
        if b & 1 == 1 { result += a; }
        b >>= 1;
        // Skip an unnecessary last doubling that could overflow.
        if b != 0 { a *= 2; }
    }
    result
}
fn main() { println!("37 × 42 = {}", multiply(37, 42)); }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_doubling() {
        for (a,b) in [(0,100),(7,0),(37,42),(1,100000),(1000,500)] {
            assert_eq!(multiply(a,b), a*b);
        }
        assert_eq!(multiply(1, 1u64 << 63),1u64 << 63);
    }
}
