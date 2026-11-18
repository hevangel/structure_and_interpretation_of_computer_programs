use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Pointer {
    Nil,
    Int(i64),
    Sym(String),
    Pair(usize),
    BrokenHeart(usize),
}

struct StopAndCopyMemory {
    capacity: usize,
    the_cars: Vec<Pointer>,
    the_cdrs: Vec<Pointer>,
    new_cars: Vec<Pointer>,
    new_cdrs: Vec<Pointer>,
    free: usize,
    gc_cycles: usize,
}

impl StopAndCopyMemory {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            the_cars: vec![Pointer::Nil; capacity],
            the_cdrs: vec![Pointer::Nil; capacity],
            new_cars: vec![Pointer::Nil; capacity],
            new_cdrs: vec![Pointer::Nil; capacity],
            free: 0,
            gc_cycles: 0,
        }
    }

    fn cons(&mut self, car: Pointer, cdr: Pointer, root: &mut Pointer) -> Result<Pointer, String> {
        if self.free >= self.capacity {
            *root = self.collect_with_roots(std::slice::from_ref(root));
            if self.free >= self.capacity {
                return Err("out of pair memory after GC".to_string());
            }
        }
        let idx = self.free;
        self.the_cars[idx] = car;
        self.the_cdrs[idx] = cdr;
        self.free += 1;
        Ok(Pointer::Pair(idx))
    }

    fn cons_raw(&mut self, car: Pointer, cdr: Pointer) -> Result<Pointer, String> {
        if self.free >= self.capacity {
            return Err("out of raw pair memory".to_string());
        }
        let idx = self.free;
        self.the_cars[idx] = car;
        self.the_cdrs[idx] = cdr;
        self.free += 1;
        Ok(Pointer::Pair(idx))
    }

    fn car(&self, p: &Pointer) -> Result<Pointer, String> {
        match p {
            Pointer::Pair(idx) => Ok(self.the_cars[*idx].clone()),
            other => Err(format!("car on non-pair: {other:?}")),
        }
    }

    fn cdr(&self, p: &Pointer) -> Result<Pointer, String> {
        match p {
            Pointer::Pair(idx) => Ok(self.the_cdrs[*idx].clone()),
            other => Err(format!("cdr on non-pair: {other:?}")),
        }
    }

    fn set_cdr(&mut self, p: &Pointer, val: Pointer) -> Result<(), String> {
        match p {
            Pointer::Pair(idx) => {
                self.the_cdrs[*idx] = val;
                Ok(())
            }
            other => Err(format!("set-cdr on non-pair: {other:?}")),
        }
    }

    fn relocate_old_result_in_new(&mut self, old: Pointer, new_free: &mut usize) -> Pointer {
        match old {
            Pointer::Pair(old_idx) => match &self.the_cars[old_idx] {
                Pointer::BrokenHeart(forwarded_idx) => Pointer::Pair(*forwarded_idx),
                _ => {
                    let target = *new_free;
                    *new_free += 1;
                    self.new_cars[target] = self.the_cars[old_idx].clone();
                    self.new_cdrs[target] = self.the_cdrs[old_idx].clone();
                    self.the_cars[old_idx] = Pointer::BrokenHeart(target);
                    self.the_cdrs[old_idx] = Pointer::Nil;
                    Pointer::Pair(target)
                }
            },
            other => other,
        }
    }

    fn collect_with_roots(&mut self, roots: &[Pointer]) -> Pointer {
        let relocated = self.collect_all_roots(roots);
        relocated.into_iter().next().unwrap_or(Pointer::Nil)
    }

    fn collect_all_roots(&mut self, roots: &[Pointer]) -> Vec<Pointer> {
        let mut new_free = 0;
        let mut scan = 0;

        let new_roots: Vec<Pointer> = roots
            .iter()
            .map(|r| self.relocate_old_result_in_new(r.clone(), &mut new_free))
            .collect();

        while scan < new_free {
            let old_car = self.new_cars[scan].clone();
            let old_cdr = self.new_cdrs[scan].clone();
            self.new_cars[scan] = self.relocate_old_result_in_new(old_car, &mut new_free);
            self.new_cdrs[scan] = self.relocate_old_result_in_new(old_cdr, &mut new_free);
            scan += 1;
        }

        // Swap `the` and `new` spaces (gc-flip).
        std::mem::swap(&mut self.the_cars, &mut self.new_cars);
        std::mem::swap(&mut self.the_cdrs, &mut self.new_cdrs);
        for slot in self.new_cars.iter_mut() {
            *slot = Pointer::Nil;
        }
        for slot in self.new_cdrs.iter_mut() {
            *slot = Pointer::Nil;
        }
        self.free = new_free;
        self.gc_cycles += 1;
        new_roots
    }

    fn extract_graph_str(&self, p: &Pointer) -> String {
        let mut visiting = BTreeSet::new();
        self.format_node(p, &mut visiting)
    }

    fn format_node(&self, p: &Pointer, visiting: &mut BTreeSet<usize>) -> String {
        match p {
            Pointer::Nil => "()".into(),
            Pointer::Int(n) => n.to_string(),
            Pointer::Sym(s) => s.clone(),
            Pointer::BrokenHeart(idx) => format!("<broken-heart:{idx}>"),
            Pointer::Pair(idx) => {
                if !visiting.insert(*idx) {
                    return "<cycle>".into();
                }
                let a = self.format_node(&self.the_cars[*idx], visiting);
                let d = self.format_node(&self.the_cdrs[*idx], visiting);
                visiting.remove(idx);
                format!("({a} . {d})")
            }
        }
    }
}

