[![Tests](https://github.com/Kkobarii/Rusty-Tape/actions/workflows/tests.yml/badge.svg)](https://github.com/Kkobarii/Rusty-Tape/actions/workflows/tests.yml)

# Rusty Tape

Rusty Tape is a Rust-based RAM (Random Access Machine) simulator that provides a simple architecture for parsing and executing low-level instructions. The project includes a parser for `.ram` files, a simulation engine and pretty good test coverage. In time, there will be a console GUI written in Ratatui, which will allow the user to simulate the steps the RAM takes.

## Features

- **RAM Machine Execution**: Simulate a basic RAM machine with registers and instructions.
- **Custom Parser**: Parse `.ram` files with support for comments, labels, and various operations.
- **Extensive Test Suite**: Ensure correctness of parsing and execution using continually deployed tests.

## Usage

### Code API
1. Create a `.ram` file with your program instructions.
2. Use the `Parser::parse_file("your_file.ram")` to load the program.
3. Execute the program using the `RamMachine` interface.

Here’s an example program:

```
R0 := 0  # Initialize sum
R1 := 2  # Increment
R2 := 5  # Counter
start: if (R2 <= 0) goto end
       R0 := R0 + R1
       R2 := R2 - 1
       goto start
end:   halt
```

You can parse and run this program using the following API:

```rust
let machine = Parser::parse_file("examples/sample.ram").unwrap();
machine.run();
```

### Running Tests
Tests are automatically run on every push to the main branch. To run them locally use:

```bash
cargo test
```
