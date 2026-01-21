"""Tests for graphviz-embed Python bindings"""

import pytest
from graphviz_embed import (
    GraphvizContext,
    render_svg,
    LAYOUT_DOT,
    LAYOUT_NEATO,
    FORMAT_SVG,
    FORMAT_JSON,
    HAS_CAIRO,
)


def test_context_creation():
    """Test creating a GraphvizContext"""
    ctx = GraphvizContext()
    assert ctx is not None


def test_simple_render():
    """Test basic SVG rendering"""
    ctx = GraphvizContext()
    svg = ctx.render("digraph { a -> b }", "dot", "svg")
    assert isinstance(svg, bytes)
    svg_str = svg.decode('utf-8')
    assert '<svg' in svg_str
    assert '</svg>' in svg_str


def test_render_svg_function():
    """Test the render_svg convenience function"""
    svg = render_svg("digraph { a -> b }")
    assert isinstance(svg, bytes)
    svg_str = svg.decode('utf-8')
    assert '<svg' in svg_str


def test_all_layouts():
    """Test all layout algorithms"""
    ctx = GraphvizContext()
    dot = "digraph { a -> b -> c }"
    
    for layout in ["dot", "neato", "fdp", "circo", "twopi"]:
        svg = ctx.render(dot, layout, "svg")
        assert isinstance(svg, bytes)
        assert b'<svg' in svg


def test_json_output():
    """Test JSON output format"""
    ctx = GraphvizContext()
    json = ctx.render("digraph { a -> b }", "dot", "json")
    assert isinstance(json, bytes)
    json_str = json.decode('utf-8')
    assert '"name"' in json_str


def test_html_labels():
    """Test HTML-like labels"""
    ctx = GraphvizContext()
    dot = '''
        digraph G {
            node [shape=none];
            a [label=<<TABLE><TR><TD>Hello</TD></TR></TABLE>>];
        }
    '''
    svg = ctx.render(dot, "dot", "svg")
    assert isinstance(svg, bytes)
    assert b'<svg' in svg


def test_invalid_dot():
    """Test error handling for invalid DOT syntax"""
    ctx = GraphvizContext()
    with pytest.raises(RuntimeError):
        ctx.render("this is not valid DOT", "dot", "svg")


def test_invalid_layout():
    """Test error handling for invalid layout"""
    ctx = GraphvizContext()
    with pytest.raises(ValueError, match="Invalid layout"):
        ctx.render("digraph { a -> b }", "invalid", "svg")


def test_invalid_format():
    """Test error handling for invalid format"""
    ctx = GraphvizContext()
    with pytest.raises(ValueError, match="Invalid format"):
        ctx.render("digraph { a -> b }", "dot", "invalid")


def test_render_to_file(tmp_path):
    """Test rendering to a file"""
    ctx = GraphvizContext()
    output_file = tmp_path / "output.svg"
    ctx.render_to_file("digraph { a -> b }", "dot", "svg", str(output_file))
    
    assert output_file.exists()
    content = output_file.read_text()
    assert '<svg' in content


def test_constants():
    """Test module constants"""
    assert LAYOUT_DOT == "dot"
    assert LAYOUT_NEATO == "neato"
    assert FORMAT_SVG == "svg"
    assert FORMAT_JSON == "json"
    assert isinstance(HAS_CAIRO, bool)


@pytest.mark.skipif(not HAS_CAIRO, reason="Cairo feature not enabled")
def test_png_rendering():
    """Test PNG rendering with Cairo"""
    from graphviz_embed import render_png, FORMAT_PNG
    
    ctx = GraphvizContext()
    png = ctx.render("digraph { a -> b }", "dot", "png")
    assert isinstance(png, bytes)
    # PNG signature
    assert png[:8] == b'\x89PNG\r\n\x1a\n'
    
    # Test convenience function
    png2 = render_png("digraph { a -> b }")
    assert isinstance(png2, bytes)
    assert png2[:8] == b'\x89PNG\r\n\x1a\n'


@pytest.mark.skipif(HAS_CAIRO, reason="Cairo feature is enabled")
def test_png_without_cairo():
    """Test that PNG raises an error without Cairo"""
    ctx = GraphvizContext()
    with pytest.raises(ValueError, match="requires the 'cairo' feature"):
        ctx.render("digraph { a -> b }", "dot", "png")
