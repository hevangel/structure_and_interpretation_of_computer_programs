use std::rc::Rc;

type RealFn = Rc<dyn Fn(f64) -> f64>;

fn fixed_point(f: &RealFn, mut guess: f64, tol: f64) -> f64 {
    for _ in 0..500 {
        let next = f(guess);
        if (next - guess).abs() < tol {
            return next;
        }
        guess = next;
    }
    guess
}

fn average_damp(f: RealFn) -> RealFn {
    Rc::new(move |x| 0.5 * (x + f(x)))
}

fn deriv(g: RealFn, dx: f64) -> RealFn {
    Rc::new(move |x| (g(x + dx) - g(x)) / dx)
}

fn newton_transform(g: RealFn) -> RealFn {
    let dg = deriv(Rc::clone(&g), 1e-6);
    Rc::new(move |x| x - g(x) / dg(x))
}

fn newtons_method(g: RealFn, guess: f64) -> f64 {
    fixed_point(&newton_transform(g), guess, 1e-11)
}

fn compose(f: RealFn, g: RealFn) -> RealFn {
    Rc::new(move |x| f(g(x)))
}

fn repeated(f: RealFn, n: usize) -> RealFn {
    let mut acc: RealFn = Rc::new(|x| x);
    for _ in 0..n {
        acc = compose(Rc::clone(&f), acc);
    }
    acc
}

fn smooth(f: RealFn, dx: f64) -> RealFn {
    Rc::new(move |x| (f(x - dx) + f(x) + f(x + dx)) / 3.0)
}

fn nth_root(x: f64, n: u32) -> f64 {
    assert!(x >= 0.0 && n >= 1);
    let damps = (usize::BITS - (n as usize).leading_zeros()).saturating_sub(1) as usize;
    let mut damped: RealFn = Rc::new(move |y| x / y.powi((n - 1) as i32));
    for _ in 0..damps.max(1) {
        damped = average_damp(damped);
    }
    fixed_point(&damped, 1.0, 1e-11)
}

fn iterative_improve(good_enough: Rc<dyn Fn(f64) -> bool>, improve: RealFn) -> RealFn {
    Rc::new(move |mut guess| {
        for _ in 0..500 {
            if good_enough(guess) {
                return guess;
            }
            guess = improve(guess);
        }
        guess
    })
}

fn main() {
    let cubic_zero = newtons_method(Rc::new(|x| x * x * x + x * x + x - 3.0), 1.0);
    let seventh = nth_root(128.0, 7);
    let smoothed = repeated(Rc::new(|x| x * x), 1);
    let impulse_smooth = smooth(Rc::new(|x| if x.abs() < 1e-6 { 3.0 } else { 0.0 }), 0.1);
    let sqrt_imp = iterative_improve(
        Rc::new(|g| (g * g - 9.0).abs() < 1e-10),
        Rc::new(|g| 0.5 * (g + 9.0 / g)),
    );
    println!(
        "cubic root={cubic_zero:.9}, 128^(1/7)={seventh:.9}, smooth(0)={:.3}, repeat(sq,1)(3)={}, sqrt(9)={:.9}",
        impulse_smooth(0.0),
        smoothed(3.0),
        sqrt_imp(1.0),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtons_method_solves_cubic() {
        // Exercise 1.40: x^3 + ax^2 + bx + c = 0
        let root = newtons_method(Rc::new(|x| x * x * x - 2.0 * x * x + x - 2.0), 2.5);
        assert!((root - 2.0).abs() < 1e-9);
    }

    #[test]
    fn nth_root_converges_for_higher_powers() {
        let root = nth_root(128.0, 7);
        assert!((root - 2.0).abs() < 1e-9);
        assert!((nth_root(81.0, 4) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn smooth_repeated_and_iterative_improve() {
        let spike: RealFn = Rc::new(|x| if x.abs() < 1e-9 { 6.0 } else { 0.0 });
        let once = smooth(Rc::clone(&spike), 0.1);
        assert!((once(0.0) - 2.0).abs() < 1e-12);
        let inc_twice = repeated(Rc::new(|x| x + 1.0), 3);
        assert_eq!(inc_twice(4.0), 7.0);

        let sqrt_fn = iterative_improve(
            Rc::new(|g| (g * g - 25.0).abs() < 1e-11),
            Rc::new(|g| 0.5 * (g + 25.0 / g)),
        );
        assert!((sqrt_fn(1.0) - 5.0).abs() < 1e-9);
    }
}
