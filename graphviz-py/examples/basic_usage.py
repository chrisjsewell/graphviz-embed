"""Example usage of graphviz-embed Python bindings"""

from graphviz_embed import (
    GraphvizContext,
    render_svg,
    LAYOUT_DOT,
    LAYOUT_NEATO,
    FORMAT_SVG,
)


def main():
    """Demonstrate various features of graphviz-embed"""
    
    # Example 1: Basic rendering
    print("Example 1: Basic SVG rendering")
    ctx = GraphvizContext()
    
    dot_source = """
        digraph G {
            rankdir=LR;
            a [label="Start"];
            b [label="Process"];
            c [label="End"];
            a -> b -> c;
        }
    """
    
    svg = ctx.render(dot_source, "dot", "svg")
    print(f"Generated SVG of {len(svg)} bytes\n")
    
    # Example 2: Using convenience function
    print("Example 2: Using render_svg convenience function")
    svg = render_svg("digraph { a -> b -> c }")
    print(f"Generated SVG of {len(svg)} bytes\n")
    
    # Example 3: Different layouts
    print("Example 3: Testing different layouts")
    simple_graph = "graph { a -- b -- c -- d -- a }"
    
    for layout in ["dot", "neato", "circo"]:
        svg = ctx.render(simple_graph, layout, "svg")
        print(f"  {layout}: {len(svg)} bytes")
    print()
    
    # Example 4: HTML labels
    print("Example 4: HTML-like labels")
    html_dot = """
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
    
    svg = ctx.render(html_dot, "dot", "svg")
    print(f"Generated HTML label SVG of {len(svg)} bytes\n")
    
    # Example 5: JSON output
    print("Example 5: JSON output")
    json_output = ctx.render("digraph { a -> b }", "dot", "json")
    print(f"JSON output:\n{json_output.decode('utf-8')[:200]}...\n")
    
    # Example 6: Save to file
    print("Example 6: Save to file")
    ctx.render_to_file(
        "digraph { hello -> world }",
        "dot",
        "svg",
        "example_output.svg"
    )
    print("Saved to example_output.svg")


if __name__ == "__main__":
    main()
