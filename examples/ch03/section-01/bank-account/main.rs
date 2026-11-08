#[derive(Debug)]
struct Account { balance: u64 }
impl Account {
    fn new(balance: u64) -> Self { Self { balance } }
    fn deposit(&mut self, amount: u64) -> Result<(), &'static str> {
        self.balance = self.balance.checked_add(amount).ok_or("balance overflow")?;
        Ok(())
    }
    fn withdraw(&mut self, amount: u64) -> Result<(), &'static str> {
        if amount > self.balance { return Err("insufficient funds"); }
        self.balance -= amount;
        Ok(())
    }
    fn balance(&self) -> u64 { self.balance }
}
fn main() {
    let mut a = Account::new(100);
    a.withdraw(30).expect("enough funds");
    println!("remaining balance: {}", a.balance());
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn history_changes_results() {
        let mut a=Account::new(20);
        a.deposit(5).unwrap();
        assert!(a.withdraw(30).is_err());
        assert_eq!(a.balance(),25);
        a.withdraw(10).unwrap();
        assert_eq!(a.balance(),15);
    }
}