fn main() {
    let mut mem = StopAndCopyMemory::new(16);
    // Allocate garbage pairs
    for i in 0..6 {
        let _ = mem.cons_raw(Pointer::Int(i), Pointer::Nil).unwrap();
    }
    // Allocate shared DAG: root = cons(shared, shared)
    let shared = mem
        .cons_raw(Pointer::Sym("leaf".into()), Pointer::Nil)
        .unwrap();
    let root = mem.cons_raw(shared.clone(), shared).unwrap();
    let before = mem.extract_graph_str(&root);
    let before_free = mem.free;
    let mut root = mem.collect_with_roots(&[root]);
    let head = mem.cons(Pointer::Int(7), Pointer::Nil, &mut root).unwrap();
    let _ = mem.set_cdr(&head, Pointer::Int(8));
    println!(
        "compacted free: {} -> {}, cycles = {}, graph preserved: {} == {}, head=({:?} . {:?})",
        before_free,
        mem.free,
        mem.gc_cycles,
        before,
        mem.extract_graph_str(&root),
        mem.car(&head),
        mem.cdr(&head)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_and_copy_compacts_shared_dag_without_duplicating() {
        let mut mem = StopAndCopyMemory::new(16);
        for i in 0..8 {
            let _ = mem.cons_raw(Pointer::Int(i), Pointer::Nil).unwrap();
        }
        let shared = mem.cons_raw(Pointer::Int(10), Pointer::Int(20)).unwrap();
        let root = mem.cons_raw(shared.clone(), shared).unwrap();
        assert_eq!(mem.free, 10);

        let new_root = mem.collect_with_roots(&[root]);
        // Only 2 live pairs: `new_root` and `shared`, compacted to indices 0 and 1!
        assert_eq!(mem.free, 2);
        let car_ptr = mem.car(&new_root).unwrap();
        let cdr_ptr = mem.cdr(&new_root).unwrap();
        assert_eq!(car_ptr, cdr_ptr); // Shared identity preserved via BrokenHeart!
        assert_eq!(mem.car(&car_ptr).unwrap(), Pointer::Int(10));
        assert_eq!(mem.cdr(&car_ptr).unwrap(), Pointer::Int(20));
    }

    #[test]
    fn handles_cyclic_lists_and_consecutive_flips() {
        let mut mem = StopAndCopyMemory::new(8);
        let tail = mem.cons_raw(Pointer::Int(2), Pointer::Nil).unwrap();
        let head = mem.cons_raw(Pointer::Int(1), tail.clone()).unwrap();
        mem.set_cdr(&tail, head.clone()).unwrap();

        let mut root = head;
        for cycle in 1..=4 {
            let _ = mem.cons_raw(Pointer::Int(999), Pointer::Nil).unwrap();
            root = mem.collect_with_roots(&[root]);
            assert_eq!(mem.free, 2);
            assert_eq!(mem.gc_cycles, cycle);
            assert_eq!(mem.car(&root).unwrap(), Pointer::Int(1));
            let second = mem.cdr(&root).unwrap();
            assert_eq!(mem.car(&second).unwrap(), Pointer::Int(2));
            assert_eq!(mem.cdr(&second).unwrap(), root);
        }

        // `cons` auto-triggers GC when memory fills
        let mut list_root = root;
        for i in 0..3 {
            list_root = mem
                .cons(Pointer::Int(i), Pointer::Nil, &mut list_root)
                .unwrap();
        }
        assert!(mem.free <= 8);
    }
}
