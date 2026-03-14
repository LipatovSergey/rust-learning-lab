mod account;
mod cli;
use crate::account::{BankAccount, Transaction, apply_transaction};
use crate::cli::{Command, parse_command, print_menu, read_i32};

fn main() {
    let mut account1 = BankAccount::new(String::from("John Doe"), 10);
    println!("New account owner: ",);
    account1.print_balance();

    loop {
        print_menu();

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
