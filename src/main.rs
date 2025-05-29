use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
};

use parser::parser::Parser;
use scanner::scanner::Scanner;

mod bitwise;
mod chunks;
mod compiler;
mod errors;
mod parser;
mod scanner;
mod vm;

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
    let ast = parser.parse();
    if parser.has_errors() {
        parser.log_errors();
        anyhow::bail!("failure during parsing");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logger();

    match env::args().nth(1) {
        Some(path) => run_file(&path),
        None => anyhow::bail!("Usage: rox <file_path>"),
    }
}
