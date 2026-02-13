use clap::Parser;
use std::{
    io::{IsTerminal, stdin},
    process::exit,
};
#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    insensitive: bool,
    #[arg(short = 'v', long, default_value_t = false)]
    exclude: bool,
    #[arg(short, long, default_value_t = false)]
    count: bool,
    query: String,
}

fn main() {
    let args = Args::parse();
    if stdin().is_terminal() && args.query.is_empty() {
        eprint!("didnt recive any file as output... \n");
        exit(1);
    }
    let mut matched: Vec<String> = Vec::new();
    for line in stdin().lines() {
        let line = line.unwrap_or("couldnt read file".to_string());
        let mut i: bool = if args.insensitive {
            line.to_lowercase().contains(&args.query.to_lowercase())
        } else {
            line.contains(&args.query)
        };
        if args.exclude {
            i = !i;
        }
        if i {
            matched.push(line);
        }
    }
    if args.count {
        println!("{}", matched.len());
    } else {
        for i in matched {
            println!("{}", i);
        }
    }
}
