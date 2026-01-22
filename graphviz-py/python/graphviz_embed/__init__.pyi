"""Type stubs for graphviz-embed"""

from typing import Literal

Layout = Literal["dot", "neato", "fdp", "sfdp", "circo", "twopi", "osage", "patchwork"]
Format = Literal["svg", "dot", "xdot", "json", "plain", "plain-ext", "canon", "png", "pdf", "ps", "eps"]

__version__: str

# Layout algorithms
LAYOUT_DOT: str
LAYOUT_NEATO: str
LAYOUT_FDP: str
LAYOUT_SFDP: str
LAYOUT_CIRCO: str
LAYOUT_TWOPI: str
LAYOUT_OSAGE: str
LAYOUT_PATCHWORK: str

# Output formats
FORMAT_SVG: str
FORMAT_DOT: str
FORMAT_XDOT: str
FORMAT_JSON: str
FORMAT_PLAIN: str
FORMAT_PLAIN_EXT: str
FORMAT_CANON: str
FORMAT_PNG: str
FORMAT_PDF: str
FORMAT_PS: str
FORMAT_EPS: str

HAS_CAIRO: bool

class GraphvizContext:
    """Graphviz rendering context"""
    
    def __init__(self) -> None:
        """Create a new Graphviz context
        
        Raises:
            RuntimeError: If Graphviz initialization fails
        """
        ...
    
    def render(self, dot_source: str, layout: Layout, format: Format) -> bytes:
        """Render a DOT graph to the specified format
        
        Args:
            dot_source: The DOT language source code for the graph
            layout: The layout algorithm to use
            format: The output format
        
        Returns:
            The rendered output as bytes
        
        Raises:
            ValueError: If the layout or format is invalid
            RuntimeError: If rendering fails
        """
        ...
    
    def render_to_file(
        self, 
        dot_source: str, 
        layout: Layout, 
        format: Format, 
        path: str
    ) -> None:
        """Render a DOT graph and save to a file
        
        Args:
            dot_source: The DOT language source code
            layout: The layout algorithm to use
            format: The output format
            path: Path to the output file
        
        Raises:
            ValueError: If the layout or format is invalid
            RuntimeError: If rendering or file writing fails
        """
        ...

def render_svg(dot_source: str, layout: Layout = "dot") -> bytes:
    """Render DOT source to SVG
    
    Args:
        dot_source: The DOT language source code for the graph
        layout: The layout algorithm to use. Defaults to "dot"
    
    Returns:
        The rendered SVG as bytes
    
    Raises:
        ValueError: If the layout is invalid
        RuntimeError: If rendering fails
    """
    ...

def render_png(dot_source: str, layout: Layout = "dot") -> bytes:
    """Render DOT source to PNG
    
    Args:
        dot_source: The DOT language source code for the graph
        layout: The layout algorithm to use. Defaults to "dot"
    
    Returns:
        The rendered PNG as bytes
    
    Raises:
        ValueError: If the layout is invalid or cairo feature is not enabled
        RuntimeError: If rendering fails
    """
    ...
