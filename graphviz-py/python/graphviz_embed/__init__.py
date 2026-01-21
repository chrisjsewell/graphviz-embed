"""
graphviz-embed: Embedded Graphviz library for Python

This package provides Python bindings for the graphviz-embed Rust library,
allowing you to render DOT graphs without external dependencies.

Example:
    >>> from graphviz_embed import GraphvizContext
    >>> ctx = GraphvizContext()
    >>> svg = ctx.render("digraph { a -> b }", "dot", "svg")
    >>> print(svg.decode('utf-8'))

Quick rendering:
    >>> from graphviz_embed import render_svg
    >>> svg = render_svg("digraph { a -> b }")
"""

from ._graphviz_py import (
    GraphvizContext,
    render_svg,
    render_png,
    __version__,
    # Layout algorithms
    LAYOUT_DOT,
    LAYOUT_NEATO,
    LAYOUT_FDP,
    LAYOUT_SFDP,
    LAYOUT_CIRCO,
    LAYOUT_TWOPI,
    LAYOUT_OSAGE,
    LAYOUT_PATCHWORK,
    # Output formats
    FORMAT_SVG,
    FORMAT_DOT,
    FORMAT_XDOT,
    FORMAT_JSON,
    FORMAT_PLAIN,
    FORMAT_PLAIN_EXT,
    FORMAT_CANON,
    HAS_CAIRO,
)

# Conditionally import Cairo formats if available
if HAS_CAIRO:
    from ._graphviz_py import (
        FORMAT_PNG,
        FORMAT_PDF,
        FORMAT_PS,
        FORMAT_EPS,
    )
    __all__ = [
        "GraphvizContext",
        "render_svg",
        "render_png",
        "__version__",
        "LAYOUT_DOT",
        "LAYOUT_NEATO",
        "LAYOUT_FDP",
        "LAYOUT_SFDP",
        "LAYOUT_CIRCO",
        "LAYOUT_TWOPI",
        "LAYOUT_OSAGE",
        "LAYOUT_PATCHWORK",
        "FORMAT_SVG",
        "FORMAT_DOT",
        "FORMAT_XDOT",
        "FORMAT_JSON",
        "FORMAT_PLAIN",
        "FORMAT_PLAIN_EXT",
        "FORMAT_CANON",
        "FORMAT_PNG",
        "FORMAT_PDF",
        "FORMAT_PS",
        "FORMAT_EPS",
        "HAS_CAIRO",
    ]
else:
    __all__ = [
        "GraphvizContext",
        "render_svg",
        "render_png",
        "__version__",
        "LAYOUT_DOT",
        "LAYOUT_NEATO",
        "LAYOUT_FDP",
        "LAYOUT_SFDP",
        "LAYOUT_CIRCO",
        "LAYOUT_TWOPI",
        "LAYOUT_OSAGE",
        "LAYOUT_PATCHWORK",
        "FORMAT_SVG",
        "FORMAT_DOT",
        "FORMAT_XDOT",
        "FORMAT_JSON",
        "FORMAT_PLAIN",
        "FORMAT_PLAIN_EXT",
        "FORMAT_CANON",
        "HAS_CAIRO",
    ]
