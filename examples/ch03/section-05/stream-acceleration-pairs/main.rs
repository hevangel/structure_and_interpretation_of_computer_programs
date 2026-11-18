use std::cell::RefCell;
use std::rc::Rc;

enum Tail<T: Clone> {
    Thunk(Box<dyn FnOnce() -> Stream<T>>),
    Forced(Stream<T>),
}

struct Cell<T: Clone> {
    head: T,
    tail: RefCell<Tail<T>>,
}

#[derive(Clone)]
struct Stream<T: Clone>(Option<Rc<Cell<T>>>);

impl<T: Clone> Drop for Stream<T> {
    fn drop(&mut self) {
        let mut cur = self.0.take();
        while let Some(rc) = cur {
            let Ok(cell) = Rc::try_unwrap(rc) else {
                break;
            };
            if let Tail::Forced(mut next) = cell.tail.into_inner() {
                cur = next.0.take();
            } else {
                cur = None;
            }
        }
    }
}

impl<T: Clone + 'static> Stream<T> {
    fn cons<F: FnOnce() -> Stream<T> + 'static>(head: T, f: F) -> Self {
        Self(Some(Rc::new(Cell {
            head,
            tail: RefCell::new(Tail::Thunk(Box::new(f))),
        })))
    }

    fn cell(&self) -> &Cell<T> {
        self.0.as_ref().expect("non-empty stream")
    }

    fn car(&self) -> T {
        self.cell().head.clone()
    }

    fn cdr(&self) -> Stream<T> {
        if let Tail::Forced(ref s) = *self.cell().tail.borrow() {
            return s.clone();
        }
        let dummy = Tail::Thunk(Box::new(|| unreachable!()));
        let old = std::mem::replace(&mut *self.cell().tail.borrow_mut(), dummy);
        match old {
            Tail::Thunk(f) => {
                let next = f();
                *self.cell().tail.borrow_mut() = Tail::Forced(next.clone());
                next
            }
            Tail::Forced(_) => unreachable!(),
        }
    }

    fn ref_at(&self, n: usize) -> T {
        let mut cur = self.clone();
        for _ in 0..n {
            cur = cur.cdr();
        }
        cur.car()
    }

    fn take(&self, n: usize) -> Vec<T> {
        let mut out = Vec::with_capacity(n);
        let mut cur = self.clone();
        for idx in 0..n {
            out.push(cur.car());
            if idx + 1 < n {
                cur = cur.cdr();
            }
        }
        out
    }

    fn map<U: Clone + 'static, F: Fn(T) -> U + ?Sized + 'static>(&self, f: Rc<F>) -> Stream<U> {
        let s = self.clone();
        Stream::cons(f(s.car()), move || s.cdr().map(f))
    }
}

fn partial_sums(s: Stream<f64>) -> Stream<f64> {
    fn go(s: Stream<f64>, acc: f64) -> Stream<f64> {
        let next = acc + s.car();
        Stream::cons(next, move || go(s.cdr(), next))
    }
    go(s, 0.0)
}

fn pi_summands(n: usize) -> Stream<f64> {
    let sign = if ((n - 1) / 2) % 2 == 0 { 1.0 } else { -1.0 };
    Stream::cons(sign / n as f64, move || pi_summands(n + 2))
}

fn pi_stream() -> Stream<f64> {
    partial_sums(pi_summands(1)).map(Rc::new(|x| x * 4.0))
}

fn sqrt_stream(x: f64) -> Stream<f64> {
    fn go(x: f64, guess: f64) -> Stream<f64> {
        Stream::cons(guess, move || go(x, 0.5 * (guess + x / guess)))
    }
    go(x, 1.0)
}

/// Euler's sequence accelerator for alternating series (3.5.3).
fn euler_transform(s: Stream<f64>) -> Stream<f64> {
    let s0 = s.ref_at(0);
    let s1 = s.ref_at(1);
    let s2 = s.ref_at(2);
    let denom = s0 - 2.0 * s1 + s2;
    let next_val = if denom.abs() < 1e-18 {
        s2
    } else {
        s2 - ((s2 - s1) * (s2 - s1)) / denom
    };
    Stream::cons(next_val, move || euler_transform(s.cdr()))
}

fn make_tableau(transform: fn(Stream<f64>) -> Stream<f64>, s: Stream<f64>) -> Stream<Stream<f64>> {
    let next = transform(s.clone());
    Stream::cons(s, move || make_tableau(transform, next))
}

fn accelerated_sequence(transform: fn(Stream<f64>) -> Stream<f64>, s: Stream<f64>) -> Stream<f64> {
    make_tableau(transform, s).map(Rc::new(|row: Stream<f64>| row.car()))
}

fn integers_from(n: u64) -> Stream<u64> {
    Stream::cons(n, move || integers_from(n + 1))
}

/// Weighted merge of ordered pairs (i <= j) for Ramanujan numbers (Exercise 3.70–3.71).
fn merge_weighted<W>(
    s1: Stream<(u64, u64)>,
    s2: Stream<(u64, u64)>,
    weight: Rc<W>,
) -> Stream<(u64, u64)>
where
    W: Fn((u64, u64)) -> u64 + 'static,
{
    let p1 = s1.car();
    let p2 = s2.car();
    if weight(p1) <= weight(p2) {
        Stream::cons(p1, move || merge_weighted(s1.cdr(), s2, weight))
    } else {
        Stream::cons(p2, move || merge_weighted(s1, s2.cdr(), weight))
    }
}

