#[derive(Clone,Debug)]
struct Node { children:Vec<usize>,marked:bool }
#[derive(Default)]
struct Heap {slots:Vec<Option<Node>>}
impl Heap {
    fn alloc(&mut self,children:Vec<usize>)->usize {
        let id=self.slots.len();
        self.slots.push(Some(Node{children,marked:false}));id
    }
    fn mark(&mut self,id:usize) {
        let children=match self.slots.get_mut(id).and_then(Option::as_mut) {
            None=>return,
            Some(node) if node.marked=>return,
            Some(node)=>{node.marked=true;node.children.clone()},
        };
        // Clone the edge list to end the mutable borrow before traversal.
        for child in children {self.mark(child);}
    }
    fn collect(&mut self,roots:&[usize])->usize {
        for &id in roots {self.mark(id);}
        let mut freed=0;
        for slot in &mut self.slots {
            if let Some(node)=slot {
                if node.marked {node.marked=false;} else {*slot=None;freed+=1;}
            }
        }
        freed
    }
}
fn main() {
    let mut h=Heap::default();
    let leaf=h.alloc(vec![]);let root=h.alloc(vec![leaf]);h.alloc(vec![]);
    println!("reclaimed {} unreachable node(s)",h.collect(&[root]));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn graph_reachability() {
        let mut h=Heap::default();
        let a=h.alloc(vec![]);let b=h.alloc(vec![a]);let dead=h.alloc(vec![]);
        assert_eq!(h.collect(&[b]),1);
        assert!(h.slots[dead].is_none());
        assert!(h.slots[a].is_some());
        assert_eq!(h.collect(&[]),2);
    }
}
