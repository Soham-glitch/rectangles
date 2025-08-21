# Project Structure

## Directory Layout

```
rectangle/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Dependency lock file (auto-generated)
├── .gitignore          # Git ignore rules (excludes /target)
├── src/
│   └── main.rs         # Main application entry point
└── target/             # Build artifacts (ignored by git)
```

## Code Organization

### Main Module (`src/main.rs`)

- Contains the `Rectangle` struct definition
- Implements all rectangle-related methods
- Houses the main function with example usage

### Struct Design Patterns

- Use `#[derive(Debug)]` for structs to enable debug printing
- Implement methods using `impl` blocks
- Use associated functions (like `square`) for constructor-like behavior
- Follow Rust naming conventions: snake_case for functions, PascalCase for types

### Method Conventions

- Use `&self` for methods that read data
- Use `&mut self` for methods that modify data
- Use associated functions (no self) for constructors
- Return owned values when creating new instances

## File Naming

- Use snake_case for file names
- Main entry point is always `src/main.rs` for binary crates
- Additional modules would go in `src/` directory
