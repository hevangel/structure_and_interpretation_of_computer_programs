use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellVal {
    Nil,
    Int(i64),
    Pair(usize),
}

#[derive(Clone, Copy, Debug)]
struct ConsCell {
    car: CellVal,
    cdr: CellVal,
}

#[derive(Default)]
struct PairHeap {
    cells: Vec<ConsCell>,
}

impl PairHeap {
    fn cons(&mut self, car: CellVal, cdr: CellVal) -> CellVal {
        let idx = self.cells.len();
        self.cells.push(ConsCell { car, cdr });
        CellVal::Pair(idx)
    }

    fn set_car(&mut self, pair: CellVal, val: CellVal) {
        if let CellVal::Pair(idx) = pair {
            self.cells[idx].car = val;
        }
    }

    fn set_cdr(&mut self, pair: CellVal, val: CellVal) {
        if let CellVal::Pair(idx) = pair {
            self.cells[idx].cdr = val;
        }
    }

    /// Naive count_pairs (Exercise 3.16): double-counts shared DAGs and diverges on cycles.
    fn naive_count_pairs(&self, val: CellVal, limit: usize) -> Option<usize> {
        let mut budget = limit;
        fn walk(heap: &PairHeap, v: CellVal, budget: &mut usize) -> Option<usize> {
            match v {
                CellVal::Pair(idx) => {
                    if *budget == 0 {
                        return None;
                    }
                    *budget -= 1;
                    let c = heap.cells[idx];
                    let l = walk(heap, c.car, budget)?;
                    let r = walk(heap, c.cdr, budget)?;
                    Some(1 + l + r)
                }
                _ => Some(0),
            }
        }
        walk(self, val, &mut budget)
    }

    /// Exercise 3.17: exact unique pair count using visited set.
    fn unique_count_pairs(&self, val: CellVal) -> usize {
        fn walk(heap: &PairHeap, v: CellVal, seen: &mut BTreeSet<usize>) {
            if let CellVal::Pair(idx) = v {
                if seen.insert(idx) {
                    let c = heap.cells[idx];
                    walk(heap, c.car, seen);
                    walk(heap, c.cdr, seen);
                }
            }
        }
        let mut seen = BTreeSet::new();
        walk(self, val, &mut seen);
        seen.len()
    }

    /// Exercise 3.19: O(1)-space Floyd tortoise-and-hare cycle detection along cdr spine.
    fn has_cdr_cycle(&self, head: CellVal) -> bool {
        let next = |v: CellVal| -> Option<CellVal> {
            match v {
                CellVal::Pair(idx) => Some(self.cells[idx].cdr),
                _ => None,
            }
        };
        let mut slow = head;
        let mut fast = head;
        while let (Some(f1), Some(s1)) = (next(fast), next(slow)) {
            let Some(f2) = next(f1) else {
                return false;
            };
            slow = s1;
            fast = f2;
            if slow == fast && matches!(slow, CellVal::Pair(_)) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug)]
struct DequeNode<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>,
}

/// Exercise 3.23: O(1) doubly-linked deque backed by an index arena.
struct Deque<T> {
    nodes: Vec<Option<DequeNode<T>>>,
    free: Vec<usize>,
    front: Option<usize>,
    rear: Option<usize>,
    len: usize,
}

