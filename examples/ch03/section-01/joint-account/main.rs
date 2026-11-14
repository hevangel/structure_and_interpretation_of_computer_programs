use std::cell::RefCell;
use std::rc::Rc;
struct Account { balance:u64 }
#[derive(Clone)]
struct Handle {
    shared:Rc<RefCell<Account>>,
    password:String,
}
impl Handle {
    fn new(balance:u64,password:&str)->Self {
        Self{shared:Rc::new(RefCell::new(Account{balance})),password:password.into()}
    }
    fn joint(&self,current:&str,new_password:&str)->Result<Self,&'static str> {
        if self.password!=current {return Err("authentication failed");}
        Ok(Self{shared:Rc::clone(&self.shared),password:new_password.into()})
    }
    fn withdraw(&self,provided:&str,amount:u64)->Result<u64,&'static str> {
        if provided!=self.password {return Err("authentication failed");}
        let mut account=self.shared.borrow_mut();
        if amount>account.balance {return Err("insufficient funds");}
        account.balance-=amount;
        Ok(account.balance)
    }
    fn balance(&self,provided:&str)->Result<u64,&'static str> {
        if provided!=self.password {return Err("authentication failed");}
        Ok(self.shared.borrow().balance)
    }
}
fn main() {
    let alice=Handle::new(100,"alice");
    let bob=alice.joint("alice","bob").unwrap();
    bob.withdraw("bob",30).unwrap();
    println!("shared balance: {}",alice.balance("alice").unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn independent_passwords_shared_balance() {
        let a=Handle::new(100,"a");
        let b=a.joint("a","b").unwrap();
        assert!(a.joint("wrong","c").is_err());
        assert!(b.withdraw("a",10).is_err());
        assert_eq!(b.withdraw("b",40).unwrap(),60);
        assert_eq!(a.balance("a").unwrap(),60);
        assert!(a.withdraw("a",70).is_err());
        assert_eq!(b.balance("b").unwrap(),60);
    }
}
