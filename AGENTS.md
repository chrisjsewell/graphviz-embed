# AGENTS.md — AI Agent Guidelines

This document provides essential information for AI agents (Claude, GPT, Copilot, etc.)
working with the graphviz-embed codebase.

## Project Overview

**graphviz-embed** is a Rust crate that embeds the Graphviz C library to render DOT
graphs without requiring external tool installation. It vendors Graphviz 14.1.1 and
Expat 2.7.3 source code and compiles them as part of the Rust build process.

## Crate Structure

```
graphviz-embed/
├── Cargo.toml                  # Workspace manifest
├── README.md                   # User documentation
├── AGENTS.md                   # This file
├── graphviz-embed/             # Safe Rust API crate
│   ├── Cargo.toml
│   ├── src/lib.rs              # Public API: GraphvizContext, Layout, Format
│   └── tests/
│       └── integration.rs      # Integration tests
├── graphviz-sys/               # FFI bindings crate
│   ├── Cargo.toml
│   ├── build.rs                # CMake build orchestration
│   └── src/lib.rs              # Raw FFI declarations
├── vendor/
│   ├── graphviz/               # Git submodule @ tag 14.1.1
│   └── expat/                  # Git submodule @ tag R_2_7_3
├── generated/                  # Pre-generated Bison/Flex outputs
│   ├── cgraph/                 # DOT parser: grammar.c, grammar.h, scan.c
│   ├── common/                 # HTML parser: htmlparse.c, htmlparse.h
│   └── expr/                   # Expression parser: exparse.c, exparse.h
├── patches/
│   └── 0001-use-pregenerated-parsers.patch
└── .github/workflows/ci.yml    # GitHub Actions CI
```

## Key Components

### graphviz-embed (Safe API)

Located in `graphviz-embed/src/lib.rs`. Provides:

- `GraphvizContext` — Main entry point for rendering
- `Layout` enum — Layout algorithms (Dot, Neato, Fdp, etc.)
- `Format` enum — Output formats (Svg, Png with cairo feature, etc.)
- `Error` type — Error handling
- `render()` method — Core rendering function

### graphviz-sys (FFI Layer)

Located in `graphviz-sys/src/lib.rs`. Provides:

- Raw FFI declarations for Graphviz C API
- `gv_init()` — Initialize with static plugins
- Plugin library symbols for static linking

### build.rs (Build Orchestration)

Located in `graphviz-sys/build.rs`. Handles:

1. Copying vendored sources to OUT_DIR
2. Copying pre-generated parser files
3. Applying CMake patches
4. Building Expat library
5. Building Graphviz library
6. Emitting linker directives
7. Handling Cairo feature flag

## Build System

The build process uses CMake via the `cmake` crate:

1. **Expat Build**: Standalone CMake build with minimal options
2. **Graphviz Build**: CMake with Expat paths and disabled features
3. **Static Linking**: All libraries linked statically

### Key CMake Flags

```cmake
-DBUILD_SHARED_LIBS=OFF          # Static libraries only
-DGRAPHVIZ_CLI=OFF               # No command-line tools
-Denable_ltdl=OFF                # Disable plugin loading
-DWITH_EXPAT=ON                  # Enable HTML labels
-DEXPAT_INCLUDE_DIR=...          # Point to built Expat
-DEXPAT_LIBRARY=...              # Point to built Expat
```

### Cross-Platform Support

The build.rs handles platform differences:

- **macOS**: Sets `CMAKE_OSX_ARCHITECTURES` for arm64/x86_64
- **Linux ARM64**: Sets cross-compiler when `host != target`
- **Windows MSVC**: Enables static CRT linking

## Feature Flags

| Feature | Effect |
|---------|--------|
| `default` | SVG, DOT, JSON output only |
| `cairo` | Enables PNG, PDF output via Cairo/Pango |

The `cairo` feature:
1. Uses pkg-config (Linux/macOS) or vcpkg (Windows) to find Cairo
2. Enables Pango plugin in Graphviz build
3. Adds `has_cairo` cfg flag for conditional compilation

## Making Changes

### Adding a New Output Format

1. Add variant to `Format` enum in `graphviz-embed/src/lib.rs`
2. Update `Format::as_str()` to return Graphviz format name
3. If Cairo-dependent, gate with `#[cfg(has_cairo)]`
4. Add test in `tests/integration.rs`

