#[derive(Clone, Copy, Debug)]
struct Interval { low: f64, high: f64 }
impl Interval {
    fn new(low: f64, high: f64) -> Self {
        assert!(low <= high && low.is_finite() && high.is_finite());
        Self { low, high }
    }
    fn mul(self, b: Self) -> Self {
        let p = [self.low*b.low, self.low*b.high, self.high*b.low, self.high*b.high];
        let low = p.iter().copied().fold(f64::INFINITY, f64::min);
        let high = p.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self::new(low, high)
    }
    fn reciprocal(self) -> Self {
        assert!(self.high < 0.0 || self.low > 0.0, "interval spans zero");
        Self::new(1.0/self.high, 1.0/self.low)
    }
}
fn main() { println!("{:?}", Interval::new(2.0, 3.0).mul(Interval::new(4.0, 5.0))); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn bounds() {
        let r = Interval::new(-2.0, 3.0).mul(Interval::new(4.0, 5.0));
        assert_eq!((r.low, r.high), (-10.0, 15.0));
        let inv = Interval::new(2.0, 4.0).reciprocal();
        assert_eq!((inv.low, inv.high), (0.25, 0.5));
    }
}
