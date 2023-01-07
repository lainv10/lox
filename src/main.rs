use std::{path::Path, io::Write};
use std::fs::read_to_string;
use anyhow::{bail, Context};
use anyhow::Result;
use lox::scanner::Scanner;

fn main() -> Result<()> {
    match std::env::args().count() {
        1 => run_prompt(),
        2 => run_file(std::env::args().nth(1).unwrap()),
        _ => bail!("Usage: lox [script]"),
    }
}

fn run_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let src = read_to_string(path).context("Failed to read source from given path")?;
    let tokens = Scanner::new(src).scan();
    dbg!(tokens);
    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    let mut input = String::new();
    loop {
        print!("> ");

        if std::io::stdout().flush().is_err() {
            eprintln!("Failed to flush stdout");
            continue;
        }

        match stdin.read_line(&mut input) {
            Ok(_) => {
                let tokens = Scanner::new(input.clone()).scan();
                for token in tokens {
                    println!("{}", token);
                }
                // println!("{tokens:?}");
            }
            Err(error) => eprintln!("Error reading line: {error}"),
        }

        // clear input buffer
        input.clear();
    }
}
