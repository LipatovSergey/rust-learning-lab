use std::io;

struct BankAccount {
    owner: String,
    balance: i32,
    last_transaction: Option<Transaction>,
}

enum Command {
    Deposit,
    Withdraw,
    ShowBalance,
    ShowLastTransaction,
    UndoLastTransaction,
    Exit,
}

enum Transaction {
    Deposit(i32),
    Withdraw(i32),
}

impl BankAccount {
    fn new(owner: String, balance: i32) -> Self {
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

    fn print_balance(&self) {
        println!("{}, balance: {}", self.owner, self.balance);
    }

    fn print_last_transaction(&self) {
        match &self.last_transaction {
            Some(transaction) => match transaction {
                Transaction::Deposit(amount) => println!("Last transaction: deposit of {amount}"),
                Transaction::Withdraw(amount) => println!("Last transaction: withdraw of {amount}"),
            },
            None => println!("No transactions yet"),
        }
    }

    fn undo_last_transaction(&mut self) {
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

fn read_i32() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
}

fn parse_command(n: i32) -> Option<Command> {
    match n {
        1 => Some(Command::Deposit),
        2 => Some(Command::Withdraw),
        3 => Some(Command::ShowBalance),
        4 => Some(Command::ShowLastTransaction),
        5 => Some(Command::UndoLastTransaction),
        0 => Some(Command::Exit),
        _ => None,
    }
}

fn apply_transaction(account: &mut BankAccount, transaction: Transaction) {
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

fn main() {
    let mut account1 = BankAccount::new(String::from("John Doe"), 10);
    println!(
        "New account owner: {}, on balance {}",
        account1.owner, account1.balance
    );

    loop {
        println!(
            "
1) Deposit
2) Withdraw
3) Show balance
4) Show last transaction
5) Cancel last transaction
0) Exit
        "
        );

        let raw = read_i32();
        let cmd_opt = parse_command(raw);

        match cmd_opt {
            Some(Command::Deposit) => {
                println!("Please enter the amount to deposit");
                let amount = read_i32();
                apply_transaction(&mut account1, Transaction::Deposit(amount));
            }
            Some(Command::Withdraw) => {
                println!("Please enter the amount to withdraw");
                let amount = read_i32();
                apply_transaction(&mut account1, Transaction::Withdraw(amount));
            }
            Some(Command::ShowBalance) => {
                println!("Your balance is");
                account1.print_balance();
            }
            Some(Command::ShowLastTransaction) => {
                account1.print_last_transaction();
            }
            Some(Command::UndoLastTransaction) => {
                account1.undo_last_transaction();
                account1.print_balance();
            }
            Some(Command::Exit) => {
                println!("Exit");
                break;
            }
            None => println!("Unknown command"),
        }
    }
}
