use std::collections::BTreeMap;
use std::rc::Rc;

type EqFn<K> = Rc<dyn Fn(&K, &K) -> bool>;

struct Table<K, V> {
    value: Option<V>,
    entries: Vec<(K, Table<K, V>)>,
    same_key: EqFn<K>,
}

impl<K: Clone, V> Table<K, V> {
    fn new(same_key: EqFn<K>) -> Self {
        Self {
            value: None,
            entries: Vec::new(),
            same_key,
        }
    }

    fn assoc_mut(&mut self, key: &K) -> Option<&mut Table<K, V>> {
        let same = &self.same_key;
        self.entries
            .iter_mut()
            .find(|(k, _)| same(k, key))
            .map(|(_, sub)| sub)
    }

    fn assoc(&self, key: &K) -> Option<&Table<K, V>> {
        self.entries
            .iter()
            .find(|(k, _)| (self.same_key)(k, key))
            .map(|(_, sub)| sub)
    }

    fn insert(&mut self, keys: &[K], value: V) {
        match keys {
            [] => self.value = Some(value),
            [first, rest @ ..] => {
                let same = Rc::clone(&self.same_key);
                if let Some(sub) = self.assoc_mut(first) {
                    sub.insert(rest, value);
                } else {
                    let mut child = Table::new(same);
                    child.insert(rest, value);
                    self.entries.push((first.clone(), child));
                }
            }
        }
    }

    fn lookup(&self, keys: &[K]) -> Option<&V> {
        match keys {
            [] => self.value.as_ref(),
            [first, rest @ ..] => self.assoc(first)?.lookup(rest),
        }
    }
}

/// Exercise 3.27: memoized recursion with explicit invocation counter.
fn fib_unmemoized(n: u64, calls: &mut usize) -> u64 {
    *calls += 1;
    if n <= 1 {
        n
    } else {
        fib_unmemoized(n - 1, calls) + fib_unmemoized(n - 2, calls)
    }
}

fn memoize_u64<F>(f: F) -> impl FnMut(u64, &mut usize) -> u64
where
    F: Fn(&mut dyn FnMut(u64, &mut usize) -> u64, u64, &mut usize) -> u64,
{
    let mut cache: BTreeMap<u64, u64> = BTreeMap::new();
    struct Holder<'a, F> {
        cache: &'a mut BTreeMap<u64, u64>,
        f: &'a F,
    }
    impl<'a, F> Holder<'a, F>
    where
        F: Fn(&mut dyn FnMut(u64, &mut usize) -> u64, u64, &mut usize) -> u64,
    {
        fn call(&mut self, x: u64, calls: &mut usize) -> u64 {
            if let Some(&v) = self.cache.get(&x) {
                return v;
            }
            *calls += 1;
            let f_ref = self.f;
            let mut rec = |arg: u64, c: &mut usize| self.call(arg, c);
            let res = f_ref(&mut rec, x, calls);
            self.cache.insert(x, res);
            res
        }
    }
    move |x, calls| {
        let mut h = Holder {
            cache: &mut cache,
            f: &f,
        };
        h.call(x, calls)
    }
}

fn main() {
    // 2D local table with custom tolerance equality (Exercise 3.24)
    let approx_eq: EqFn<f64> = Rc::new(|a, b| (a - b).abs() < 0.05);
    let mut grid = Table::new(approx_eq);
    grid.insert(&[1.0, 2.0], "point-A");
    grid.insert(&[3.0, 4.0], "point-B");
    println!("lookup [1.02, 1.98] = {:?}", grid.lookup(&[1.02, 1.98]));

    let mut slow_calls = 0;
    let slow = fib_unmemoized(20, &mut slow_calls);
    let mut memo_calls = 0;
    let mut memo_fib = memoize_u64(|rec, n, c| {
        if n <= 1 {
            n
        } else {
            rec(n - 1, c) + rec(n - 2, c)
        }
    });
    let fast = memo_fib(20, &mut memo_calls);
    println!(
        "fib(20)={fast} (slow={slow}): unmemoized calls={slow_calls}, memoized calls={memo_calls}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_dimensional_table_and_tolerance_key_matching() {
        let tol: EqFn<f64> = Rc::new(|a, b| (a - b).abs() < 0.01);
        let mut tbl = Table::new(tol);
        tbl.insert(&[1.0, 2.0, 3.0], 42);
        tbl.insert(&[1.0, 2.5, 3.0], 99);
        assert_eq!(tbl.lookup(&[1.005, 1.995, 3.001]), Some(&42));
        assert_eq!(tbl.lookup(&[1.0, 2.5, 3.0]), Some(&99));
        assert_eq!(tbl.lookup(&[1.0, 2.2, 3.0]), None);
    }

    #[test]
    fn memoized_fibonacci_evaluates_each_subproblem_once() {
        let mut slow_calls = 0;
        let _ = fib_unmemoized(18, &mut slow_calls);

        let mut memo_calls = 0;
        let mut memo_fib = memoize_u64(|rec, n, c| {
            if n <= 1 {
                n
            } else {
                rec(n - 1, c) + rec(n - 2, c)
            }
        });
        assert_eq!(memo_fib(30, &mut memo_calls), 832_040);
        assert_eq!(memo_calls, 31);
        assert!(slow_calls > 5_000);
        // Second query hits cache with zero extra evaluations.
        assert_eq!(memo_fib(30, &mut memo_calls), 832_040);
        assert_eq!(memo_calls, 31);
    }
}
