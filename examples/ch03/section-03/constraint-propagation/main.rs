use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Informant {
    User,
    Constraint(usize),
}

trait Constraint {
    fn id(&self) -> usize;
    fn process_new_value(&self);
    fn process_forget_value(&self);
}

struct ConnectorState {
    value: Option<f64>,
    informant: Option<Informant>,
    constraints: Vec<Rc<dyn Constraint>>,
}

#[derive(Clone)]
struct Connector(Rc<RefCell<ConnectorState>>);

impl Connector {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(ConnectorState {
            value: None,
            informant: None,
            constraints: Vec::new(),
        })))
    }

    fn get_value(&self) -> Option<f64> {
        self.0.borrow().value
    }

    fn has_value(&self) -> bool {
        self.0.borrow().value.is_some()
    }

    fn connect(&self, c: Rc<dyn Constraint>) {
        let has_val = {
            let mut st = self.0.borrow_mut();
            st.constraints.push(Rc::clone(&c));
            st.value.is_some()
        };
        if has_val {
            c.process_new_value();
        }
    }

    fn set_value(&self, new_val: f64, setter: Informant) -> Result<(), String> {
        let notify = {
            let mut st = self.0.borrow_mut();
            match st.value {
                None => {
                    st.value = Some(new_val);
                    st.informant = Some(setter);
                    st.constraints.clone()
                }
                Some(old) => {
                    if (old - new_val).abs() > 1e-12 {
                        return Err(format!("contradiction: {old} vs {new_val}"));
                    }
                    return Ok(());
                }
            }
        };
        for c in notify {
            if !matches!(setter, Informant::Constraint(id) if id == c.id()) {
                c.process_new_value();
            }
        }
        Ok(())
    }

    /// Only the original informant can retract a value; independent user constraints remain intact.
    fn forget_value(&self, retractor: Informant) {
        let notify = {
            let mut st = self.0.borrow_mut();
            if st.informant != Some(retractor) {
                return;
            }
            st.value = None;
            st.informant = None;
            st.constraints.clone()
        };
        for c in notify {
            if !matches!(retractor, Informant::Constraint(id) if id == c.id()) {
                c.process_forget_value();
            }
        }
    }
}

struct Adder {
    id: usize,
    a1: Connector,
    a2: Connector,
    sum: Connector,
}

impl Constraint for Adder {
    fn id(&self) -> usize {
        self.id
    }

    fn process_new_value(&self) {
        let me = Informant::Constraint(self.id);
        match (
            self.a1.get_value(),
            self.a2.get_value(),
            self.sum.get_value(),
        ) {
            (Some(x), Some(y), _) => {
                let _ = self.sum.set_value(x + y, me);
            }
            (Some(x), None, Some(s)) => {
                let _ = self.a2.set_value(s - x, me);
            }
            (None, Some(y), Some(s)) => {
                let _ = self.a1.set_value(s - y, me);
            }
            _ => {}
        }
    }

    fn process_forget_value(&self) {
        let me = Informant::Constraint(self.id);
        self.sum.forget_value(me);
        self.a1.forget_value(me);
        self.a2.forget_value(me);
        self.process_new_value();
    }
}

struct Multiplier {
    id: usize,
    m1: Connector,
    m2: Connector,
    product: Connector,
}

impl Constraint for Multiplier {
    fn id(&self) -> usize {
        self.id
    }

    fn process_new_value(&self) {
        let me = Informant::Constraint(self.id);
        let v1 = self.m1.get_value();
        let v2 = self.m2.get_value();
        let vp = self.product.get_value();
        if v1 == Some(0.0) || v2 == Some(0.0) {
            let _ = self.product.set_value(0.0, me);
        } else if let (Some(x), Some(y)) = (v1, v2) {
            let _ = self.product.set_value(x * y, me);
        } else if let (Some(p), Some(x)) = (vp, v1) {
            if x != 0.0 {
                let _ = self.m2.set_value(p / x, me);
            }
        } else if let (Some(p), Some(y)) = (vp, v2) {
            if y != 0.0 {
                let _ = self.m1.set_value(p / y, me);
            }
        }
    }

    fn process_forget_value(&self) {
        let me = Informant::Constraint(self.id);
        self.product.forget_value(me);
        self.m1.forget_value(me);
        self.m2.forget_value(me);
        self.process_new_value();
    }
}

/// Exercise 3.35: bidirectional squarer (a^2 = b, solving b -> sqrt(b) backward).
struct Squarer {
    id: usize,
    a: Connector,
    b: Connector,
}

impl Constraint for Squarer {
    fn id(&self) -> usize {
        self.id
    }

    fn process_new_value(&self) {
        let me = Informant::Constraint(self.id);
        if let Some(bv) = self.b.get_value() {
            if bv >= 0.0 {
                let _ = self.a.set_value(bv.sqrt(), me);
            }
        } else if let Some(av) = self.a.get_value() {
            let _ = self.b.set_value(av * av, me);
        }
    }

    fn process_forget_value(&self) {
        let me = Informant::Constraint(self.id);
        self.a.forget_value(me);
        self.b.forget_value(me);
        self.process_new_value();
    }
}

struct Network {
    next_id: usize,
}

