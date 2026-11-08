fn power(mut base: i64, mut exponent: u32) -> i64 {
    let mut result = 1;
    while exponent > 0 {
        // Odd exponent means one factor of base belongs in the answer.
        if exponent % 2 == 1 { result *= base; }
        base *= base;
        exponent /= 2;
    }
    result
}
fn main() { println!("3^13 = {}", power(3, 13)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn powers() {
        assert_eq!(power(3, 13), 1_594_323);
        assert_eq!(power(5, 0), 1);
        assert_eq!(power(-2, 5), -32);
    }
}
