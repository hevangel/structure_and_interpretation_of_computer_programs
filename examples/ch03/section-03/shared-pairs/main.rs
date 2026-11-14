use std::cell::RefCell;
use std::rc::{Rc,Weak};
struct Node {
    value:i64,
    parent:RefCell<Weak<Node>>,
}
fn main() {
    let root=Rc::new(Node{value:10,parent:RefCell::new(Weak::new())});
    let child=Rc::new(Node{value:20,parent:RefCell::new(Rc::downgrade(&root))});
    println!("child {} parent {}",child.value,child.parent.borrow().upgrade().unwrap().value);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn weak_link_does_not_keep_parent_alive() {
        let child;
        {
            let parent=Rc::new(Node{value:1,parent:RefCell::new(Weak::new())});
            child=Rc::new(Node{value:2,parent:RefCell::new(Rc::downgrade(&parent))});
            let alias=Rc::clone(&parent);
            assert_eq!(alias.value,1);
            assert_eq!(Rc::strong_count(&parent),2);
            assert!(child.parent.borrow().upgrade().is_some());
        }
        assert!(child.parent.borrow().upgrade().is_none());
    }
}
