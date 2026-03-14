use std::io;

pub enum Command {
    Deposit,
    Withdraw,
    ShowBalance,
    ShowLastTransaction,
    UndoLastTransaction,
    Exit,
}

impl Command {
    fn menu_line(&self) -> &'static str {
        match self {
            Self::Deposit => "1) Deposit",
            Self::Withdraw => "2) Withdraw",
            Self::ShowBalance => "3) Show Balance",
            Self::ShowLastTransaction => "4) Show last transaction",
            Self::UndoLastTransaction => "5) Undo last transaction",
            Self::Exit => "0) Exit",
        }
    }
}

pub fn print_menu() {
    println!(" ");
    println!("{}", Command::Deposit.menu_line());
    println!("{}", Command::Withdraw.menu_line());
    println!("{}", Command::ShowBalance.menu_line());
    println!("{}", Command::ShowLastTransaction.menu_line());
    println!("{}", Command::UndoLastTransaction.menu_line());
    println!("{}", Command::Exit.menu_line());
    println!(" ");
}

pub fn read_i32() -> i32 {
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

pub fn parse_command(n: i32) -> Option<Command> {
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
