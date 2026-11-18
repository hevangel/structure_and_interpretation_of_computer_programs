use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::thread;

/// Exercise 3.38: enumerate all distinct balance values achievable if three concurrent
/// transactions (Peter +10, Paul -20, Mary /= 2) interleave read/write steps arbitrarily.
fn enumerate_unserialized_ex3_38(initial: i64) -> BTreeSet<i64> {
    #[derive(Clone, Copy)]
    enum Step {
        PeterRead,
        PeterWrite,
        PaulRead,
        PaulWrite,
        MaryRead,
        MaryWrite,
    }

    #[derive(Clone, Copy, Default)]
    struct TraceState {
        p: usize,
        q: usize,
        m: usize,
        p_val: i64,
        q_val: i64,
        m_val: i64,
    }

    fn dfs(bal: i64, st: TraceState, out: &mut BTreeSet<i64>) {
        if st.p == 2 && st.q == 2 && st.m == 2 {
            out.insert(bal);
            return;
        }
        let mut choices = Vec::new();
        if st.p == 0 {
            choices.push(Step::PeterRead);
        } else if st.p == 1 {
            choices.push(Step::PeterWrite);
        }
        if st.q == 0 {
            choices.push(Step::PaulRead);
        } else if st.q == 1 {
            choices.push(Step::PaulWrite);
        }
        if st.m == 0 {
            choices.push(Step::MaryRead);
        } else if st.m == 1 {
            choices.push(Step::MaryWrite);
        }
        for step in choices {
            match step {
                Step::PeterRead => dfs(
                    bal,
                    TraceState {
                        p: 1,
                        p_val: bal,
                        ..st
                    },
                    out,
                ),
                Step::PeterWrite => dfs(st.p_val + 10, TraceState { p: 2, ..st }, out),
                Step::PaulRead => dfs(
                    bal,
                    TraceState {
                        q: 1,
                        q_val: bal,
                        ..st
                    },
                    out,
                ),
                Step::PaulWrite => dfs(st.q_val - 20, TraceState { q: 2, ..st }, out),
                Step::MaryRead => dfs(
                    bal,
                    TraceState {
                        m: 1,
                        m_val: bal,
                        ..st
                    },
                    out,
                ),
                Step::MaryWrite => dfs(st.m_val - st.m_val / 2, TraceState { m: 2, ..st }, out),
            }
        }
    }

    let mut out = BTreeSet::new();
    dfs(initial, TraceState::default(), &mut out);
    out
}

/// SICP `make-serializer` abstraction + unique account ID for Exercise 3.48 deadlock avoidance.
struct SerializedAccount {
    id: u64,
    state: Mutex<i64>,
}

impl SerializedAccount {
    fn new(id: u64, balance: i64) -> Arc<Self> {
        Arc::new(Self {
            id,
            state: Mutex::new(balance),
        })
    }

    fn balance(&self) -> i64 {
        *self.state.lock().expect("unpoisoned mutex")
    }
}

/// Exercise 3.48: always acquire the lower-numbered account's serializer first to break
/// circular wait and eliminate deadlocks under opposing concurrent exchanges.
fn serialized_exchange(a: &SerializedAccount, b: &SerializedAccount) {
    if a.id == b.id {
        return;
    }
    let (first, second) = if a.id < b.id { (a, b) } else { (b, a) };
    let mut g1 = first.state.lock().expect("unpoisoned first lock");
    let mut g2 = second.state.lock().expect("unpoisoned second lock");
    let diff = *g1 - *g2;
    *g1 -= diff;
    *g2 += diff;
}

fn run_opposing_exchanges(rounds: usize) -> (i64, i64) {
    let acc_a = SerializedAccount::new(1, 100);
    let acc_b = SerializedAccount::new(2, 250);
    let a1 = Arc::clone(&acc_a);
    let b1 = Arc::clone(&acc_b);
    let a2 = Arc::clone(&acc_a);
    let b2 = Arc::clone(&acc_b);

    let t1 = thread::spawn(move || {
        for _ in 0..rounds {
            serialized_exchange(&a1, &b1);
        }
    });
    let t2 = thread::spawn(move || {
        for _ in 0..rounds {
            serialized_exchange(&b2, &a2);
        }
    });
    t1.join().expect("worker 1 finished");
    t2.join().expect("worker 2 finished");
    (acc_a.balance(), acc_b.balance())
}

fn main() {
    let outcomes = enumerate_unserialized_ex3_38(100);
    let (a, b) = run_opposing_exchanges(500);
    println!("Ex 3.38 unserialized outcomes from 100: {:?}", outcomes);
    println!(
        "deadlock-free opposing exchanges final balances: ({a}, {b}), sum={}",
        a + b
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unserialized_interleavings_produce_anomalous_balances() {
        let outcomes = enumerate_unserialized_ex3_38(100);
        // Sequential orders give 35, 40, 45, 50; read/write races add 30, 55, 60, 80, 90, 110.
        assert_eq!(
            outcomes,
            BTreeSet::from([30, 35, 40, 45, 50, 55, 60, 80, 90, 110])
        );
    }

    #[test]
    fn ordered_serialized_exchange_prevents_deadlock_and_conserves_total() {
        let (a, b) = run_opposing_exchanges(2_000);
        assert_eq!(a + b, 350);
        assert!((a == 100 && b == 250) || (a == 250 && b == 100));
    }
}
