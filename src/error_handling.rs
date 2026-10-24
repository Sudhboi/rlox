use crate::run::Lox;

impl Lox {
    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
}
