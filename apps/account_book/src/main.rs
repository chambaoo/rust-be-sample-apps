use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0")]
struct Args {
    arg1: String,
    arg2: String,
}

fn main() {
    // let command_name
    //     = std::env::args().nth(0).unwrap_or("CLI".to_string());

    // let name = std::env::args().nth(1).unwrap_or("World".to_string());

    // println!("Hello {} via {}", name, command_name);

    let _args = Args::parse();
}
