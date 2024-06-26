use std::env;
use rust_learn::sortstr;

enum GivenCommand {
    HELP,
    SUM,
    SORT,
    SORTSTR,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut given_command: GivenCommand = GivenCommand::HELP;
    if args.len() > 1 {
        if &args[1] == "sum" {
            given_command = GivenCommand::SUM;
        } else if &args[1] == "sort" {
            given_command = GivenCommand::SORT;
        } else if &args[1] == "sortstr" {
            given_command = GivenCommand::SORTSTR;
        }
    }
    match given_command {
        GivenCommand::HELP => show_help(),
        GivenCommand::SUM => sum(),
        GivenCommand::SORT => sort(),
        GivenCommand::SORTSTR => sortstr::sort(),
    }
}


fn show_help() {
    println!("Usage: cargo run -- <command>");
    println!("command:");
    println!("  help: display this help");
    println!("  sum: perform a simple sum");
    println!("  sort: perform a bubblesort between numbers");
}

fn sum() {
    let a: i32 = 10;
    let b: i32 = 5;
    let tot: i32 = a + b;
    println!("{} + {} = {}", a, b, tot);
}

fn sort() {
    let mut numbers = vec![6, 2, 9, 5, 2, 1, 8, 4];
    for i in 0..numbers.len() {
        print!("{} ", numbers[i]);
    }
    println!(" - unsorted");
    // apply bubblesort
    let mut keep_cycling: bool = true;
    while keep_cycling {
        keep_cycling = false;
        for i in 0..numbers.len()-1 {
            if numbers[i] > numbers[i+1] {
                numbers.swap(i, i+1);
                keep_cycling = true;
            }
        }
    }
    for i in 0..numbers.len() {
        print!("{} ", numbers[i]);
    }
    println!(" - sorted");
}
