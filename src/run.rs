use crate::scanner::{self, Scanner};
use std::io::{self, Write};
use std::{fs, process};

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    fn build() -> Lox {
        Lox { had_error: false }
    }
    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::build(self, source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{token}");
        }
    }
}

pub fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path);
    let mut program = Lox::build();
    match contents {
        Ok(source) => program.run(&source),
        Err(e) => println!("Error reading file! {e}"),
    }
    if program.had_error {
        process::exit(65);
    }
}

pub fn run_prompt() {
    let mut program = Lox::build();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let read = io::stdin().read_line(&mut line).unwrap();
        if read == 0 {
            println!();
            process::exit(0);
        }
        match line.as_str() {
            "\n" => break,
            other => program.run(other.trim_end()),
        }
        program.had_error = false;
    }
}
