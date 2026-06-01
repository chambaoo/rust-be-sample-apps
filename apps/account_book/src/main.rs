use clap::{Parser, Subcommand};
use csv::Writer;

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
    let _args = App::parse();

    match _args.command {
        Command::New => new(),
        Command::Deposit => unimplemented!(),
        Command::Withdraw => unimplemented!(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}

fn new() {
    let mut writer = Writer::from_path("accounts.csv").unwrap();
    writer 
        .write_record(["日付", "用途", "金額"])
        .unwrap();

    writer.flush().unwrap();
}