pub struct BankAccount {
    owner: String,
    balance: i32,
    last_transaction: Option<Transaction>,
}

pub enum Transaction {
    Deposit(i32),
    Withdraw(i32),
}

impl BankAccount {
    pub fn new(owner: String, balance: i32) -> Self {
        Self {
            owner,
            balance,
            last_transaction: None,
        }
    }

    fn deposit(&mut self, amount: i32) {
        if amount <= 0 {
            println!("Amount can't be 0 or less");
        } else {
            self.balance += amount;
            self.last_transaction = Some(Transaction::Deposit(amount))
        }
    }

    fn withdraw(&mut self, amount: i32) {
        if amount <= 0 {
            println!("Amount can't be 0 or less");
        } else if amount > self.balance {
            println!("Insufficient funds on the balance");
        } else {
            self.balance -= amount;
            self.last_transaction = Some(Transaction::Withdraw(amount))
        }
    }

    pub fn print_balance(&self) {
        println!("{}, balance: {}", self.owner, self.balance);
    }

    pub fn print_last_transaction(&self) {
        match &self.last_transaction {
            Some(transaction) => match transaction {
                Transaction::Deposit(amount) => println!("Last transaction: deposit of {amount}"),
                Transaction::Withdraw(amount) => println!("Last transaction: withdraw of {amount}"),
            },
            None => println!("No transactions yet"),
        }
    }

    pub fn undo_last_transaction(&mut self) {
        match &self.last_transaction {
            Some(transaction) => match transaction {
                Transaction::Deposit(amount) => {
                    self.balance -= amount;
                    self.last_transaction = None;
                }
                Transaction::Withdraw(amount) => {
                    self.balance += amount;
                    self.last_transaction = None;
                }
            },
            None => println!("No transactions to undo"),
        }
    }
}

pub fn apply_transaction(account: &mut BankAccount, transaction: Transaction) {
    match transaction {
        Transaction::Deposit(amount) => {
            account.deposit(amount);
            account.print_balance();
        }
        Transaction::Withdraw(amount) => {
            account.withdraw(amount);
            account.print_balance();
        }
    }
}
