use std::env;

enum GivenCommand {
    HELP,
    SUM,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut given_command: GivenCommand = GivenCommand::HELP;
    if args.len() > 1 {
        if &args[1] == "sum" {
            given_command = GivenCommand::SUM;
            println!("sum");
        } else {
            println!("fallback to help");
        }
    }
    match given_command {
        GivenCommand::HELP => show_help(),
        GivenCommand::SUM => sum(),
    }
}


fn show_help() {
    println!("Usage: cargo run -- <command>");
    println!("command:");
    println!("  help: display this help");
    println!("  sum: perform a simple sum");
}

fn sum() {
    let a: i32 = 10;
    let b: i32 = 5;
    let tot: i32 = a + b;
    println!("{} + {} = {}", a, b, tot);
}
