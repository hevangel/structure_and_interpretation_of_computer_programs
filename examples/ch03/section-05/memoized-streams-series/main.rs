use std::cell::{Cell, RefCell};
use std::rc::Rc;

enum StreamTail<T: Clone> {
    Empty,
    Thunk(Box<dyn FnOnce() -> Stream<T>>),
    Forced(Stream<T>),
}

struct StreamCell<T: Clone> {
    head: T,
    tail: RefCell<StreamTail<T>>,
}

/// Shared memoized stream: forcing the tail caches the computed node so recursive
/// streams (`fibs`, `sieve`, `power_series`) evaluate each element at most once.
#[derive(Clone)]
struct Stream<T: Clone>(Option<Rc<StreamCell<T>>>);

impl<T: Clone + 'static> Stream<T> {
    fn empty() -> Self {
        Self(None)
    }

    fn cons<F>(head: T, tail_thunk: F) -> Self
    where
        F: FnOnce() -> Stream<T> + 'static,
    {
        Self(Some(Rc::new(StreamCell {
            head,
            tail: RefCell::new(StreamTail::Thunk(Box::new(tail_thunk))),
        })))
    }

    fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn car(&self) -> T {
        self.0.as_ref().expect("stream is empty").head.clone()
    }

    fn cdr(&self) -> Stream<T> {
        let cell = self.0.as_ref().expect("stream is empty");
        match &*cell.tail.borrow() {
            StreamTail::Empty => return Stream::empty(),
            StreamTail::Forced(s) => return s.clone(),
            StreamTail::Thunk(_) => {}
        }
        let old = std::mem::replace(&mut *cell.tail.borrow_mut(), StreamTail::Empty);
        if let StreamTail::Thunk(thunk) = old {
            let next = thunk();
            *cell.tail.borrow_mut() = StreamTail::Forced(next.clone());
            next
        } else {
            unreachable!()
        }
    }

    fn take(&self, n: usize) -> Vec<T> {
        let mut out = Vec::with_capacity(n);
        let mut cur = self.clone();
        for idx in 0..n {
            if cur.is_empty() {
                break;
            }
            out.push(cur.car());
            if idx + 1 < n {
                cur = cur.cdr();
            }
        }
        out
    }

    fn map<U: Clone + 'static, F>(&self, f: Rc<F>) -> Stream<U>
    where
        F: Fn(T) -> U + ?Sized + 'static,
    {
        if self.is_empty() {
            return Stream::empty();
        }
        let s = self.clone();
        Stream::cons(f(s.car()), move || s.cdr().map(f))
    }

    fn filter<P>(&self, pred: Rc<P>) -> Stream<T>
    where
        P: Fn(&T) -> bool + ?Sized + 'static,
    {
        let mut cur = self.clone();
        while !cur.is_empty() {
            let h = cur.car();
            if pred(&h) {
                let tail = cur.cdr();
                return Stream::cons(h, move || tail.filter(pred));
            }
            cur = cur.cdr();
        }
        Stream::empty()
    }
}

fn zip_with<T: Clone + 'static, F>(a: Stream<T>, b: Stream<T>, f: Rc<F>) -> Stream<T>
where
    F: Fn(T, T) -> T + ?Sized + 'static,
{
    if a.is_empty() || b.is_empty() {
        return Stream::empty();
    }
    let head = f(a.car(), b.car());
    Stream::cons(head, move || zip_with(a.cdr(), b.cdr(), f))
}

fn add_streams(a: Stream<f64>, b: Stream<f64>) -> Stream<f64> {
    zip_with(a, b, Rc::new(|x, y| x + y))
}

fn scale_stream(s: Stream<f64>, factor: f64) -> Stream<f64> {
    s.map(Rc::new(move |x| x * factor))
}

fn integers_from(n: u64) -> Stream<u64> {
    Stream::cons(n, move || integers_from(n + 1))
}

fn sieve(s: Stream<u64>) -> Stream<u64> {
    let p = s.car();
    Stream::cons(p, move || {
        let pred: Rc<dyn Fn(&u64) -> bool> = Rc::new(move |x: &u64| *x % p != 0);
        sieve(s.cdr().filter(pred))
    })
}

/// Exercise 3.56: Hamming numbers (2^i * 3^j * 5^k) via ordered merge without duplicates.
fn merge_streams(s1: Stream<u64>, s2: Stream<u64>) -> Stream<u64> {
    if s1.is_empty() {
        return s2;
    }
    if s2.is_empty() {
        return s1;
    }
    let x = s1.car();
    let y = s2.car();
    if x < y {
        Stream::cons(x, move || merge_streams(s1.cdr(), s2))
    } else if x > y {
        Stream::cons(y, move || merge_streams(s1, s2.cdr()))
    } else {
        Stream::cons(x, move || merge_streams(s1.cdr(), s2.cdr()))
    }
}

fn scale_u64(s: Stream<u64>, factor: u64) -> Stream<u64> {
    s.map(Rc::new(move |x| x * factor))
}

fn hamming_stream() -> Stream<u64> {
    // Recursive lazy tie-the-knot via an explicit shared cell holding the stream head.
    let cell: Rc<RefCell<Option<Stream<u64>>>> = Rc::new(RefCell::new(None));
    let cell_ref = Rc::clone(&cell);
    let s = Stream::cons(1, move || {
        let h = cell_ref.borrow().as_ref().unwrap().clone();
        merge_streams(
            scale_u64(h.clone(), 2),
            merge_streams(scale_u64(h.clone(), 3), scale_u64(h, 5)),
        )
    });
    *cell.borrow_mut() = Some(s.clone());
    s
}

