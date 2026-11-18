#[derive(Clone, Copy, Debug)]
enum RandMsg {
    Generate,
    Reset(u64),
}

struct RandGen {
    state: u64,
}

impl RandGen {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn dispatch(&mut self, msg: RandMsg) -> u64 {
        match msg {
            RandMsg::Generate => {
                // Deterministic 64-bit LCG (Knuth MMIX constants).
                self.state = self
                    .state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                self.state >> 16
            }
            RandMsg::Reset(seed) => {
                self.state = seed;
                seed
            }
        }
    }

    fn unit_f64(&mut self) -> f64 {
        (self.dispatch(RandMsg::Generate) & ((1u64 << 48) - 1)) as f64 / (1u64 << 48) as f64
    }

    fn random_in_range(&mut self, low: f64, high: f64) -> f64 {
        low + self.unit_f64() * (high - low)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn monte_carlo<F: FnMut() -> bool>(trials: usize, mut experiment: F) -> f64 {
    assert!(trials > 0);
    let passed = (0..trials).filter(|_| experiment()).count();
    passed as f64 / trials as f64
}

fn estimate_pi_cesaro(rng: &mut RandGen, trials: usize) -> f64 {
    let p = monte_carlo(trials, || {
        let a = rng.dispatch(RandMsg::Generate).max(1);
        let b = rng.dispatch(RandMsg::Generate).max(1);
        gcd(a, b) == 1
    });
    (6.0 / p).sqrt()
}

fn estimate_integral<P>(
    rng: &mut RandGen,
    pred: P,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    trials: usize,
) -> f64
where
    P: Fn(f64, f64) -> bool,
{
    let fraction = monte_carlo(trials, || {
        let x = rng.random_in_range(x1, x2);
        let y = rng.random_in_range(y1, y2);
        pred(x, y)
    });
    fraction * (x2 - x1) * (y2 - y1)
}

fn main() {
    let mut rng = RandGen::new(42);
    let pi_cesaro = estimate_pi_cesaro(&mut rng, 40_000);
    rng.dispatch(RandMsg::Reset(42));
    let pi_disk = estimate_integral(
        &mut rng,
        |x, y| x * x + y * y <= 1.0,
        -1.0,
        1.0,
        -1.0,
        1.0,
        40_000,
    );
    println!("Cesaro pi ≈ {pi_cesaro:.4}, Unit-disk pi ≈ {pi_disk:.4}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_reproduces_identical_sequence_and_estimates_pi() {
        let mut rng = RandGen::new(12345);
        let first: Vec<u64> = (0..5).map(|_| rng.dispatch(RandMsg::Generate)).collect();
        rng.dispatch(RandMsg::Reset(12345));
        let second: Vec<u64> = (0..5).map(|_| rng.dispatch(RandMsg::Generate)).collect();
        assert_eq!(first, second);

        let cesaro = estimate_pi_cesaro(&mut rng, 60_000);
        assert!((cesaro - std::f64::consts::PI).abs() < 0.1);

        let area = estimate_integral(
            &mut rng,
            |x, y| x * x + y * y <= 1.0,
            -1.0,
            1.0,
            -1.0,
            1.0,
            60_000,
        );
        assert!((area - std::f64::consts::PI).abs() < 0.05);
    }
}
