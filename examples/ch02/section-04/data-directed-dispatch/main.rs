use std::any::Any;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct TaggedDatum {
    tag: String,
    contents: Rc<dyn Any>,
}

impl TaggedDatum {
    fn new<T: 'static>(tag: &str, value: T) -> Self {
        Self {
            tag: tag.to_string(),
            contents: Rc::new(value),
        }
    }

    fn downcast<T: 'static>(&self) -> &T {
        self.contents
            .downcast_ref::<T>()
            .expect("tagged payload matches expected representation")
    }
}

type OpKey = (String, Vec<String>);
type DispatchFn = Rc<dyn Fn(&[TaggedDatum]) -> Result<TaggedDatum, String>>;

#[derive(Default)]
struct DispatchTable {
    table: BTreeMap<OpKey, DispatchFn>,
}

impl DispatchTable {
    fn put(&mut self, op: &str, type_tags: &[&str], proc: DispatchFn) {
        let key = (
            op.to_string(),
            type_tags.iter().map(|s| (*s).to_string()).collect(),
        );
        self.table.insert(key, proc);
    }

    fn get(&self, op: &str, type_tags: &[String]) -> Option<&DispatchFn> {
        self.table.get(&(op.to_string(), type_tags.to_vec()))
    }

    /// Core data-directed dispatch: never edits when new type packages are registered.
    fn apply_generic(&self, op: &str, args: &[TaggedDatum]) -> Result<TaggedDatum, String> {
        let tags: Vec<String> = args.iter().map(|a| a.tag.clone()).collect();
        let proc = self
            .get(op, &tags)
            .ok_or_else(|| format!("no method for ({op}, {tags:?})"))?;
        proc(args)
    }
}

fn install_rectangular_package(tbl: &mut DispatchTable) {
    tbl.put(
        "make_from_real_imag",
        &["rectangular"],
        Rc::new(|args| {
            let x = *args[0].downcast::<f64>();
            let y = *args[1].downcast::<f64>();
            Ok(TaggedDatum::new("rectangular", (x, y)))
        }),
    );
    tbl.put(
        "real_part",
        &["rectangular"],
        Rc::new(|args| {
            let &(x, _) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", x))
        }),
    );
    tbl.put(
        "imag_part",
        &["rectangular"],
        Rc::new(|args| {
            let &(_, y) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", y))
        }),
    );
    tbl.put(
        "magnitude",
        &["rectangular"],
        Rc::new(|args| {
            let &(x, y) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", x.hypot(y)))
        }),
    );
    tbl.put(
        "angle",
        &["rectangular"],
        Rc::new(|args| {
            let &(x, y) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", y.atan2(x)))
        }),
    );
}

