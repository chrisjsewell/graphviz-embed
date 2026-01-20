# Contributing to graphviz-embed

Thank you for your interest in contributing to graphviz-embed! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.70+ (stable)
- CMake 3.16+
- C compiler (GCC, Clang, or MSVC)
- Git (with submodule support)

### Setting Up the Development Environment

1. Clone the repository with submodules:
   ```bash
   git clone --recursive https://github.com/chrisjsewell/graphviz-embed.git
   cd graphviz-embed
   ```

2. If you already cloned without `--recursive`:
   ```bash
   git submodule update --init --recursive
   ```

3. Build and test:
   ```bash
   cargo build
   cargo test
   ```

## How to Contribute

### Reporting Bugs

- Check existing issues first to avoid duplicates
- Include your OS, Rust version, and any relevant error messages
- Provide a minimal reproducible example if possible

### Suggesting Features

- Open an issue describing the feature and its use case
- Check the [Future Improvements](README.md#future-improvements) section to see if it's already planned

### Submitting Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run lints: `cargo fmt && cargo clippy`
6. Commit with a clear message
7. Push and open a PR

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Address all Clippy warnings (`cargo clippy`)
- Add doc comments for public APIs
- Include tests for new functionality

## Project Structure

```
graphviz-embed/
├── graphviz-embed/     # Safe Rust API crate
│   ├── src/lib.rs      # Public API
│   └── tests/          # Integration tests
├── graphviz-sys/       # FFI bindings crate
│   ├── build.rs        # CMake build orchestration
│   └── src/lib.rs      # Raw FFI declarations
├── vendor/
│   ├── graphviz/       # Graphviz submodule (v14.1.1)
│   └── expat/          # Expat submodule (v2.7.1)
├── generated/          # Pre-generated Bison/Flex parsers
└── .github/workflows/  # CI configuration
```

## Working with Vendored Dependencies

### Graphviz Submodule

The Graphviz source is pinned to version 14.1.1. Do not modify files in `vendor/graphviz/` directly — changes will be lost.

### Regenerating Parser Files

If you need to update the pre-generated parser files (e.g., after a Graphviz version bump):

1. Install Bison ≥3.0 and Flex:
   ```bash
   # macOS
   brew install bison flex
   
   # Linux
   apt install bison flex
   ```

2. Run the generation script:
   ```bash
   ./scripts/generate-parsers.sh
   ```

3. Commit the updated files in `generated/`

### Updating Graphviz Version

1. Update the submodule:
   ```bash
   cd vendor/graphviz
   git fetch origin
   git checkout <new-tag>
   cd ../..
   ```

2. Regenerate parser files (see above)

3. Test the build on all platforms

4. Update version references in documentation

## Testing

### Running Tests

```bash
# All tests
cargo test

# With Cairo feature
cargo test --features cairo

# Specific test
cargo test test_html_labels
```

### Test Categories

- **Unit tests**: In `graphviz-sys/src/lib.rs` and `graphviz-embed/src/lib.rs`
- **Integration tests**: In `graphviz-embed/tests/integration.rs`
- **Doc tests**: In documentation examples

## Build System

The build is orchestrated by `graphviz-sys/build.rs`:

1. Copies vendored sources to `OUT_DIR`
2. Copies pre-generated parser files
3. Patches CMakeLists.txt files inline
4. Builds Expat via CMake
5. Builds Graphviz via CMake
6. Emits linker directives

If you're debugging build issues, set `CARGO_BUILD_JOBS=1` for sequential output.

## Questions?

- Open an issue for questions about the codebase
- Check [AGENTS.md](AGENTS.md) for detailed technical documentation
- Review existing issues and PRs for context

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).
