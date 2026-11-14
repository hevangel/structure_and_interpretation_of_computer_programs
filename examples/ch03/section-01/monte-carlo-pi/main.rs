struct Lcg { state:u64 }
impl Lcg {
    fn next(&mut self)->f64 {
        self.state=self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        // Use the high 53 bits to build a floating point value in [0,1).
        (self.state>>11) as f64 / (1u64<<53) as f64
    }
}
fn estimate_pi(seed:u64,trials:usize)->f64 {
    assert!(trials>0);
    let mut rng=Lcg{state:seed};
    let mut inside=0usize;
    for _ in 0..trials {
        let x=rng.next();let y=rng.next();
        if x*x+y*y<=1.0 {inside+=1;}
    }
    4.0*inside as f64/trials as f64
}
fn main() {println!("Monte Carlo π ≈ {:.5}",estimate_pi(42,100_000));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn seeded_experiment_is_reproducible() {
        let a=estimate_pi(42,100_000);
        let b=estimate_pi(42,100_000);
        assert_eq!(a,b);
        assert!((a-std::f64::consts::PI).abs()<0.04);
    }
}
