#[derive(Clone, Copy, Debug, PartialEq)]
struct Interval {
    low: f64,
    high: f64,
}

impl Interval {
    fn new(low: f64, high: f64) -> Self {
        assert!(low <= high && low.is_finite() && high.is_finite());
        Self { low, high }
    }

    fn make_center_percent(center: f64, percent_tol: f64) -> Self {
        assert!(percent_tol >= 0.0);
        let width = center.abs() * (percent_tol / 100.0);
        Self::new(center - width, center + width)
    }

    fn center(self) -> f64 {
        0.5 * (self.low + self.high)
    }

    fn width(self) -> f64 {
        0.5 * (self.high - self.low)
    }

    fn add(self, other: Self) -> Self {
        Self::new(self.low + other.low, self.high + other.high)
    }

    fn sub(self, other: Self) -> Self {
        Self::new(self.low - other.high, self.high - other.low)
    }

    fn mul(self, other: Self) -> Self {
        let p = [
            self.low * other.low,
            self.low * other.high,
            self.high * other.low,
            self.high * other.high,
        ];
        let low = p.iter().copied().fold(f64::INFINITY, f64::min);
        let high = p.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self::new(low, high)
    }

    /// 9-case sign analysis (Exercise 2.11) avoiding 4 multiplications unless both span zero.
    fn mul_fast(self, other: Self) -> Self {
        let (a, b) = (self.low, self.high);
        let (c, d) = (other.low, other.high);
        if a >= 0.0 {
            if c >= 0.0 {
                Self::new(a * c, b * d)
            } else if d <= 0.0 {
                Self::new(b * c, a * d)
            } else {
                Self::new(b * c, b * d)
            }
        } else if b <= 0.0 {
            if c >= 0.0 {
                Self::new(a * d, b * c)
            } else if d <= 0.0 {
                Self::new(b * d, a * c)
            } else {
                Self::new(a * d, a * c)
            }
        } else if c >= 0.0 {
            Self::new(a * d, b * d)
        } else if d <= 0.0 {
            Self::new(b * c, a * c)
        } else {
            Self::new((a * d).min(b * c), (a * c).max(b * d))
        }
    }

    fn reciprocal(self) -> Result<Self, &'static str> {
        if self.low <= 0.0 && self.high >= 0.0 {
            return Err("cannot divide by an interval spanning zero");
        }
        Ok(Self::new(1.0 / self.high, 1.0 / self.low))
    }

    fn div(self, other: Self) -> Result<Self, &'static str> {
        Ok(self.mul(other.reciprocal()?))
    }
}

/// Parallel resistor formula R1*R2 / (R1 + R2) repeats variables and inflates error bounds.
fn par1(r1: Interval, r2: Interval) -> Result<Interval, &'static str> {
    r1.mul(r2).div(r1.add(r2))
}

/// Equivalent 1 / (1/R1 + 1/R2) uses each uncertain resistor once -> tighter bounds.
fn par2(r1: Interval, r2: Interval) -> Result<Interval, &'static str> {
    let one = Interval::new(1.0, 1.0);
    let sum = one.div(r1)?.add(one.div(r2)?);
    one.div(sum)
}

fn main() {
    let r1 = Interval::make_center_percent(100.0, 5.0);
    let r2 = Interval::make_center_percent(200.0, 2.0);
    let p1 = par1(r1, r2).unwrap();
    let p2 = par2(r1, r2).unwrap();
    let diff = r1.sub(r1);
    println!(
        "par1 width={:.4}, par2 width={:.4}, r1-r1 center={:.2} width={:.2}, mul_fast={:?}",
        p1.width(),
        p2.width(),
        diff.center(),
        diff.width(),
        r1.mul_fast(r2),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_case_mul_matches_exhaustive_mul() {
        let samples = [
            Interval::new(2.0, 4.0),
            Interval::new(-5.0, -1.0),
            Interval::new(-3.0, 6.0),
            Interval::new(0.0, 3.0),
            Interval::new(-4.0, 0.0),
        ];
        for &a in &samples {
            for &b in &samples {
                assert_eq!(a.mul_fast(b), a.mul(b));
            }
        }
    }

    #[test]
    fn par2_produces_tighter_bounds_than_par1() {
        let r1 = Interval::make_center_percent(100.0, 5.0);
        let r2 = Interval::make_center_percent(200.0, 5.0);
        let p1 = par1(r1, r2).unwrap();
        let p2 = par2(r1, r2).unwrap();
        assert!(p2.width() < p1.width());
        assert!(p1.low <= p2.low && p2.high <= p1.high);
    }

    #[test]
    fn zero_straddling_division_is_rejected() {
        let num = Interval::new(1.0, 2.0);
        let bad = Interval::new(-1.0, 1.0);
        assert!(num.div(bad).is_err());
        assert_eq!(num.sub(Interval::new(0.5, 1.0)), Interval::new(0.0, 1.5));
    }
}
