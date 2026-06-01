use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New,
    Deposit,
    Withdraw,
    Import,
    Report,
}

fn main() {
    // let command_name
    //     = std::env::args().nth(0).unwrap_or("CLI".to_string());

    // let name = std::env::args().nth(1).unwrap_or("World".to_string());

    // println!("Hello {} via {}", name, command_name);

    let _args = App::parse();
}
