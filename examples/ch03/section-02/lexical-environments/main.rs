use std::collections::HashMap;
#[derive(Default)]
struct Environment {
    values: HashMap<String, i64>,
    parent: Option<Box<Environment>>,
}
impl Environment {
    fn child(parent: Self) -> Self {
        Self { values: HashMap::new(), parent: Some(Box::new(parent)) }
    }
    fn define(&mut self, name: &str, value: i64) {
        self.values.insert(name.to_owned(), value);
    }
    fn lookup(&self, name: &str) -> Option<i64> {
        // The closest frame takes precedence over any ancestor.
        self.values.get(name).copied().or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.lookup(name))
        })
    }
}
fn main() {
    let mut outer=Environment::default();
    outer.define("x",10);
    let mut inner=Environment::child(outer);
    inner.define("x",20);
    println!("inner x = {:?}",inner.lookup("x"));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn shadowing_and_free_variables() {
        let mut global=Environment::default();
        global.define("x",1);global.define("y",2);
        let mut local=Environment::child(global);local.define("x",99);
        assert_eq!(local.lookup("x"),Some(99));
        assert_eq!(local.lookup("y"),Some(2));
        assert_eq!(local.lookup("z"),None);
    }
}
