use clap::Parser;
use std::{fs, path};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
