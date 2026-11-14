use std::collections::BTreeSet;
fn ancestors<'a>(child:&'a str,facts:&[(&'a str,&'a str)])->BTreeSet<&'a str> {
    let mut visited=BTreeSet::new();
    let mut frontier=vec![child];
    while let Some(person)=frontier.pop() {
        for &(parent,offspring) in facts {
            if offspring==person && visited.insert(parent) {
                // Newly learned parent may have a parent of their own.
                frontier.push(parent);
            }
        }
    }
    visited.remove(child); // A cycle should not make someone their own ancestor.
    visited
}
fn main() {
    let facts=[("Ada","Bob"),("Bob","Carol"),("Ada","Dan")];
    println!("Carol's ancestors: {:?}",ancestors("Carol",&facts));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn transitive_facts() {
        let facts=[("Ada","Bob"),("Bob","Carol"),("Ada","Dan")];
        assert_eq!(ancestors("Carol",&facts),BTreeSet::from(["Ada","Bob"]));
        assert_eq!(ancestors("Dan",&facts),BTreeSet::from(["Ada"]));
        assert!(ancestors("Unknown",&facts).is_empty());
    }
}