### Adding a New Layout Engine

1. Add variant to `Layout` enum
2. Update `Layout::as_str()`
3. Ensure plugin is registered in `graphviz-sys/src/lib.rs` `gv_init()`
4. Add library to link list in `build.rs` if needed
5. Add test

### Updating Graphviz Version

1. Update submodule: `cd vendor/graphviz && git fetch && git checkout <new-tag>`
2. Regenerate parser files (see [generated/README.md](generated/README.md))
3. Update patch if CMakeLists.txt changed
4. Update version in [generated/README.md](generated/README.md)
5. Run full test suite
6. Update README.md if API changed

### Updating Expat Version

1. Update submodule: `cd vendor/expat && git fetch && git checkout <new-tag>`
2. Test build on all platforms
3. Update version references in documentation

### Regenerating Parser Files

Requires Bison ≥3.0 and Flex:

```bash
# cgraph parser
cd vendor/graphviz/lib/cgraph
bison -d -o grammar.c grammar.y
flex --case-insensitive -o scan.c scan.l
cp grammar.c grammar.h scan.c ../../../generated/cgraph/

# HTML parser
cd ../common
bison -d -o htmlparse.c htmlparse.y
cp htmlparse.c htmlparse.h ../../../generated/common/

# Expression parser
cd ../expr
bison --yacc -Wno-yacc --defines --debug --verbose -o exparse.c exparse.y
cp exparse.c exparse.h ../../../generated/expr/
```

### Updating Patches

If Graphviz CMakeLists.txt changes:

1. Create fresh clone of Graphviz at new version
2. Apply existing patch to understand required changes
3. Manually update CMakeLists.txt files
4. Generate new patch: `git diff > patches/0001-use-pregenerated-parsers.patch`

## Testing

### Unit Tests

```bash
cargo test -p graphviz-sys
cargo test -p graphviz-embed
```

### Integration Tests

```bash
cargo test --test integration
```

### With Cairo Feature

```bash
cargo test --features cairo
```

### Test Categories

| Test | Purpose |
|------|---------|
| `test_render_simple_svg` | Basic rendering works |
| `test_layout_*` | Each layout engine works |
| `test_html_labels` | HTML label parsing works |
| `test_invalid_dot_error` | Error handling works |
| `test_render_png` | Cairo rendering works (feature-gated) |

## Common Issues

### Build Fails: "CMake not found"

Install CMake:
- macOS: `brew install cmake`
- Ubuntu: `apt install cmake`
- Windows: Download from cmake.org

### Build Fails: "Bison/Flex not found"

This shouldn't happen with pre-generated files. If it does:
1. Ensure `generated/` directory contains `.c` and `.h` files
2. Ensure patch is being applied (check build output)

### Build Fails: "undefined reference to gvplugin_*"

Plugin not being linked. Check:
1. Plugin library in `emit_link_directives()` in build.rs
2. Plugin registered in `gv_init()` in graphviz-sys/src/lib.rs

### Build Fails on Windows: "cairo not found"

Install via vcpkg:
```powershell
vcpkg install cairo pango
```

Set environment variable:
```powershell
$env:VCPKG_ROOT = "C:\path\to\vcpkg"
```

### Rendering Produces Empty Output

1. Check DOT syntax is valid
2. Ensure layout was applied (check for error returns)
3. For raster formats, ensure `cairo` feature is enabled

## Code Style

- Follow rustfmt defaults
- Use `cargo clippy` before committing
- Document public APIs with doc comments
- Use `thiserror` for error types
- Prefer returning `Result` over panicking

## Unsafe Code Guidelines

The `graphviz-sys` crate contains unsafe code for FFI:

1. All unsafe blocks should have safety comments
2. Null pointer checks before dereferencing
3. Proper lifetime management for C allocations
4. Use `gvFreeRenderData()` for data from `gvRenderData()`
5. Always call `agclose()` on graphs
6. Always call `gvFreeContext()` on context

## CI/CD

GitHub Actions runs on every push/PR:

- Matrix build: Linux (x86_64, arm64), macOS (x86_64, arm64), Windows
- Tests with and without `cairo` feature
- Cairo deps installed on runners

## Contact

For questions about the codebase:
1. Check existing issues on GitHub
2. Read the inline code comments
3. Consult Graphviz documentation: https://graphviz.org/documentation/
