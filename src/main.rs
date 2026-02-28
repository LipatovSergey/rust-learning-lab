use std::io::{self, stdin};

struct BankAccount {
    owner: String,
    balance: i32,
}

impl BankAccount {
    fn new(owner: String, balance: i32) -> Self {
        Self { owner, balance }
    }

    fn deposit(&mut self, amount: i32) {
        if amount <= 0 {
            println!("Amount can't be 0 or less");
        } else {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: i32) {
        if amount <= 0 {
            println!("Amount can't be 0 or less");
        } else if amount > self.balance {
            println!("Insufficient funds on the balance");
        } else {
            self.balance -= amount;
        }
    }

    fn print_balance(&self) {
        println!("{}, balance: {}", self.owner, self.balance);
    }
}

fn main() {
    let mut account1 = BankAccount::new(String::from("John Doe"), 10);
    println!(
        "New account owner: {}, on balance {}",
        account1.owner, account1.balance
    );

    account1.deposit(0);
    account1.print_balance();
    account1.deposit(-3);
    account1.print_balance();
    account1.deposit(5);
    account1.print_balance();

    account1.withdraw(0);
    account1.print_balance();
    account1.withdraw(-5);
    account1.print_balance();
    account1.withdraw(100);
    account1.print_balance();
    account1.withdraw(7);
    account1.print_balance();
}
