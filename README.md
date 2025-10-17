# Rust Algorithms Project

A Rust workspace project with a binary application and a separate algorithm library.

## Project Structure

```
rust_algos/
├── Cargo.toml          # Workspace configuration
├── src/
│   └── main.rs         # Main binary application
└── algo_lib/           # Algorithm library (separate crate)
    ├── Cargo.toml
    └── src/
        └── lib.rs      # Library implementation
```

## Features

- **Workspace Setup**: Uses Cargo workspace to manage multiple crates
- **Library Dependency**: `algo_lib` is a separate library crate that the main binary depends on
- **Modular Design**: Algorithms are implemented in the library and can be reused across projects

## Usage

### Build the project
```bash
cargo build
```

### Run the application
```bash
cargo run
```

### Run tests
```bash
# Run all tests
cargo test

# Run only algo_lib tests
cargo test --package algo_lib
```

### Run submission
```bash
cargo equip --bin rust_algos --remove docs --remove comments > sol.rs
```

## Adding New Algorithms

Add new functions to `algo_lib/src/lib.rs`:

```rust
pub fn your_algorithm(param: Type) -> ReturnType {
    // implementation
}
```

Then use them in `src/main.rs`:

```rust
use algo_lib;

fn main() {
    let result = algo_lib::your_algorithm(value);
}
```
