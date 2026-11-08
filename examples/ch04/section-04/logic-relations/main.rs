use std::collections::HashMap;
#[derive(Clone,Copy)]
enum Term<'a> { Constant(&'a str), Variable(&'a str) }
fn matches<'a>(pattern:Term<'a>,fact:&'a str,env:&mut HashMap<&'a str,&'a str>)->bool {
    match pattern {
        Term::Constant(c)=>c==fact,
        Term::Variable(name)=>match env.get(name) {
            Some(existing)=>*existing==fact,
            None=>{env.insert(name,fact);true}
        }
    }
}
fn query<'a>(facts:&[(&'a str,&'a str)],parent:Term<'a>,child:Term<'a>)
    ->Vec<HashMap<&'a str,&'a str>> {
    facts.iter().filter_map(|&(a,b)|{
        let mut bindings=HashMap::new();
        if matches(parent,a,&mut bindings)&&matches(child,b,&mut bindings) {
            Some(bindings)
        } else {None}
    }).collect()
}
fn main() {
    let facts=[("ada","bob"),("ada","cora"),("bob","dana")];
    let result=query(&facts,Term::Constant("ada"),Term::Variable("child"));
    println!("ada has {} children",result.len());
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn bindings_and_consistency() {
        let facts=[("a","b"),("a","c"),("x","x")];
        let r=query(&facts,Term::Constant("a"),Term::Variable("who"));
        assert_eq!(r.len(),2);
        let same=query(&facts,Term::Variable("v"),Term::Variable("v"));
        assert_eq!(same.len(),1);
        assert_eq!(same[0]["v"],"x");
    }
}
