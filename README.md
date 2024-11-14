## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# rust_refactor

A library for automated code refactoring in Rust.

## Features

- Extract functions from code snippets.
- Rename variables throughout a codebase.

## Installation

To use `rust_refactor`, add the following to your `Cargo.toml`:

```toml
[dependencies]
rust_refactor = "0.1.0"
```
## Usage
```rust
use rust_refactor::refactor;

fn main() {
    let code = "let x = 5; x + 1";
    
    let extracted = refactor::extract_function(code, "my_func");
    println!("{}", extracted);
    
    let renamed = refactor::rename_variable(code, "x", "y");
    println!("{}", renamed);
}
```
## License
This project is licensed under the MIT License


## Running the Project

1. **Build the project**:
   ```sh
   cargo build
   ```
## Author
Ben Santora 
