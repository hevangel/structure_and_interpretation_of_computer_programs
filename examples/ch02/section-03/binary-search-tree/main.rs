enum Set { Empty, Node(i32, Box<Set>, Box<Set>) }
impl Set {
    fn insert(self, value: i32) -> Self {
        match self {
            Self::Empty => Self::Node(value, Box::new(Self::Empty), Box::new(Self::Empty)),
            Self::Node(v, left, right) if value < v =>
                Self::Node(v, Box::new(left.insert(value)), right),
            Self::Node(v, left, right) if value > v =>
                Self::Node(v, left, Box::new(right.insert(value))),
            existing => existing,
        }
    }
    fn contains(&self, value: i32) -> bool {
        match self {
            Self::Empty => false,
            Self::Node(v, left, right) if value < *v => left.contains(value),
            Self::Node(v, left, right) if value > *v => right.contains(value),
            Self::Node(..) => true,
        }
    }
}
fn main() {
    let set=Set::Empty.insert(5).insert(2).insert(8);
    println!("contains 2? {}", set.contains(2));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn membership() {
        let set=Set::Empty.insert(5).insert(2).insert(8).insert(5);
        assert!(set.contains(5) && set.contains(8));
        assert!(!set.contains(7));
    }
}
