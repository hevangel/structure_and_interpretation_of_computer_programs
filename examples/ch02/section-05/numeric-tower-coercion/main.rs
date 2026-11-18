#[derive(Clone, Copy, Debug, PartialEq)]
enum TowerNum {
    Int(i64),
    Rat(i64, i64),
    Real(f64),
    Complex { re: f64, im: f64 },
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs().max(1)
}

fn make_rat(n: i64, d: i64) -> TowerNum {
    assert!(d != 0, "zero denominator");
    let sign = if d < 0 { -1 } else { 1 };
    let g = gcd(n, d);
    TowerNum::Rat(sign * n / g, sign * d / g)
}

impl TowerNum {
    fn rank(self) -> u8 {
        match self {
            TowerNum::Int(_) => 0,
            TowerNum::Rat(_, _) => 1,
            TowerNum::Real(_) => 2,
            TowerNum::Complex { .. } => 3,
        }
    }

    fn raise_once(self) -> Option<Self> {
        match self {
            TowerNum::Int(n) => Some(TowerNum::Rat(n, 1)),
            TowerNum::Rat(n, d) => Some(TowerNum::Real(n as f64 / d as f64)),
            TowerNum::Real(x) => Some(TowerNum::Complex { re: x, im: 0.0 }),
            TowerNum::Complex { .. } => None,
        }
    }

    fn raise_to(mut self, target_rank: u8) -> Self {
        while self.rank() < target_rank {
            self = self.raise_once().expect("higher rank exists");
        }
        self
    }

    fn project(self) -> Option<Self> {
        match self {
            TowerNum::Int(_) => None,
            TowerNum::Rat(n, d) => Some(TowerNum::Int(n / d)),
            TowerNum::Real(x) => {
                // Rational approximation via fixed denominator for exact small decimals.
                let den = 1_000_000i64;
                let num = (x * den as f64).round() as i64;
                Some(make_rat(num, den))
            }
            TowerNum::Complex { re, .. } => Some(TowerNum::Real(re)),
        }
    }

    fn equ(self, other: Self) -> bool {
        let top = self.rank().max(other.rank());
        match (self.raise_to(top), other.raise_to(top)) {
            (TowerNum::Int(a), TowerNum::Int(b)) => a == b,
            (TowerNum::Rat(n1, d1), TowerNum::Rat(n2, d2)) => n1 * d2 == n2 * d1,
            (TowerNum::Real(a), TowerNum::Real(b)) => (a - b).abs() < 1e-12,
            (TowerNum::Complex { re: r1, im: i1 }, TowerNum::Complex { re: r2, im: i2 }) => {
                (r1 - r2).abs() < 1e-12 && (i1 - i2).abs() < 1e-12
            }
            _ => unreachable!(),
        }
    }

    fn is_zero(self) -> bool {
        self.equ(TowerNum::Int(0))
    }

    /// Exercise 2.85 canonical simplification: repeatedly project while raise_once preserves value.
    fn drop(mut self) -> Self {
        while let Some(lower) = self.project() {
            if let Some(re_raised) = lower.raise_once() {
                if re_raised.equ(self) {
                    self = lower;
                    continue;
                }
            }
            break;
        }
        self
    }

    fn add(self, other: Self) -> Self {
        let top = self.rank().max(other.rank());
        let raw = match (self.raise_to(top), other.raise_to(top)) {
            (TowerNum::Int(a), TowerNum::Int(b)) => TowerNum::Int(a + b),
            (TowerNum::Rat(n1, d1), TowerNum::Rat(n2, d2)) => make_rat(n1 * d2 + n2 * d1, d1 * d2),
            (TowerNum::Real(a), TowerNum::Real(b)) => TowerNum::Real(a + b),
            (TowerNum::Complex { re: r1, im: i1 }, TowerNum::Complex { re: r2, im: i2 }) => {
                TowerNum::Complex {
                    re: r1 + r2,
                    im: i1 + i2,
                }
            }
            _ => unreachable!(),
        };
        raw.drop()
    }

    fn mul(self, other: Self) -> Self {
        let top = self.rank().max(other.rank());
        let raw = match (self.raise_to(top), other.raise_to(top)) {
            (TowerNum::Int(a), TowerNum::Int(b)) => TowerNum::Int(a * b),
            (TowerNum::Rat(n1, d1), TowerNum::Rat(n2, d2)) => make_rat(n1 * n2, d1 * d2),
            (TowerNum::Real(a), TowerNum::Real(b)) => TowerNum::Real(a * b),
            (TowerNum::Complex { re: r1, im: i1 }, TowerNum::Complex { re: r2, im: i2 }) => {
                TowerNum::Complex {
                    re: r1 * r2 - i1 * i2,
                    im: r1 * i2 + i1 * r2,
                }
            }
            _ => unreachable!(),
        };
        raw.drop()
    }
}

fn main() {
    let a = TowerNum::Int(2);
    let b = TowerNum::Complex { re: 3.0, im: 4.0 };
    let conj = TowerNum::Complex { re: 3.0, im: -4.0 };
    let sum = a.add(b);
    let norm_sq = b.mul(conj);
    println!(
        "2 + (3+4i) = {:?}, (3+4i)(3-4i) dropped = {:?}, zero? {}",
        sum,
        norm_sq,
        TowerNum::Complex { re: 0.0, im: 0.0 }.is_zero()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_step_raise_and_add_across_tower() {
        let res = TowerNum::Int(5).add(TowerNum::Complex { re: 1.5, im: -2.0 });
        assert_eq!(res, TowerNum::Complex { re: 6.5, im: -2.0 });
        assert_eq!(TowerNum::Int(2).add(make_rat(1, 2)), make_rat(5, 2));
    }

    #[test]
    fn drop_simplifies_complex_and_rat_to_int() {
        let z = TowerNum::Complex { re: 6.0, im: 0.0 };
        assert_eq!(z.drop(), TowerNum::Int(6));
        let conj_prod =
            TowerNum::Complex { re: 3.0, im: 4.0 }.mul(TowerNum::Complex { re: 3.0, im: -4.0 });
        assert_eq!(conj_prod, TowerNum::Int(25));
        assert!(make_rat(0, 5).is_zero());
        assert_eq!(make_rat(6, 2).drop(), TowerNum::Int(3));
    }
}
