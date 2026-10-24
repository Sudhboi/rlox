use crate::{
    run::Lox,
    tokens::{
        Literal, Token,
        TokenType::{self, *},
    },
};

pub struct Scanner<'a> {
    program: &'a mut Lox,
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'_> {
    pub fn build(program: &'a mut Lox, source: &str) -> Scanner<'a> {
        Scanner {
            program,
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::build(token_type, text, literal, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn check_next(&mut self, expected: char, token: TokenType, default: TokenType) {
        if self.match_next(expected) {
            self.add_token(token);
        } else {
            self.add_token(default);
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),

            '!' => self.check_next('=', BangEqual, Bang),
            '=' => self.check_next('=', EqualEqual, Equal),
            '>' => self.check_next('=', GreaterEqual, Greater),
            '<' => self.check_next('=', LessEqual, Less),

            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }

            ' ' | '\r' | '\t' => {}

            '\n' => {
                self.line += 1;
            }

            _ => self.program.error(self.line, "Unexpected Character"),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        &self.tokens
    }
}