impl<T> Deque<T> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free: Vec::new(),
            front: None,
            rear: None,
            len: 0,
        }
    }

    fn alloc(&mut self, node: DequeNode<T>) -> usize {
        if let Some(idx) = self.free.pop() {
            self.nodes[idx] = Some(node);
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Some(node));
            idx
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn front_insert(&mut self, value: T) {
        let old_front = self.front;
        let idx = self.alloc(DequeNode {
            value,
            prev: None,
            next: old_front,
        });
        if let Some(f) = old_front {
            self.nodes[f].as_mut().unwrap().prev = Some(idx);
        } else {
            self.rear = Some(idx);
        }
        self.front = Some(idx);
        self.len += 1;
    }

    fn rear_insert(&mut self, value: T) {
        let old_rear = self.rear;
        let idx = self.alloc(DequeNode {
            value,
            prev: old_rear,
            next: None,
        });
        if let Some(r) = old_rear {
            self.nodes[r].as_mut().unwrap().next = Some(idx);
        } else {
            self.front = Some(idx);
        }
        self.rear = Some(idx);
        self.len += 1;
    }

    fn front_delete(&mut self) -> Option<T> {
        let f = self.front?;
        let node = self.nodes[f].take().unwrap();
        self.free.push(f);
        self.front = node.next;
        if let Some(nf) = self.front {
            self.nodes[nf].as_mut().unwrap().prev = None;
        } else {
            self.rear = None;
        }
        self.len -= 1;
        Some(node.value)
    }

    fn rear_delete(&mut self) -> Option<T> {
        let r = self.rear?;
        let node = self.nodes[r].take().unwrap();
        self.free.push(r);
        self.rear = node.prev;
        if let Some(nr) = self.rear {
            self.nodes[nr].as_mut().unwrap().next = None;
        } else {
            self.front = None;
        }
        self.len -= 1;
        Some(node.value)
    }
}

fn main() {
    let mut heap = PairHeap::default();
    let a = heap.cons(CellVal::Int(1), CellVal::Nil);
    let b = heap.cons(a, a);
    let c = heap.cons(b, b); // 3 cells -> naive count = 7!
    println!(
        "3-cell DAG: naive={:?}, unique={}, cycle={}",
        heap.naive_count_pairs(c, 100),
        heap.unique_count_pairs(c),
        heap.has_cdr_cycle(c)
    );
    heap.set_car(a, CellVal::Int(99));
    heap.set_cdr(a, c);
    println!("after set_cdr cycle: has_cycle={}", heap.has_cdr_cycle(c));

    let mut dq = Deque::new();
    dq.front_insert(10);
    dq.rear_insert(20);
    dq.front_insert(5);
    println!(
        "deque pops: {:?}, {:?}, {:?}; empty={}",
        dq.front_delete(),
        dq.rear_delete(),
        dq.front_delete(),
        dq.is_empty()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_cell_graphs_return_3_4_7_and_cycle() {
        // 3 pairs linear: naive=3
        let mut h3 = PairHeap::default();
        let p3 = h3.cons(CellVal::Int(3), CellVal::Nil);
        let p2 = h3.cons(CellVal::Int(2), p3);
        let p1 = h3.cons(CellVal::Int(1), p2);
        assert_eq!(h3.naive_count_pairs(p1, 50), Some(3));
        assert_eq!(h3.unique_count_pairs(p1), 3);

        // 3 pairs DAG returning 4
        let mut h4 = PairHeap::default();
        let leaf = h4.cons(CellVal::Int(1), CellVal::Nil);
        let mid = h4.cons(leaf, leaf);
        let top = h4.cons(mid, CellVal::Nil);
        assert_eq!(h4.naive_count_pairs(top, 50), Some(4));
        assert_eq!(h4.unique_count_pairs(top), 3);

        // 3 pairs DAG returning 7
        let mut h7 = PairHeap::default();
        let x = h7.cons(CellVal::Int(1), CellVal::Nil);
        let y = h7.cons(x, x);
        let z = h7.cons(y, y);
        assert_eq!(h7.naive_count_pairs(z, 50), Some(7));
        assert_eq!(h7.unique_count_pairs(z), 3);
        assert!(!h7.has_cdr_cycle(z));

        // 3 pairs with cycle -> naive fails, unique=3, Floyd=true
        h7.set_cdr(x, z);
        assert_eq!(h7.naive_count_pairs(z, 50), None);
        assert_eq!(h7.unique_count_pairs(z), 3);
        assert!(h7.has_cdr_cycle(z));
    }

    #[test]
    fn deque_supports_o1_both_end_operations() {
        let mut dq = Deque::new();
        assert!(dq.is_empty());
        dq.rear_insert("b");
        dq.front_insert("a");
        dq.rear_insert("c");
        assert_eq!(dq.front_delete(), Some("a"));
        assert_eq!(dq.rear_delete(), Some("c"));
        assert_eq!(dq.rear_delete(), Some("b"));
        assert!(dq.is_empty());
        assert_eq!(dq.front_delete(), None);
    }
}
