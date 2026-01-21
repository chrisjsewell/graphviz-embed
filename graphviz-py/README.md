# graphviz-embed (Python)

**Embedded Graphviz library for Python** — render DOT graphs without external dependencies.

This package provides Python bindings for the [graphviz-embed](https://github.com/chrisjsewell/graphviz-embed) Rust library, allowing you to render DOT graphs using a fully embedded Graphviz library. No external tools or system Graphviz installation required.

## Features

- ✅ **Zero runtime dependencies** for SVG, DOT, JSON, and plain text output
- ✅ **HTML-like labels** supported via bundled Expat library
- ✅ **All layout engines**: dot, neato, fdp, sfdp, circo, twopi, osage, patchwork
- ✅ **Cross-platform**: Linux (x86_64, arm64), macOS (x86_64, arm64), Windows (x86_64)
- ⚡ **Optional Cairo support** for PNG and PDF output

## Installation

```bash
pip install graphviz-embed
```

## Quick Start

```python
from graphviz_embed import GraphvizContext

# Create a Graphviz context
ctx = GraphvizContext()

# Define your graph in DOT language
dot_source = """
    digraph G {
        rankdir=LR;
        a [label="Start"];
        b [label="Process"];
        c [label="End"];
        a -> b -> c;
    }
"""

# Render to SVG
svg = ctx.render(dot_source, "dot", "svg")
print(svg.decode('utf-8'))
```

## Convenience Functions

For quick rendering, use the convenience functions:

```python
from graphviz_embed import render_svg

svg = render_svg("digraph { a -> b -> c }")
print(svg.decode('utf-8'))
```

## Layout Engines

|   Layout    |       Description        |             Best For              |
| ----------- | ------------------------ | --------------------------------- |
| `dot`       | Hierarchical layout      | Directed graphs, flowcharts       |
| `neato`     | Spring model layout      | Undirected graphs, small networks |
| `fdp`       | Force-directed placement | Larger undirected graphs          |
| `sfdp`      | Scalable force-directed  | Very large graphs                 |
| `circo`     | Circular layout          | Cyclic structures                 |
| `twopi`     | Radial layout            | Trees, hierarchies                |
| `osage`     | Clustered layout         | Grouped components                |
| `patchwork` | Treemap layout           | Hierarchical data visualization   |

## Output Formats

### Always Available

- `svg` - SVG vector graphics
- `dot` - DOT with layout info
- `xdot` - Extended DOT format
- `json` - JSON output
- `plain` - Plain text coordinates
- `plain-ext` - Extended plain text
- `canon` - Canonical DOT

### With Cairo (optional)

- `png` - PNG raster image
- `pdf` - PDF document
- `ps` - PostScript
- `eps` - Encapsulated PostScript

## HTML Labels

HTML-like labels are supported for rich formatting:

```python
from graphviz_embed import GraphvizContext

ctx = GraphvizContext()

dot = """
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
"""

svg = ctx.render(dot, "dot", "svg")
```

## Saving to File

```python
from graphviz_embed import GraphvizContext

ctx = GraphvizContext()
ctx.render_to_file(
    "digraph { a -> b }", 
    "dot", 
    "svg", 
    "output.svg"
)
```

## API Reference

### `GraphvizContext`

The main class for rendering DOT graphs.

**Methods:**

- `__init__()` - Create a new Graphviz context
- `render(dot_source: str, layout: str, format: str) -> bytes` - Render DOT source to bytes
- `render_to_file(dot_source: str, layout: str, format: str, path: str) -> None` - Render to file

### Convenience Functions

- `render_svg(dot_source: str, layout: str = "dot") -> bytes` - Quick SVG rendering
- `render_png(dot_source: str, layout: str = "dot") -> bytes` - Quick PNG rendering (requires Cairo)

### Constants

Layout algorithms:
- `LAYOUT_DOT`, `LAYOUT_NEATO`, `LAYOUT_FDP`, `LAYOUT_SFDP`
- `LAYOUT_CIRCO`, `LAYOUT_TWOPI`, `LAYOUT_OSAGE`, `LAYOUT_PATCHWORK`

Output formats:
- `FORMAT_SVG`, `FORMAT_DOT`, `FORMAT_XDOT`, `FORMAT_JSON`
- `FORMAT_PLAIN`, `FORMAT_PLAIN_EXT`, `FORMAT_CANON`
- `FORMAT_PNG`, `FORMAT_PDF`, `FORMAT_PS`, `FORMAT_EPS` (Cairo only)

Feature flags:
- `HAS_CAIRO` - Boolean indicating if Cairo support is available

## Requirements

### Build-time

When installing from source (not required for pre-built wheels):

- Python 3.8+
- Rust 1.70+
- CMake 3.16+
- C compiler (GCC, Clang, or MSVC)

### Runtime

**Default (no Cairo):** None! Graphviz and Expat are statically linked into the binary.

**With Cairo:** Requires Cairo and Pango shared libraries. Install via:
- **Linux**: `apt install libcairo2 libpango-1.0-0`
- **macOS**: `brew install cairo pango`
- **Windows**: Cairo/Pango DLLs must be in PATH

## License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

### Third-Party Licenses

- **Graphviz**: Eclipse Public License 1.0 (EPL-1.0)
- **Expat**: MIT License

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/chrisjsewell/graphviz-embed) for guidelines.

## Links

- [Source Code](https://github.com/chrisjsewell/graphviz-embed)
- [Issue Tracker](https://github.com/chrisjsewell/graphviz-embed/issues)
- [Rust Crate](https://crates.io/crates/graphviz-embed)
