fn horner(coefficients:&[f64],x:f64)->f64 {
    // For [c0,c1,...,cn], fold cn*x^n+...+c0 from the top.
    coefficients.iter().rev().fold(0.0,|acc,&coefficient|acc*x+coefficient)
}
fn main() { println!("p(2) = {}",horner(&[1.0,0.0,3.0,2.0],2.0)); }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polynomial_values() {
        assert_eq!(horner(&[1.0,0.0,3.0,2.0],2.0),29.0);
        assert_eq!(horner(&[7.0],100.0),7.0);
        assert_eq!(horner(&[],3.0),0.0);
    }
}
