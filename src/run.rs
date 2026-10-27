use crate::scanner::Scanner;
use crate::tokens::{TokenType, make_keywords_map};
use std::collections::HashMap;
use std::io::{self, Write};
use std::{fs, process};

pub struct Lox {
    pub had_error: bool,
    keywords_map: HashMap<String, TokenType>,
}

impl Lox {
    fn build() -> Lox {
        Lox {
            had_error: false,
            keywords_map: make_keywords_map(),
        }
    }
    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::build(self, source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{token}");
        }
    }
    pub fn get_keyword(&self, word: &[char]) -> &TokenType {
        let val: String = word.iter().collect();
        self.keywords_map
            .get(&val)
            .unwrap_or(&TokenType::Identifier)
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