fn install_polar_package(tbl: &mut DispatchTable) {
    tbl.put(
        "make_from_mag_ang",
        &["polar"],
        Rc::new(|args| {
            let r = *args[0].downcast::<f64>();
            let a = *args[1].downcast::<f64>();
            Ok(TaggedDatum::new("polar", (r, a)))
        }),
    );
    tbl.put(
        "real_part",
        &["polar"],
        Rc::new(|args| {
            let &(r, a) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", r * a.cos()))
        }),
    );
    tbl.put(
        "imag_part",
        &["polar"],
        Rc::new(|args| {
            let &(r, a) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", r * a.sin()))
        }),
    );
    tbl.put(
        "magnitude",
        &["polar"],
        Rc::new(|args| {
            let &(r, _) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", r))
        }),
    );
    tbl.put(
        "angle",
        &["polar"],
        Rc::new(|args| {
            let &(_, a) = args[0].downcast::<(f64, f64)>();
            Ok(TaggedDatum::new("f64", a))
        }),
    );
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SymExpr {
    Num(i64),
    Var(String),
    Tagged(String, Box<SymExpr>, Box<SymExpr>),
}

/// Exercise 2.73 style data-directed symbolic differentiation using a rule table.
type DerivRule = Rc<dyn Fn(&SymExpr, &SymExpr, &str, &DerivRegistry) -> SymExpr>;

#[derive(Default)]
struct DerivRegistry {
    rules: BTreeMap<String, DerivRule>,
}

impl DerivRegistry {
    fn put(&mut self, op: &str, rule: DerivRule) {
        self.rules.insert(op.to_string(), rule);
    }

    fn deriv(&self, expr: &SymExpr, var: &str) -> SymExpr {
        match expr {
            SymExpr::Num(_) => SymExpr::Num(0),
            SymExpr::Var(v) => SymExpr::Num(i64::from(v == var)),
            SymExpr::Tagged(op, left, right) => {
                let rule = self.rules.get(op).expect("registered deriv operator");
                rule(left, right, var, self)
            }
        }
    }
}

/// Message-passing alternative (2.4.3 / Exercise 2.75): data object dispatches on op string.
fn make_from_mag_ang_mp(r: f64, a: f64) -> Rc<dyn Fn(&str) -> f64> {
    Rc::new(move |op| match op {
        "real_part" => r * a.cos(),
        "imag_part" => r * a.sin(),
        "magnitude" => r,
        "angle" => a,
        _ => panic!("unknown message: {op}"),
    })
}

fn main() {
    let mut table = DispatchTable::default();
    install_rectangular_package(&mut table);
    install_polar_package(&mut table);

    let z1 = TaggedDatum::new("rectangular", (3.0f64, 4.0f64));
    let z2 = TaggedDatum::new("polar", (5.0f64, 0.0f64));
    let mag1 = *table
        .apply_generic("magnitude", &[z1])
        .unwrap()
        .downcast::<f64>();
    let real2 = *table
        .apply_generic("real_part", &[z2])
        .unwrap()
        .downcast::<f64>();
    let mp = make_from_mag_ang_mp(2.0, std::f64::consts::FRAC_PI_2);

    let mut reg = DerivRegistry::default();
    reg.put(
        "+",
        Rc::new(|u, v, var, r| {
            SymExpr::Tagged(
                "+".into(),
                Box::new(r.deriv(u, var)),
                Box::new(r.deriv(v, var)),
            )
        }),
    );
    let sum = SymExpr::Tagged(
        "+".into(),
        Box::new(SymExpr::Var("x".into())),
        Box::new(SymExpr::Num(3)),
    );
    println!(
        "mag(3+4i)={mag1}, real(5<0)={real2}, mp.imag={:.3}, d(x+3)/dx={:?}",
        mp("imag_part"),
        reg.deriv(&sum, "x")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_directed_complex_dispatch_supports_additive_types() {
        let mut table = DispatchTable::default();
        install_rectangular_package(&mut table);
        install_polar_package(&mut table);

        let rect = TaggedDatum::new("rectangular", (3.0f64, 4.0f64));
        let mag = *table
            .apply_generic("magnitude", std::slice::from_ref(&rect))
            .unwrap()
            .downcast::<f64>();
        let re = *table
            .apply_generic("real_part", &[rect])
            .unwrap()
            .downcast::<f64>();
        assert!((mag - 5.0).abs() < 1e-12);
        assert!((re - 3.0).abs() < 1e-12);

        let polar = TaggedDatum::new("polar", (2.0f64, std::f64::consts::FRAC_PI_2));
        let im = *table
            .apply_generic("imag_part", &[polar])
            .unwrap()
            .downcast::<f64>();
        assert!((im - 2.0).abs() < 1e-12);
    }

    #[test]
    fn additive_deriv_plugin_needs_no_core_edits() {
        let mut reg = DerivRegistry::default();
        reg.put(
            "+",
            Rc::new(|u, v, var, r| {
                SymExpr::Tagged(
                    "+".into(),
                    Box::new(r.deriv(u, var)),
                    Box::new(r.deriv(v, var)),
                )
            }),
        );
        reg.put(
            "*",
            Rc::new(|u, v, var, r| {
                let left =
                    SymExpr::Tagged("*".into(), Box::new(u.clone()), Box::new(r.deriv(v, var)));
                let right =
                    SymExpr::Tagged("*".into(), Box::new(r.deriv(u, var)), Box::new(v.clone()));
                SymExpr::Tagged("+".into(), Box::new(left), Box::new(right))
            }),
        );
        let expr = SymExpr::Tagged(
            "*".into(),
            Box::new(SymExpr::Var("x".into())),
            Box::new(SymExpr::Var("x".into())),
        );
        let d = reg.deriv(&expr, "x");
        assert_eq!(
            d,
            SymExpr::Tagged(
                "+".into(),
                Box::new(SymExpr::Tagged(
                    "*".into(),
                    Box::new(SymExpr::Var("x".into())),
                    Box::new(SymExpr::Num(1)),
                )),
                Box::new(SymExpr::Tagged(
                    "*".into(),
                    Box::new(SymExpr::Num(1)),
                    Box::new(SymExpr::Var("x".into())),
                )),
            )
        );
    }

    #[test]
    fn message_passing_object_dispatches_operations() {
        let mp = make_from_mag_ang_mp(4.0, 0.0);
        assert!((mp("real_part") - 4.0).abs() < 1e-12);
        assert!((mp("magnitude") - 4.0).abs() < 1e-12);
    }
}
