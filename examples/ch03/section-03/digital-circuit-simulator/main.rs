use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[derive(Default)]
struct Simulator {
    // Wire 0 and 1 are inputs; wire 2 is an AND-gate output.
    wires: [bool;3],
    events: BinaryHeap<Reverse<(u64,u64,usize,bool)>>,
    serial: u64,
    history: Vec<(u64,usize,bool)>,
}
impl Simulator {
    fn schedule(&mut self,time:u64,wire:usize,value:bool) {
        self.serial+=1;
        // The sequence number establishes deterministic order at equal times.
        self.events.push(Reverse((time,self.serial,wire,value)));
    }
    fn run(&mut self) {
        while let Some(Reverse((time,_,wire,value)))=self.events.pop() {
            assert!(wire<self.wires.len());
            if self.wires[wire]==value {continue;}
            self.wires[wire]=value;
            self.history.push((time,wire,value));
            if wire<2 {
                // A gate reacts to each change after two simulation time units.
                let result=self.wires[0]&&self.wires[1];
                self.schedule(time+2,2,result);
            }
        }
    }
}
fn main() {
    let mut sim=Simulator::default();
    sim.schedule(1,0,true);sim.schedule(3,1,true);sim.run();
    println!("AND output: {}, trace: {:?}",sim.wires[2],sim.history);
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn gate_has_propagation_delay() {
        let mut s=Simulator::default();
        s.schedule(1,0,true);s.schedule(3,1,true);s.schedule(6,0,false);s.run();
        assert!(s.history.contains(&(5,2,true)));
        assert!(s.history.contains(&(8,2,false)));
        assert!(!s.wires[2]);
    }
}
