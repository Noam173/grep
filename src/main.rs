use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, stdin};

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

    if args.files.is_empty() {
        let reader = stdin();
        process(reader.lock(), &args)?;
    } else {
        for file in &args.files {
            let reader = BufReader::new(File::open(file)?);
            process(reader, &args)?;
        }
    }

    Ok(())
}

fn process<R: BufRead>(reader: R, args: &Args) -> Result<()> {
    let query = if args.insensitive {
        args.query.to_lowercase()
    } else {
        args.query.clone()
    };

    let mut count: u16 = 0;

    for line in reader.lines() {
        let line = line?;

        let matched = if args.insensitive {
            line.to_lowercase().contains(&query)
        } else {
            line.contains(&query)
        };

        let matched = if args.exclude { !matched } else { matched };

        if matched {
            if !args.count {
                println!("{}", line);
            } else {
                count += 1;
            }
        }
    }

    if args.count {
        println!("{}", count);
    }

    Ok(())
}