/// Power series combinators (3.5.2, Exercises 3.59–3.62)
fn integrate_series(s: Stream<f64>) -> Stream<f64> {
    fn go(s: Stream<f64>, n: usize) -> Stream<f64> {
        if s.is_empty() {
            return Stream::empty();
        }
        Stream::cons(s.car() / n as f64, move || go(s.cdr(), n + 1))
    }
    go(s, 1)
}

fn mul_series(s1: Stream<f64>, s2: Stream<f64>) -> Stream<f64> {
    let a0 = s1.car();
    let b0 = s2.car();
    let s1_tail = s1.cdr();
    let s2_tail = s2.cdr();
    Stream::cons(a0 * b0, move || {
        add_streams(scale_stream(s2_tail, a0), mul_series(s1_tail, s2))
    })
}

fn invert_unit_series(s: Stream<f64>) -> Stream<f64> {
    assert!((s.car() - 1.0).abs() < 1e-12);
    let cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let cell_ref = Rc::clone(&cell);
    let sr = s.cdr();
    let inv = Stream::cons(1.0, move || {
        let self_s = cell_ref.borrow().as_ref().unwrap().clone();
        scale_stream(mul_series(sr, self_s), -1.0)
    });
    *cell.borrow_mut() = Some(inv.clone());
    inv
}

fn div_series(num: Stream<f64>, den: Stream<f64>) -> Stream<f64> {
    let d0 = den.car();
    assert!(d0.abs() > 1e-12, "zero constant term in denominator");
    let inv = invert_unit_series(scale_stream(den, 1.0 / d0));
    scale_stream(mul_series(num, inv), 1.0 / d0)
}

fn exp_series() -> Stream<f64> {
    let cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let cell_ref = Rc::clone(&cell);
    let s = Stream::cons(1.0, move || {
        integrate_series(cell_ref.borrow().as_ref().unwrap().clone())
    });
    *cell.borrow_mut() = Some(s.clone());
    s
}

fn sine_and_cosine_series() -> (Stream<f64>, Stream<f64>) {
    let sin_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let cos_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let sin_ref = Rc::clone(&sin_cell);
    let cos_ref = Rc::clone(&cos_cell);

    let cos_s = Stream::cons(1.0, move || {
        scale_stream(
            integrate_series(sin_ref.borrow().as_ref().unwrap().clone()),
            -1.0,
        )
    });
    let sin_s = Stream::cons(0.0, move || {
        integrate_series(cos_ref.borrow().as_ref().unwrap().clone())
    });
    *cos_cell.borrow_mut() = Some(cos_s.clone());
    *sin_cell.borrow_mut() = Some(sin_s.clone());
    (sin_s, cos_s)
}

fn counted_stream(counter: Rc<Cell<usize>>, n: usize) -> Stream<usize> {
    Stream::cons(n, move || {
        counter.set(counter.get() + 1);
        counted_stream(counter, n + 1)
    })
}

fn main() {
    let primes = sieve(integers_from(2)).take(10);
    let hamming = hamming_stream().take(12);
    let (sin_s, cos_s) = sine_and_cosine_series();
    let tan_s = div_series(sin_s.clone(), cos_s.clone());
    let _exp = exp_series().take(6);
    let forces = Rc::new(Cell::new(0));
    let counted = counted_stream(Rc::clone(&forces), 0);
    let _ = counted.take(6);
    let _ = counted.take(6);
    println!("primes[..10] = {:?}", primes);
    println!("hamming[..12] = {:?}", hamming);
    println!(
        "tan(x) coeffs[..6] = {:?}; forces(take 6 x2) = {}",
        tan_s.take(6),
        forces.get()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memoized_stream_forces_each_tail_at_most_once() {
        let forces = Rc::new(Cell::new(0));
        let s = counted_stream(Rc::clone(&forces), 0);
        assert_eq!(s.take(6), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(forces.get(), 5);
        // Re-traversing the same prefix performs zero extra thunk evaluations.
        assert_eq!(s.take(6), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(forces.get(), 5);
    }

    #[test]
    fn sieve_and_hamming_streams() {
        assert_eq!(
            sieve(integers_from(2)).take(10),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
        assert_eq!(
            hamming_stream().take(12),
            vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16]
        );
    }

    #[test]
    fn sin_squared_plus_cos_squared_equals_one_and_exp_tan() {
        let (sin_s, cos_s) = sine_and_cosine_series();
        let identity = add_streams(
            mul_series(sin_s.clone(), sin_s.clone()),
            mul_series(cos_s.clone(), cos_s.clone()),
        )
        .take(13);
        assert!((identity[0] - 1.0).abs() < 1e-12);
        for (deg, &coeff) in identity.iter().enumerate().skip(1) {
            assert!(coeff.abs() < 1e-12, "non-zero degree {deg}: {coeff}");
        }

        let tan_coeffs = div_series(sin_s, cos_s).take(6);
        let expected = [0.0, 1.0, 0.0, 1.0 / 3.0, 0.0, 2.0 / 15.0];
        for (a, b) in tan_coeffs.iter().zip(expected) {
            assert!((a - b).abs() < 1e-12);
        }

        let exp_c = exp_series().take(5);
        for (k, &c) in exp_c.iter().enumerate() {
            let fact = (1..=k.max(1)).product::<usize>() as f64;
            let expected_c = if k == 0 { 1.0 } else { 1.0 / fact };
            assert!((c - expected_c).abs() < 1e-12);
        }
    }
}
