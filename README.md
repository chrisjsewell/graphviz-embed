# graphviz-embed

**Embedded Graphviz library for Rust** — render DOT graphs without external dependencies.

[![Crates.io](https://img.shields.io/crates/v/graphviz-embed.svg)](https://crates.io/crates/graphviz-embed)
[![Documentation](https://docs.rs/graphviz-embed/badge.svg)](https://docs.rs/graphviz-embed)
[![CI](https://github.com/chrisjsewell/graphviz-embed/workflows/CI/badge.svg)](https://github.com/chrisjsewell/graphviz-embed/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

This crate provides a safe Rust API to render DOT graphs using a fully embedded Graphviz library. No external tools or system Graphviz installation required.

## Features

- ✅ **Zero runtime dependencies** for SVG, DOT, JSON, and plain text output
- ✅ **HTML-like labels** supported via bundled Expat library
- ✅ **All layout engines**: dot, neato, fdp, sfdp, circo, twopi, osage, patchwork
- ✅ **Cross-platform**: Linux (x86_64, arm64), macOS (x86_64, arm64), Windows (x86_64)
- ⚡ **Optional Cairo support** for PNG and PDF output

## Requirements

### Build-time

| Dependency | Required | Notes |
|------------|----------|-------|
| Rust 1.70+ | ✅ | Edition 2021 |
| CMake 3.16+ | ✅ | Builds vendored C libraries |
| C compiler | ✅ | GCC, Clang, or MSVC |

```bash
# Linux (Debian/Ubuntu)
apt install cmake build-essential

# macOS (Xcode command line tools provide the C compiler)
brew install cmake

# Windows
# Install Visual Studio with "Desktop development with C++" workload
# CMake is included, or install separately from cmake.org
```

### Runtime

**None!** All libraries are statically linked into your binary.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
graphviz-embed = "0.1"
```

For PNG/PDF output, enable the `cairo` feature:

```toml
[dependencies]
graphviz-embed = { version = "0.1", features = ["cairo"] }
```

> **Note:** The `cairo` feature requires Cairo and Pango libraries installed on your system:
> - **Linux**: `apt install libcairo2-dev libpango1.0-dev`
> - **macOS**: `brew install cairo pango`
> - **Windows**: Install via vcpkg (`vcpkg install cairo pango`)

## Quick Start

```rust
use graphviz_embed::{GraphvizContext, Layout, Format};

fn main() -> Result<(), graphviz_embed::Error> {
    // Create a Graphviz context
    let ctx = GraphvizContext::new()?;

    // Define your graph in DOT language
    let dot_source = r#"
        digraph G {
            rankdir=LR;
            a [label="Start"];
            b [label="Process"];
            c [label="End"];
            a -> b -> c;
        }
    "#;

    // Render to SVG
    let svg = ctx.render(dot_source, Layout::Dot, Format::Svg)?;
    println!("{}", String::from_utf8_lossy(&svg));

    Ok(())
}
```

## Layout Engines

| Layout | Description | Best For |
|--------|-------------|----------|
| `Dot` | Hierarchical layout | Directed graphs, flowcharts |
| `Neato` | Spring model layout | Undirected graphs, small networks |
| `Fdp` | Force-directed placement | Larger undirected graphs |
| `Sfdp` | Scalable force-directed | Very large graphs |
| `Circo` | Circular layout | Cyclic structures |
| `Twopi` | Radial layout | Trees, hierarchies |
| `Osage` | Clustered layout | Grouped components |
| `Patchwork` | Treemap layout | Hierarchical data visualization |

## Output Formats

### Always Available

| Format | Description |
|--------|-------------|
| `Svg` | SVG vector graphics |
| `Dot` | DOT with layout info |
| `Xdot` | Extended DOT format |
| `Json` | JSON output |
| `Plain` | Plain text coordinates |
| `PlainExt` | Extended plain text |
| `Canon` | Canonical DOT |

### With `cairo` Feature

| Format | Description |
|--------|-------------|
| `Png` | PNG raster image |
| `Pdf` | PDF document |
| `Ps` | PostScript |
| `Eps` | Encapsulated PostScript |

## HTML Labels

HTML-like labels are supported for rich formatting:

```rust
use graphviz_embed::{GraphvizContext, Layout, Format};

let ctx = GraphvizContext::new().unwrap();

let dot = r#"
    digraph G {
        node [shape=none];
        table [label=<
            <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                <TR><TD BGCOLOR="lightblue"><B>Header</B></TD></TR>
                <TR><TD>Row 1</TD></TR>
                <TR><TD>Row 2</TD></TR>
            </TABLE>
        >];
    }
"#;

let svg = ctx.render(dot, Layout::Dot, Format::Svg).unwrap();
```

## Supported Platforms

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64 | ✅ |
| Linux | aarch64 | ✅ |
| macOS | x86_64 (Intel) | ✅ |
| macOS | aarch64 (Apple Silicon) | ✅ |
| Windows | x86_64 (MSVC) | ✅ |

## Design Considerations

This crate was designed with several key goals in mind:

### Why Vendor Graphviz?

Most existing Rust Graphviz crates either:
1. Call the external `dot` command (requires system installation)
2. Link to system Graphviz libraries (requires headers and libs)

By vendoring Graphviz source code, this crate provides a true "just works" experience — add the dependency and start rendering graphs.

### Pre-generated Parser Files

Graphviz uses Bison and Flex to generate parsers for DOT syntax and HTML labels. Rather than requiring these tools at build time, we pre-generate the parser files and include them in the repository. This means:

- No Bison/Flex required to build
- Simpler CI/CD pipelines
- Works on Windows without MSYS2

### Static Plugin Linking

Graphviz normally uses `ltdl` (libtool's dlopen wrapper) to load rendering plugins dynamically. For an embedded library, we:

1. Disable ltdl entirely
2. Statically link all layout engines and renderers
3. Register plugins programmatically at initialization

This ensures the library is self-contained with no runtime plugin discovery.

### Bundled Expat

HTML-like labels (using `<TABLE>`, `<B>`, etc.) require XML parsing. Rather than making this optional or requiring system Expat, we bundle Expat 2.7.3 and always enable HTML label support.

### Optional Cairo

Cairo/Pango support for raster output (PNG, PDF) is optional because:

1. These libraries have many transitive dependencies
2. SVG output covers most use cases
3. System Cairo installation varies by platform

Enable the `cairo` feature when you need raster output.

## Build Challenges & Solutions

During development, several challenges were encountered that may be useful for contributors or anyone attempting similar projects:

### Parser Generation

**Problem**: Graphviz requires Bison ≥3.0 and Flex at build time to generate parsers for DOT syntax (`lib/cgraph`), HTML labels (`lib/common`), and expressions (`lib/expr`).

**Solution**: Pre-generate parser files and patch CMakeLists.txt to use them instead of invoking Bison/Flex. The build script applies inline patches to:
- Replace `BISON_TARGET()` and `FLEX_TARGET()` with static file references
- Comment out `set_package_properties(BISON/FLEX PROPERTIES TYPE REQUIRED)`

### CMake Configuration Complexity

**Problem**: Graphviz's CMake configuration has many optional dependencies (GTS, LTDL, etc.) that can cause build failures when not available.

**Solution**: The build script patches CMakeLists.txt to:
- Disable GTS (GNU Triangulated Surface) which requires glib
- Disable LTDL (dynamic plugin loading) for static linking
- Disable ipsepcola which requires C++ stdlib headers that may not be available in cross-compilation

### Static Plugin Linking

**Problem**: Graphviz expects plugins to be loaded dynamically via ltdl. Static linking requires manual plugin registration.

**Solution**: 
- Build plugins as static libraries in specific build directories
- Export plugin symbols (`gvplugin_*_LTX_library`) 
- Register plugins programmatically via `gvAddLibrary()` at initialization
- Add comprehensive library search paths in build.rs for plugin locations

### Thread Safety

**Problem**: The Graphviz C library uses global state and is not thread-safe. Running tests in parallel caused SIGSEGV crashes.

**Solution**: Implement a global mutex (`GRAPHVIZ_MUTEX`) that serializes all Graphviz operations:
- Context creation
- Graph parsing and rendering  
- Context destruction

### Library Naming and Paths

**Problem**: CMake places built libraries in various subdirectories, and library names can differ between platforms.

**Solution**: The build script searches multiple paths:
- `lib/` and `lib64/` in install directory
- `build/plugin/{dot_layout,neato_layout,core}/` for plugins
- `build/lib/{dotgen,neatogen,fdpgen,...}/` for internal libraries

## Platform-Specific Considerations

### Linux

- **x86_64**: Works out of the box with GCC or Clang
- **aarch64**: Requires cross-compilation toolchain (`aarch64-linux-gnu-gcc`) when building from x86_64 host
- **Dependencies**: Only `cmake` and a C compiler required; `libcairo2-dev` and `libpango1.0-dev` for Cairo feature

### macOS

- **Apple Silicon (M1/M2/M3)**: Set `CMAKE_OSX_ARCHITECTURES=arm64`
- **Intel**: Set `CMAKE_OSX_ARCHITECTURES=x86_64`
- **Homebrew Bison**: System bison is too old (2.3); if regenerating parsers, use `/opt/homebrew/opt/bison/bin/bison`
- **Dependencies**: `brew install cmake` required; `brew install cairo pango` for Cairo feature

### Windows

- **MSVC**: Builds use dynamic CRT (`/MD`, `/MDd`) by default to match Rust's defaults
- **Static CRT**: For standalone binaries without MSVC runtime dependencies, use:
  ```bash
  RUSTFLAGS='-C target-feature=+crt-static' cargo build --release
  ```
- **MinGW**: Not currently tested; may require additional configuration
- **Cairo**: Install via vcpkg: `vcpkg install cairo pango`
- **vcpkg Integration**: Set `VCPKG_ROOT` environment variable

### Cross-Compilation

When cross-compiling (e.g., building Linux ARM64 from x86_64):
1. Install target toolchain
2. Build script auto-detects cross-compilation when `HOST != TARGET`
3. Sets appropriate CMake cross-compilation variables

## Known Limitations

1. **No dynamic plugin loading**: All plugins must be statically linked at compile time
2. **No GVPR support**: The graph pattern scanning language is not exposed in the API
3. **No incremental layout**: Each render call creates a new graph; no persistent graph editing
4. **Single-threaded rendering**: Operations are serialized via mutex for safety
5. **Shared global context**: All `GraphvizContext` instances share the same underlying C context. This is intentional — the Graphviz C library has issues with repeated context creation/destruction on some platforms (notably macOS Intel). This has no impact on functionality; creating multiple `GraphvizContext` instances is safe and has negligible overhead.
6. **Linux release mode tests**: Due to a limitation with the rust-lld linker not properly handling Graphviz's complex static library dependencies, testing this crate itself requires debug mode on Linux (`cargo test` instead of `cargo test --release`). This **does not affect** downstream packages that depend on graphviz-embed — they can use release builds and `cargo test --release` without any issues. The limitation only applies to this crate's own test suite.

## Future Improvements

### Short Term

- [ ] Add `sfdp`, `osage`, and `patchwork` layout tests (currently only basic layouts tested)
- [ ] Expose more Graphviz attributes in the Rust API
- [ ] Add graph introspection API (node/edge enumeration)
- [ ] Support rendering to `std::io::Write` instead of just `Vec<u8>`

### Medium Term

- [ ] Add WebAssembly (WASM) support for browser-based rendering
- [ ] Implement graph builder API for programmatic graph construction
- [ ] Add async rendering support
- [ ] Support custom fonts and font paths

### Long Term

- [ ] Expose GVPR for graph transformations
- [ ] Support incremental layout updates
- [ ] Add GPU-accelerated rendering via wgpu
- [ ] Consider bindgen for automatic FFI generation

### Contributions Welcome

If you're interested in any of these improvements, contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Building from Source

### Prerequisites

- Rust 1.70+ (for edition 2021)
- CMake 3.16+
- C/C++ compiler (GCC, Clang, or MSVC)

### Clone and Build

```bash
git clone --recursive https://github.com/chrisjsewell/graphviz-embed.git
cd graphviz-embed
cargo build --release
```

### Running Tests

```bash
cargo test
```

With Cairo:

```bash
cargo test --features cairo
```

## License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

### Third-Party Licenses

- **Graphviz**: Eclipse Public License 1.0 (EPL-1.0)
- **Expat**: MIT License

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

For AI agents and automated tools, see [AGENTS.md](AGENTS.md) for codebase documentation.

## Releasing (Maintainers)

Releases are automated via GitHub Actions. When a version tag is pushed, the CI will publish to crates.io after all tests pass.

### Setup (one-time)

1. Create an API token at [crates.io/settings/tokens](https://crates.io/settings/tokens)
2. Add it as a GitHub secret named `CRATES_IO_TOKEN` in **Settings → Secrets and variables → Actions**

### Release Process

1. Update version in `Cargo.toml` (workspace version)
2. Update `CHANGELOG.md` if applicable
3. Commit the changes:
   ```bash
   git add -A
   git commit -m "Release v0.1.0"
   git push origin main
   ```
4. Create a release on GitHub:
   - Go to **Releases → Draft a new release**
   - Create a new tag (e.g., `v0.1.0`)
   - Add release notes
   - Publish the release

Alternatively, push a tag directly:
```bash
git tag v0.1.0
git push origin v0.1.0
```

The CI will automatically:
- Run all tests on all platforms
- Verify the tag version matches `Cargo.toml`
- Publish `graphviz-sys` to crates.io
- Publish `graphviz-embed` to crates.io

## Acknowledgments

- [Graphviz](https://graphviz.org/) — the amazing graph visualization software
- [Expat](https://libexpat.github.io/) — fast XML parsing library
- [Cairo](https://www.cairographics.org/) — 2D graphics library (optional)
