use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Poly {
    var: String,
    /// Map from degree -> non-zero integer coefficient.
    terms: BTreeMap<usize, i64>,
}

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs().max(1)
}

impl Poly {
    fn from_terms(var: &str, items: &[(usize, i64)]) -> Self {
        let mut terms = BTreeMap::new();
        for &(deg, coeff) in items {
            *terms.entry(deg).or_insert(0) += coeff;
        }
        terms.retain(|_, c| *c != 0);
        Self {
            var: var.to_string(),
            terms,
        }
    }

    fn zero(var: &str) -> Self {
        Self {
            var: var.to_string(),
            terms: BTreeMap::new(),
        }
    }

    fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    fn order(&self) -> usize {
        self.terms.keys().next_back().copied().unwrap_or(0)
    }

    fn leading_coeff(&self) -> i64 {
        self.terms.values().next_back().copied().unwrap_or(0)
    }

    fn negate(&self) -> Self {
        Self {
            var: self.var.clone(),
            terms: self.terms.iter().map(|(&d, &c)| (d, -c)).collect(),
        }
    }

    fn scale(&self, factor: i64) -> Self {
        if factor == 0 {
            return Self::zero(&self.var);
        }
        Self {
            var: self.var.clone(),
            terms: self.terms.iter().map(|(&d, &c)| (d, c * factor)).collect(),
        }
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.var, other.var, "polynomial variable mismatch");
        let mut terms = self.terms.clone();
        for (&d, &c) in &other.terms {
            *terms.entry(d).or_insert(0) += c;
        }
        terms.retain(|_, c| *c != 0);
        Self {
            var: self.var.clone(),
            terms,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&other.negate())
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.var, other.var, "polynomial variable mismatch");
        let mut terms = BTreeMap::new();
        for (&d1, &c1) in &self.terms {
            for (&d2, &c2) in &other.terms {
                *terms.entry(d1 + d2).or_insert(0) += c1 * c2;
            }
        }
        terms.retain(|_, c| *c != 0);
        Self {
            var: self.var.clone(),
            terms,
        }
    }

    /// Exact polynomial division (Exercise 2.91) returning (quotient, remainder) over Z when divisible.
    fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        assert_eq!(self.var, divisor.var);
        assert!(!divisor.is_zero(), "division by zero polynomial");
        let mut q = Self::zero(&self.var);
        let mut r = self.clone();
        let d_ord = divisor.order();
        let d_lc = divisor.leading_coeff();
        while !r.is_zero() && r.order() >= d_ord {
            let r_lc = r.leading_coeff();
            if r_lc % d_lc != 0 {
                break;
            }
            let diff_deg = r.order() - d_ord;
            let coeff = r_lc / d_lc;
            let term = Self::from_terms(&self.var, &[(diff_deg, coeff)]);
            q = q.add(&term);
            r = r.sub(&term.mul(divisor));
        }
        (q, r)
    }

    /// Exercise 2.96: integer-preserving pseudoremainder avoids fractional coefficients.
    fn pseudoremainder(&self, divisor: &Self) -> Self {
        assert_eq!(self.var, divisor.var);
        assert!(!divisor.is_zero());
        let o1 = self.order();
        let o2 = divisor.order();
        if o1 < o2 {
            return self.clone();
        }
        let c = divisor.leading_coeff();
        let factor = c.pow((1 + o1 - o2) as u32);
        let (_, rem) = self.scale(factor).div_rem(divisor);
        rem
    }

    fn content_gcd(&self) -> Self {
        if self.is_zero() {
            return self.clone();
        }
        let g = self.terms.values().fold(0i64, |acc, &c| gcd_i64(acc, c));
        let sign = if self.leading_coeff() < 0 { -1 } else { 1 };
        let divisor = g * sign;
        Self {
            var: self.var.clone(),
            terms: self.terms.iter().map(|(&d, &c)| (d, c / divisor)).collect(),
        }
    }

    fn gcd(&self, other: &Self) -> Self {
        assert_eq!(self.var, other.var);
        let mut a = self.clone();
        let mut b = other.clone();
        while !b.is_zero() {
            let r = a.pseudoremainder(&b);
            a = b;
            b = r;
        }
        a.content_gcd()
    }
}

/// Exercise 2.97: reduce a polynomial rational function N(x) / D(x) by their GCD.
fn reduce_poly_fraction(num: &Poly, den: &Poly) -> (Poly, Poly) {
    assert!(!den.is_zero());
    let g = num.gcd(den);
    let factor = g
        .leading_coeff()
        .pow((1 + num.order().max(den.order()) - g.order()) as u32);
    let (mut n_red, rem_n) = num.scale(factor).div_rem(&g);
    let (mut d_red, rem_d) = den.scale(factor).div_rem(&g);
    assert!(rem_n.is_zero() && rem_d.is_zero());
    let common = gcd_i64(
        n_red.terms.values().fold(0, |acc, &c| gcd_i64(acc, c)),
        d_red.terms.values().fold(0, |acc, &c| gcd_i64(acc, c)),
    );
    let sign = if d_red.leading_coeff() < 0 { -1 } else { 1 };
    let div = common * sign;
    n_red.terms.values_mut().for_each(|c| *c /= div);
    d_red.terms.values_mut().for_each(|c| *c /= div);
    (n_red, d_red)
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let mut first = true;
        for (&deg, &coeff) in self.terms.iter().rev() {
            if !first {
                write!(f, " + ")?;
            }
            first = false;
            match deg {
                0 => write!(f, "{coeff}")?,
                1 => write!(f, "{coeff}*{}", self.var)?,
                d => write!(f, "{coeff}*{}^{d}", self.var)?,
            }
        }
        Ok(())
    }
}

fn main() {
    let p1 = Poly::from_terms("x", &[(2, 1), (0, -1)]); // x^2 - 1
    let p2 = Poly::from_terms("x", &[(1, 1), (0, -1)]); // x - 1
    let p3 = Poly::from_terms("x", &[(1, 1), (0, 2)]); // x + 2
    let (q, r) = p1.div_rem(&p2);
    let (rn, rd) = reduce_poly_fraction(&p1.mul(&p3), &p2.mul(&p3));
    println!("(x^2 - 1)/(x - 1) = ({q}, rem={r}); reduced fraction = ({rn}) / ({rd})");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polynomial_division_and_gcd() {
        let x2_minus_1 = Poly::from_terms("x", &[(2, 1), (0, -1)]);
        let x_minus_1 = Poly::from_terms("x", &[(1, 1), (0, -1)]);
        let x_plus_1 = Poly::from_terms("x", &[(1, 1), (0, 1)]);
        let (q, r) = x2_minus_1.div_rem(&x_minus_1);
        assert_eq!(q, x_plus_1);
        assert!(r.is_zero());
    }

    #[test]
    fn pseudoremainder_gcd_reduces_rational_functions() {
        // Exercise 2.95 / 2.97 non-monic GCD and fraction reduction:
        // p1 = x + 1, p2 = x^2 - 1, p3 = 3x - 2 => gcd(p1*p3, p2*p3) = (x+1)(3x-2)
        let p1 = Poly::from_terms("x", &[(1, 1), (0, 1)]);
        let p2 = Poly::from_terms("x", &[(1, 1), (0, -1)]);
        let q = Poly::from_terms("x", &[(1, 3), (0, -2)]);
        let num = p1.mul(&q);
        let den = p2.mul(&q);
        let (rn, rd) = reduce_poly_fraction(&num, &den);
        assert_eq!(rn, p1);
        assert_eq!(rd, p2);
    }
}