fn weighted_pairs<W>(s: Stream<u64>, t: Stream<u64>, weight: Rc<W>) -> Stream<(u64, u64)>
where
    W: Fn((u64, u64)) -> u64 + 'static,
{
    let s0 = s.car();
    let t0 = t.car();
    let t_tail = t.cdr();
    let s_tail = s.cdr();
    let w1 = Rc::clone(&weight);
    let w2 = Rc::clone(&weight);
    Stream::cons((s0, t0), move || {
        merge_weighted(
            t_tail.map(Rc::new(move |x| (s0, x))),
            weighted_pairs(s_tail, t_tail_ref(t_tail_clone_helper(t0, t)), w1),
            w2,
        )
    })
}

fn t_tail_clone_helper(_t0: u64, t: Stream<u64>) -> Stream<u64> {
    t.cdr()
}

fn t_tail_ref(s: Stream<u64>) -> Stream<u64> {
    s
}

fn ramanujan_numbers() -> Vec<u64> {
    let cube_sum = Rc::new(|(i, j): (u64, u64)| i * i * i + j * j * j);
    let pairs = weighted_pairs(integers_from(1), integers_from(1), Rc::clone(&cube_sum));
    let mut cur = pairs;
    let mut out = Vec::new();
    let mut prev_w = 0u64;
    for _ in 0..400 {
        let p = cur.car();
        let w = cube_sum(p);
        if w == prev_w && !out.contains(&w) {
            out.push(w);
        }
        prev_w = w;
        cur = cur.cdr();
    }
    out
}

/// Section 3.5.4: delayed-integrand feedback ODE solver `dy/dt = f(y)`.
fn delayed_integral(
    delayed_integrand: Rc<RefCell<Option<Stream<f64>>>>,
    initial: f64,
    dt: f64,
) -> Stream<f64> {
    fn go(s: Stream<f64>, acc: f64, dt: f64) -> Stream<f64> {
        let next = acc + s.car() * dt;
        Stream::cons(next, move || go(s.cdr(), next, dt))
    }
    Stream::cons(initial, move || {
        let integrand = delayed_integrand.borrow().as_ref().unwrap().clone();
        go(integrand, initial, dt)
    })
}

fn solve_ode(f: Rc<dyn Fn(f64) -> f64>, y0: f64, dt: f64) -> Stream<f64> {
    let dy_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let y = delayed_integral(Rc::clone(&dy_cell), y0, dt);
    let dy = y.map(f);
    *dy_cell.borrow_mut() = Some(dy);
    y
}

/// Section 3.5.5: functional stream Monte Carlo using pure PRNG stream.
fn rand_stream(seed: u64) -> Stream<u64> {
    let next = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    Stream::cons(next >> 16, move || rand_stream(next))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn cesaro_pi_stream(seed: u64) -> Stream<f64> {
    fn map_successive_pairs(s: Stream<u64>) -> Stream<bool> {
        let a = s.car().max(1);
        let s1 = s.cdr();
        let b = s1.car().max(1);
        Stream::cons(gcd(a, b) == 1, move || map_successive_pairs(s1.cdr()))
    }
    fn monte_carlo_stream(exp: Stream<bool>, passed: usize, failed: usize) -> Stream<f64> {
        let (p, f) = if exp.car() {
            (passed + 1, failed)
        } else {
            (passed, failed + 1)
        };
        Stream::cons(p as f64 / (p + f) as f64, move || {
            monte_carlo_stream(exp.cdr(), p, f)
        })
    }
    monte_carlo_stream(map_successive_pairs(rand_stream(seed)), 0, 0).map(Rc::new(|p: f64| {
        if p > 0.0 {
            (6.0 / p).sqrt()
        } else {
            0.0
        }
    }))
}

fn main() {
    let pi_accel = accelerated_sequence(euler_transform, pi_stream()).take(8);
    let ram = ramanujan_numbers();
    let e_approx = solve_ode(Rc::new(|y| y), 1.0, 0.001).ref_at(1000);
    let pi_mc = cesaro_pi_stream(42).ref_at(10_000);
    println!(
        "tableau pi[7]={:.12}, sqrt(2)[5]={:.9}",
        pi_accel[7],
        sqrt_stream(2.0).ref_at(5)
    );
    println!(
        "Ramanujan numbers={:?}, ODE e≈{e_approx:.6}, stream MC pi≈{pi_mc:.4}",
        &ram[..2]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tableau_acceleration_converges_to_pi_in_8_terms() {
        let raw_8 = pi_stream().ref_at(7);
        let accel_8 = accelerated_sequence(euler_transform, pi_stream()).ref_at(7);
        assert!((raw_8 - std::f64::consts::PI).abs() > 0.05);
        assert!((accel_8 - std::f64::consts::PI).abs() < 1e-10);
        assert!((sqrt_stream(2.0).ref_at(6) - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn ramanujan_pairs_ode_and_functional_monte_carlo() {
        let ram = ramanujan_numbers();
        assert_eq!(&ram[..2], &[1729, 4104]);

        let e_val = solve_ode(Rc::new(|y| y), 1.0, 0.001).ref_at(1000);
        assert!((e_val - std::f64::consts::E).abs() < 0.002);

        let pi_est = cesaro_pi_stream(777).ref_at(20_000);
        assert!((pi_est - std::f64::consts::PI).abs() < 0.1);
    }
}
