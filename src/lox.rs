mod interpreter;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run(&mut self, source: &str) {
        // Use the scanner module from interpreter
        let mut scanner = interpreter::scanner::Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        if self.had_error {
            return;
        }

        // Use the parser module
        let mut parser = interpreter::parser::Parser::new(tokens);
        let statements = parser.parse();

        if self.had_error {
            return;
        }

        // Use the resolver module
        let mut resolver = interpreter::resolver::Resolver::new();
        resolver.resolve(&statements);

        if self.had_error {
            return;
        }

        // Execute the interpreted statements
        if let Err(runtime_error) = interpreter::interpreter::Interpreter::new().interpret(&statements) {
            self.runtime_error(runtime_error);
        }
    }

    fn runtime_error(&mut self, error: interpreter::runtime_error::RuntimeError) {
        eprintln!("{}\n[line {}]", error.message, error.token.line);
        self.had_runtime_error = true;
    }
}