impl Network {
    fn new() -> Self {
        Self { next_id: 1 }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn adder(&mut self, a1: &Connector, a2: &Connector, sum: &Connector) {
        let c: Rc<dyn Constraint> = Rc::new(Adder {
            id: self.next_id(),
            a1: a1.clone(),
            a2: a2.clone(),
            sum: sum.clone(),
        });
        a1.connect(Rc::clone(&c));
        a2.connect(Rc::clone(&c));
        sum.connect(c);
    }

    fn multiplier(&mut self, m1: &Connector, m2: &Connector, product: &Connector) {
        let c: Rc<dyn Constraint> = Rc::new(Multiplier {
            id: self.next_id(),
            m1: m1.clone(),
            m2: m2.clone(),
            product: product.clone(),
        });
        m1.connect(Rc::clone(&c));
        m2.connect(Rc::clone(&c));
        product.connect(c);
    }

    fn squarer(&mut self, a: &Connector, b: &Connector) {
        let c: Rc<dyn Constraint> = Rc::new(Squarer {
            id: self.next_id(),
            a: a.clone(),
            b: b.clone(),
        });
        a.connect(Rc::clone(&c));
        b.connect(c);
    }

    fn constant(&mut self, value: f64, out: &Connector) {
        let _ = out.set_value(value, Informant::Constraint(self.next_id()));
    }

    /// Exercise 3.37 expression-oriented combinators: c+, c*, cv.
    fn c_add(&mut self, x: &Connector, y: &Connector) -> Connector {
        let z = Connector::new();
        self.adder(x, y, &z);
        z
    }

    fn c_mul(&mut self, x: &Connector, y: &Connector) -> Connector {
        let z = Connector::new();
        self.multiplier(x, y, &z);
        z
    }

    fn c_div(&mut self, x: &Connector, y: &Connector) -> Connector {
        let z = Connector::new();
        self.multiplier(&z, y, x);
        z
    }

    fn cv(&mut self, value: f64) -> Connector {
        let z = Connector::new();
        self.constant(value, &z);
        z
    }

    /// Exercise 3.33: averager(a, b, c) enforces (a + b) = 2 * c.
    fn averager(&mut self, a: &Connector, b: &Connector, c: &Connector) {
        let two = self.cv(2.0);
        let sum = self.c_add(a, b);
        self.multiplier(&two, c, &sum);
    }
}

fn celsius_fahrenheit_converter(net: &mut Network, c: &Connector) -> Connector {
    // 9 * C = 5 * (F - 32)  =>  F = (9/5)*C + 32
    let nine = net.cv(9.0);
    let five = net.cv(5.0);
    let thirty_two = net.cv(32.0);
    let ratio = net.c_div(&nine, &five);
    let scaled = net.c_mul(&ratio, c);
    net.c_add(&scaled, &thirty_two)
}

fn main() {
    let mut net = Network::new();
    let c = Connector::new();
    let f = celsius_fahrenheit_converter(&mut net, &c);
    c.set_value(100.0, Informant::User).unwrap();
    let f_100 = f.get_value();
    c.forget_value(Informant::User);
    f.set_value(32.0, Informant::User).unwrap();
    let c_32 = c.get_value();

    let (a, b, avg, s_in, s_out) = (
        Connector::new(),
        Connector::new(),
        Connector::new(),
        Connector::new(),
        Connector::new(),
    );
    net.averager(&a, &b, &avg);
    a.set_value(10.0, Informant::User).unwrap();
    avg.set_value(20.0, Informant::User).unwrap();
    net.squarer(&s_in, &s_out);
    s_out.set_value(49.0, Informant::User).unwrap();
    println!(
        "100 C -> {:?} F; F=32 -> {:?} C; avg b={:?} (set={}); sqrt(49)={:?}",
        f_100,
        c_32,
        b.get_value(),
        b.has_value(),
        s_in.get_value()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retracting_value_propagates_only_from_informant() {
        let mut net = Network::new();
        let c = Connector::new();
        let f = celsius_fahrenheit_converter(&mut net, &c);

        c.set_value(25.0, Informant::User).unwrap();
        assert_eq!(f.get_value(), Some(77.0));
        assert!(f.set_value(100.0, Informant::User).is_err());

        // Retracting C forgets F, allowing reverse calculation from F=212.
        c.forget_value(Informant::User);
        assert!(!f.has_value());
        f.set_value(212.0, Informant::User).unwrap();
        assert_eq!(c.get_value(), Some(100.0));
    }

    #[test]
    fn averager_and_squarer_work_in_all_directions() {
        let mut net = Network::new();
        let (a, b, avg) = (Connector::new(), Connector::new(), Connector::new());
        net.averager(&a, &b, &avg);
        a.set_value(10.0, Informant::User).unwrap();
        avg.set_value(20.0, Informant::User).unwrap();
        assert_eq!(b.get_value(), Some(30.0));

        // Retract avg and set b=40 -> avg updates to 25 while user-set a=10 stays!
        avg.forget_value(Informant::User);
        assert!(!b.has_value());
        assert_eq!(a.get_value(), Some(10.0));
        b.set_value(40.0, Informant::User).unwrap();
        assert_eq!(avg.get_value(), Some(25.0));

        let (x, y) = (Connector::new(), Connector::new());
        net.squarer(&x, &y);
        y.set_value(49.0, Informant::User).unwrap();
        assert_eq!(x.get_value(), Some(7.0));
    }
}
