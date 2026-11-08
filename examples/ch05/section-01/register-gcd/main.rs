#[derive(Debug)]
struct Machine { a:u64,b:u64,steps:usize }
impl Machine {
    fn run(&mut self) {
        while self.b!=0 {
            // A register transfer: (a,b) <- (b,a mod b).
            let remainder=self.a%self.b;
            self.a=self.b;self.b=remainder;
            self.steps+=1;
        }
    }
}
fn main() {
    let mut m=Machine{a:206,b:40,steps:0};
    m.run();println!("gcd={} steps={}",m.a,m.steps);
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn machine_state() {
        let mut m=Machine{a:206,b:40,steps:0};
        m.run();
        assert_eq!((m.a,m.b),(2,0));
        assert!(m.steps>0);
    }
}
