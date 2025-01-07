# lox-in-rust

This repository contains a Rust-based implementation of the *Crafting Interpreters* book by Robert Nystrom. The goal is to translate the book's code and concepts into idiomatic Rust as an educational exercise.

---

## Features
- Lexer and parser for the Lox language
- Tree-walk interpreter (**Phase 1**)
- Bytecode interpreter (**Phase 2**)

---

## Project Structure

The project is divided into two phases:
1. **Tree-Walk Interpreter** (based on the book's Java implementation)
2. **Bytecode Virtual Machine** (based on the book's C implementation)

---

### Directory Layout
```plaintext
lox-in-rust/
├── src/
│   ├── interpreter/        # Phase 1: Tree-walk interpreter modules
│   │   ├── ast.rs          # Abstract Syntax Tree (AST) definitions for expressions
│   │   ├── ast_printer.rs  # Debugging utility to print ASTs
│   │   ├── environment.rs  # Manages variable scope and state
│   │   ├── interpreter.rs  # Executes AST in a tree-walk fashion
│   │   ├── lox.rs          # Main entry point for the interpreter phase
│   │   ├── parser.rs       # Converts tokens into an AST
│   │   ├── scanner.rs      # Tokenizes source code into tokens
│   │   ├── stmt.rs         # AST definitions for statements
│   │   ├── token.rs        # Token representation
│   │   ├── token_type.rs   # Enum for token types
│   │   └── mod.rs          # Exports interpreter modules
│   ├── vm/                 # Phase 2: Bytecode VM modules
│   │   ├── chunk.rs        # Manages sequences of bytecode instructions
│   │   ├── vm.rs           # Core VM for executing bytecode
│   │   ├── compiler.rs     # Translates source code into bytecode
│   │   ├── memory.rs       # Manages memory allocation
│   │   ├── scanner.rs      # Tokenizes source code for the compiler phase
│   │   ├── value.rs        # Represents Lox values (e.g., numbers, strings)
│   │   ├── table.rs        # Implements hash tables for variable storage
│   │   └── mod.rs          # Exports VM modules
│   └── main.rs             # Main entry point for the entire project
├── Cargo.toml              # Rust project configuration
├── LICENSE                 # License file (MIT)
└── README.md               # Project documentation

---

## How to Run

### Prerequisites

1. Install Rust using rustup:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh

2. Clone this repository:
git clone https://github.com/sergiogallegos/lox-in-rust.git
cd lox-in-rust

### Steps to Run the Project

1. Build the project:
cargo build

2. Run the interpreter phase:
cargo run

By default, this will run the tree-walk interpreter. To switch to the bycode VM, update the main entry point in main.rs to:
vm::vm::run();

3. Run tests:
cargo test



## License
This project is licensed under the MIT License. It is inspired by the *Crafting Interpreters* book, and the original ideas belong to Robert Nystrom.
