use std::{
    fs::read_to_string,
    io::{self, Write},
};

use parser::parser::Parser;
use scanner::scanner::Scanner;

mod bitwise;
mod chunks;
mod compiler;
mod parser;
mod scanner;
mod vm;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to to the file to execute
    path: String,
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

fn interpret(src: &str) -> anyhow::Result<()> {
    let mut scanner = Scanner::new(src);
    let tokens = scanner.scan()?;

    let mut parser = Parser::new(tokens);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logger();
    let cli = Cli::parse();
    return run_file(&cli.path);
}
