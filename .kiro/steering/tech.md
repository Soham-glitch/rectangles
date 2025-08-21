# Technology Stack

## Language & Edition

- **Rust** (2024 edition)
- No external dependencies - uses only Rust standard library

## Build System

- **Cargo** - Rust's built-in package manager and build system

## Common Commands

### Building

```bash
cargo build          # Build in debug mode
cargo build --release # Build optimized release version
```

### Running

```bash
cargo run            # Compile and run the project
```

### Testing

```bash
cargo test           # Run unit tests
cargo test -- --nocapture # Run tests with output
```

### Code Quality

```bash
cargo check          # Fast syntax and type checking
cargo clippy         # Rust linter for catching common mistakes
cargo fmt            # Format code according to Rust style guidelines
```

## Project Configuration

- Package name: `rectangle`
- Version: `0.1.0`
- No external dependencies currently used
