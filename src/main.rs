use std::env::args;
use std::io::{IsTerminal, stdin};
use std::process::exit;
fn main() {
    if stdin().is_terminal() {
        eprintln!("didnt get any input...");
        exit(1);
    }
    let (params, query) = get_env();
    grep(params, query);
}
fn get_env() -> (Vec<String>, String) {
    let mut envs: Vec<String> = args().collect();
    let mut query = String::new();
    let mut params: Vec<String> = Vec::new();
    envs.remove(0);
    for i in envs {
        match i {
            n if n.contains("-") => params.push(n),
            _ => query += &i,
        }
    }
    (params, query)
}
fn grep(params: Vec<String>, query: String) -> Option<String> {
    let mut matched: Vec<String> = Vec::new();
    for line in stdin().lines() {
        let line = line.ok()?;
        let mut idk: bool = if params.contains(&"-i".to_string()) {
            line.to_lowercase().contains(&query.to_lowercase())
        } else {
            line.contains(&query)
        };
        if params.contains(&"-v".to_string()) {
            idk = !idk;
        }
        if idk {
            matched.push(line);
        }
    }
    if !params.contains(&"-c".to_string()) {
        for i in matched {
            println!("{i}");
        }
    } else {
        println!("{}", matched.len());
    }
    None
}
