/*
  Problem 46: Module Visibility

  Create a struct Account with a private field balance: f64 and public methods
  new(initial), deposit(amount), withdraw(amount), and balance().
  The balance should never go negative. Withdraw returns Result<(), String>.

  Run the tests for this problem with:
    cargo test --test module_visibility_test
*/

pub struct Account {
    balance: f64,
}

impl Account {
    pub fn new(initial: f64) -> Self {
        Self { balance: initial }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        return
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance - amount < 0.0 {
            Err(String::from("Can't withdraw as the balance is less"))
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}
