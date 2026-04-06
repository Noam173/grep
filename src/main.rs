use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal, Result, stdin};
use std::process::exit;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    insensitive: bool,

    #[arg(short = 'v', long)]
    exclude: bool,

    #[arg(short, long)]
    count: bool,

    query: String,
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.files.is_empty() && stdin().is_terminal() {
        true => {
            eprintln!("didnt recive any input");
            exit(1);
        }
        false => {
            if args.files.is_empty() {
                let reader = stdin();
                grep(reader.lock(), &args);
            } else {
                for file in &args.files {
                    let reader = BufReader::new(File::open(file)?);
                    grep(reader, &args);
                }
            }
        }
    }
    Ok(())
}
fn grep<R: BufRead>(reader: R, args: &Args) {
    let query = if args.insensitive {
        args.query.to_lowercase()
    } else {
        args.query.clone()
    };
    let mut count: usize = 0;
    let reader = reader.lines().map_while(Result::ok);
    if args.insensitive {
        reader
            .filter(|line| {
                let matched = line.to_lowercase().contains(&query);
                if args.exclude { !matched } else { matched }
            })
            .for_each(|line| {
                if !args.count {
                    println!("{}", line)
                } else {
                    count += 1
                }
            });
    } else {
        reader
            .filter(|line| {
                let matched = line.contains(&query);
                if args.exclude { !matched } else { matched }
            })
            .for_each(|line| {
                if !args.count {
                    println!("{}", line)
                } else {
                    count += 1
                }
            });
    }
    if args.count {
        println!("{}", count)
    };
}
