use clap::Parser;
use std::io::{Result, stdin};
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    insensitive: bool,
    #[arg(short = 'v', long)]
    exclude: bool,
    #[arg(short, long)]
    count: bool,
    query: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    grep(args)
}
fn grep(args: Args) -> Result<()> {
    let lines = stdin().lines();
    let mut matched: Vec<String> = Vec::new();
    let l_query = args.query.to_lowercase();

    for line in lines {
        let line = line?;
        let mut i: bool = if args.insensitive {
            line.to_lowercase().contains(&l_query)
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
    Ok(())
}
