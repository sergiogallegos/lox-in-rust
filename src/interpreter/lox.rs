use std::fs;
use std::io::{self, Write, BufRead};
use std::process;


use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::runtime_error::RuntimeError;
use crate::interpreter::resolver::Resolver;
use crate::interpreter::parser::Parser;
use crate::interpreter::scanner::Scanner;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
}

impl Lox {
    /// Creates a new instance of the Lox interpreter.
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
            interpreter: Interpreter::new(),
        }
    }

    /// Entry point for the Lox interpreter.
    pub fn main() {
        let args: Vec<String> = std::env::args().collect();
        let mut lox = Lox::new();

        if args.len() > 2 {
            eprintln!("Usage: lox [script]");
            process::exit(64);
        } else if args.len() == 2 {
            lox.run_file(&args[1]);
        } else {
            lox.run_prompt();
        }
    }

    /// Runs a Lox script from a file.
    pub fn run_file(&mut self, path: &str) {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Failed to read file: {}", err);
                process::exit(74);
            }
        };
        self.run(&content);

        if self.had_error {
            process::exit(65);
        }
        if self.had_runtime_error {
            process::exit(70);
        }
    }

    /// Runs the Lox REPL (interactive prompt).
    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("> ");
            stdout.flush().unwrap();

            let mut input = String::new();
            if stdin.lock().read_line(&mut input).unwrap() == 0 {
                break;
            }

            self.run(&input);
            self.had_error = false; // Reset the error state for the next input
        }
    }

    /// Executes the given Lox source code.
    pub fn run(&mut self, source: &str) {
        // Tokenize the source code
        let mut scanner = Scanner::new(source.to_string());
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("Error scanning tokens: {}", err);
                return;
            }
        };

        // Parse the tokens into statements
        let mut parser = Parser::new(tokens);
        let statements = match parser.parse() {
            Ok(statements) => statements,
            Err(_) => {
                // Errors are already reported in the parser
                return;
            }
        };

        if self.had_error {
            return;
        }

        // Resolve the statements
        let mut resolver = Resolver::new(&mut self.interpreter);
        if let Err(err) = resolver.resolve(&statements) {
            eprintln!("Error resolving statements: {}", err);
            return;
        }

        if self.had_error {
            return;
        }

        // Interpret the statements
        if let Err(runtime_error) = self.interpreter.interpret(&statements) {
            self.runtime_error(runtime_error);
        }
    }

    /// Reports a compile-time error.
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    /// Helper method to format and display error messages.
    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }

    /// Reports a runtime error.
    pub fn runtime_error(&mut self, error: RuntimeError) {
        eprintln!("{}\n[line {}]", error.message, error.token.line);
        self.had_runtime_error = true;
    }
}