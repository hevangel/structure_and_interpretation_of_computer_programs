use std::sync::{Arc,Mutex};
use std::thread;
fn transfer(bank: &Mutex<[u64;2]>, from: usize, amount: u64) {
    let mut balances=bank.lock().expect("mutex should not be poisoned");
    let to=1-from;
    if balances[from] >= amount {
        balances[from]-=amount;
        balances[to]+=amount;
    }
    // Both entries are changed under the same lock.
    assert_eq!(balances[0]+balances[1],1000);
}
fn run() -> [u64;2] {
    let bank=Arc::new(Mutex::new([500,500]));
    let handles:Vec<_>=(0..4).map(|worker|{
        let shared=Arc::clone(&bank);
        thread::spawn(move || {
            for _ in 0..1000 {
                transfer(&shared,worker%2,1);
                transfer(&shared,(worker+1)%2,1);
            }
        })
    }).collect();
    for h in handles {h.join().expect("thread completed");}
    let result=*bank.lock().expect("mutex should not be poisoned");
    result
}
fn main() { println!("final balances: {:?}",run()); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn value_is_conserved() {
        let [a,b]=run();
        assert_eq!(a+b,1000);
    }
}
