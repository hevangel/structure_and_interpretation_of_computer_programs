#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rational { num: i64, den: i64 }
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { (a, b) = (b, a % b); }
    a.abs()
}
impl Rational {
    fn new(num: i64, den: i64) -> Self {
        assert!(den != 0);
        let sign = if den < 0 { -1 } else { 1 };
        let g = gcd(num, den);
        Self { num: sign * num / g, den: sign * den / g }
    }
    fn add(self, other: Self) -> Self {
        Self::new(self.num * other.den + other.num * self.den, self.den * other.den)
    }
}
fn main() { println!("{:?}", Rational::new(1, 2).add(Rational::new(1, 3))); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn canonical_representation() {
        assert_eq!(Rational::new(2, -4), Rational::new(-1, 2));
        assert_eq!(Rational::new(1, 2).add(Rational::new(1, 3)), Rational::new(5, 6));
    }
}
