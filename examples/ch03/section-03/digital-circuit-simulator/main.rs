use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

type Action = Rc<dyn Fn(&mut Agenda)>;

#[derive(Default)]
struct Agenda {
    current_time: u64,
    segments: BTreeMap<u64, VecDeque<Action>>,
}

impl Agenda {
    fn after_delay(&mut self, delay: u64, action: Action) {
        let time = self.current_time + delay;
        self.segments.entry(time).or_default().push_back(action);
    }

    fn propagate(&mut self) {
        while let Some((&time, _)) = self.segments.iter().next() {
            self.current_time = time;
            let action = {
                let q = self.segments.get_mut(&time).expect("time bucket exists");
                let a = q.pop_front().expect("non-empty bucket");
                if q.is_empty() {
                    self.segments.remove(&time);
                }
                a
            };
            action(self);
        }
    }
}

struct WireState {
    signal: bool,
    actions: Vec<Action>,
}

#[derive(Clone)]
struct Wire(Rc<RefCell<WireState>>);

impl Wire {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(WireState {
            signal: false,
            actions: Vec::new(),
        })))
    }

    fn get(&self) -> bool {
        self.0.borrow().signal
    }

    fn set(&self, new_val: bool, agenda: &mut Agenda) {
        let actions = {
            let mut st = self.0.borrow_mut();
            if st.signal == new_val {
                return;
            }
            st.signal = new_val;
            st.actions.clone()
        };
        for act in actions {
            act(agenda);
        }
    }

    fn add_action(&self, action: Action, agenda: &mut Agenda) {
        self.0.borrow_mut().actions.push(Rc::clone(&action));
        // SICP 3.3.4: newly attached procedures run immediately to initialize outputs.
        action(agenda);
    }
}

const INVERTER_DELAY: u64 = 2;
const AND_DELAY: u64 = 3;
const OR_DELAY: u64 = 5;

fn inverter(input: &Wire, output: &Wire, agenda: &mut Agenda) {
    let inp = input.clone();
    let out = output.clone();
    input.add_action(
        Rc::new(move |ag| {
            let new_sig = !inp.get();
            let out_w = out.clone();
            ag.after_delay(INVERTER_DELAY, Rc::new(move |ag2| out_w.set(new_sig, ag2)));
        }),
        agenda,
    );
}

fn and_gate(a1: &Wire, a2: &Wire, output: &Wire, agenda: &mut Agenda) {
    let (w1, w2, out) = (a1.clone(), a2.clone(), output.clone());
    let action: Action = Rc::new(move |ag| {
        let new_sig = w1.get() && w2.get();
        let out_w = out.clone();
        ag.after_delay(AND_DELAY, Rc::new(move |ag2| out_w.set(new_sig, ag2)));
    });
    a1.add_action(Rc::clone(&action), agenda);
    a2.add_action(action, agenda);
}

fn or_gate(o1: &Wire, o2: &Wire, output: &Wire, agenda: &mut Agenda) {
    let (w1, w2, out) = (o1.clone(), o2.clone(), output.clone());
    let action: Action = Rc::new(move |ag| {
        let new_sig = w1.get() || w2.get();
        let out_w = out.clone();
        ag.after_delay(OR_DELAY, Rc::new(move |ag2| out_w.set(new_sig, ag2)));
    });
    o1.add_action(Rc::clone(&action), agenda);
    o2.add_action(action, agenda);
}

fn half_adder(a: &Wire, b: &Wire, s: &Wire, c: &Wire, agenda: &mut Agenda) {
    let d = Wire::new();
    let e = Wire::new();
    or_gate(a, b, &d, agenda);
    and_gate(a, b, c, agenda);
    inverter(c, &e, agenda);
    and_gate(&d, &e, s, agenda);
}

fn full_adder(a: &Wire, b: &Wire, c_in: &Wire, sum: &Wire, c_out: &Wire, agenda: &mut Agenda) {
    let s = Wire::new();
    let c1 = Wire::new();
    let c2 = Wire::new();
    half_adder(b, c_in, &s, &c1, agenda);
    half_adder(a, &s, sum, &c2, agenda);
    or_gate(&c1, &c2, c_out, agenda);
}

/// Exercise 3.30: n-bit ripple-carry adder (LSB-first slices).
fn ripple_carry_adder(
    a_bits: &[Wire],
    b_bits: &[Wire],
    s_bits: &[Wire],
    c_out: &Wire,
    agenda: &mut Agenda,
) {
    assert_eq!(a_bits.len(), b_bits.len());
    assert_eq!(a_bits.len(), s_bits.len());
    let mut carry_in = Wire::new();
    for i in 0..a_bits.len() {
        let next_carry = if i + 1 == a_bits.len() {
            c_out.clone()
        } else {
            Wire::new()
        };
        full_adder(
            &a_bits[i],
            &b_bits[i],
            &carry_in,
            &s_bits[i],
            &next_carry,
            agenda,
        );
        carry_in = next_carry;
    }
}

fn add_4bit(a: u8, b: u8) -> (u8, bool, u64) {
    let mut agenda = Agenda::default();
    let a_bits: Vec<_> = (0..4).map(|_| Wire::new()).collect();
    let b_bits: Vec<_> = (0..4).map(|_| Wire::new()).collect();
    let s_bits: Vec<_> = (0..4).map(|_| Wire::new()).collect();
    let c_out = Wire::new();
    ripple_carry_adder(&a_bits, &b_bits, &s_bits, &c_out, &mut agenda);
    agenda.propagate();

    let t0 = agenda.current_time;
    for i in 0..4 {
        a_bits[i].set((a >> i) & 1 == 1, &mut agenda);
        b_bits[i].set((b >> i) & 1 == 1, &mut agenda);
    }
    agenda.propagate();

    let mut sum = 0u8;
    for (i, w) in s_bits.iter().enumerate() {
        if w.get() {
            sum |= 1 << i;
        }
    }
    (sum, c_out.get(), agenda.current_time - t0)
}

fn main() {
    let (sum, carry, elapsed) = add_4bit(11, 7);
    println!("4-bit ripple-carry 11 + 7 = {sum} (carry={carry}), settled in {elapsed} ticks");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_adder_critical_path_timing() {
        let mut agenda = Agenda::default();
        let (a, b, s, c) = (Wire::new(), Wire::new(), Wire::new(), Wire::new());
        half_adder(&a, &b, &s, &c, &mut agenda);
        agenda.propagate();
        assert!(!s.get() && !c.get());

        // Setting a=true triggers OR(5) + AND(3) = 8 ticks to sum=true.
        let t_start = agenda.current_time;
        a.set(true, &mut agenda);
        agenda.propagate();
        assert!(s.get() && !c.get());
        assert_eq!(agenda.current_time - t_start, 8);
    }

    #[test]
    fn four_bit_ripple_carry_adder_exhaustive() {
        for a in 0u8..16 {
            for b in 0u8..16 {
                let (sum, carry, _elapsed) = add_4bit(a, b);
                let total = u8::from(carry) * 16 + sum;
                assert_eq!(total, a + b, "{a} + {b}");
            }
        }
    }
}
