use std::{
    fs::read_to_string,
    io::{self, Write},
};

use clap::Parser;
use scanner::scanner::Scanner;

mod bitwise;
mod chunks;
mod scanner;
mod vm;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to to the file to execute
    path: Option<String>,
}

#[allow(unused_must_use)]
fn init_logger() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .try_init();
}

fn run_file(path: &str) -> anyhow::Result<()> {
    let src = read_to_string(path)?;
    interpret(&src)?;
    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut buf = String::with_capacity(1024);

    loop {
        // --- prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // --- read input and run
        io::stdin().read_line(&mut buf)?;
        if buf == String::from("exit") {
            break;
        }
        interpret(&buf)?;

        // --- clear buffer for next read
        buf.clear();
    }

    Ok(())
}

fn interpret(src: &str) -> anyhow::Result<()> {
    let mut scanner = Scanner::new(src);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logger();
    let cli = Cli::parse();
    match cli.path {
        Some(path) => run_file(&path),
        None => repl(),
    }
}
