use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, IsTerminal};
use std::io::{BufRead, Result};
use std::process::exit;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    insensitive: bool,

    #[arg(short = 'v', long)]
    exclude: bool,

    #[arg(short, long)]
    count: bool,

    query: String,
    pub files: Vec<String>,
}
impl Default for Args {
    fn default() -> Self {
        Args {
            insensitive: false,
            exclude: false,
            count: false,
            query: "git".into(),
            files: vec!["/home/noam/.bashrc".into()],
        }
    }
}
pub fn check() -> Result<()> {
    let args = Args::parse();
    match args.files.is_empty() && stdin().is_terminal() {
        true => {
            eprintln!("didnt recive any input");
            exit(1);
        }
        false => {
            if args.files.is_empty() {
                let reader = stdin();
                run(reader.lock(), &args);
            } else {
                for file in &args.files {
                    let reader = BufReader::new(File::open(file)?);
                    run(reader, &args);
                }
            }
        }
    }
    Ok(())
}
pub fn run<R: BufRead>(reader: R, args: &Args) {
    let query = if args.insensitive {
        args.query.to_lowercase()
    } else {
        args.query.clone()
    };
    let reader = reader.lines().map_while(Result::ok);
    let n = reader.filter(|f| {
        let matched = if args.insensitive {
            f.to_lowercase().contains(&query)
        } else {
            f.contains(&query)
        };
        if args.exclude {
            !matched
        } else {
            matched
        }
    });
    if !args.count {
        n.for_each(|line| println!("{}", line));
    } else {
        println!("{}", n.count());
    }
}
