enum Structure { Weight(u64), Mobile(Box<Mobile>) }
struct Branch { length:u64, hanging:Structure }
struct Mobile { left:Branch, right:Branch }
fn weight(structure:&Structure)->u64 {
    match structure {
        Structure::Weight(w)=>*w,
        Structure::Mobile(m)=>total_weight(m),
    }
}
fn total_weight(m:&Mobile)->u64 {
    weight(&m.left.hanging)+weight(&m.right.hanging)
}
fn branch_balanced(branch:&Branch)->bool {
    match &branch.hanging {
        Structure::Weight(_)=>true,
        Structure::Mobile(child)=>balanced(child),
    }
}
fn balanced(m:&Mobile)->bool {
    // A balanced parent may still contain an unbalanced child.
    m.left.length*weight(&m.left.hanging)
      ==m.right.length*weight(&m.right.hanging)
      &&branch_balanced(&m.left)
      &&branch_balanced(&m.right)
}
fn branch(length:u64,weight:u64)->Branch {
    Branch{length,hanging:Structure::Weight(weight)}
}
fn main() {
    let mobile=Mobile{left:branch(2,3),right:branch(3,2)};
    println!("weight {}, balanced {}",total_weight(&mobile),balanced(&mobile));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checks_recursive_balance() {
        let inner=Mobile{left:branch(1,2),right:branch(1,2)};
        let outer=Mobile{left:Branch{length:1,hanging:Structure::Mobile(Box::new(inner))},
                         right:branch(2,2)};
        assert_eq!(total_weight(&outer),6);
        assert!(balanced(&outer));
        let wrong=Mobile{left:branch(1,2),right:branch(1,4)};
        assert!(!balanced(&wrong));
    }
}
