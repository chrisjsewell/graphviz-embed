//! Integration tests for graphviz-embed
//!
//! These tests verify the end-to-end functionality of the library.

use graphviz_embed::{Format, GraphvizContext, Layout};

/// Test basic SVG rendering with a simple graph
#[test]
fn test_render_simple_svg() {
    let ctx = GraphvizContext::new().expect("Failed to create context");

    let dot = "digraph G { a -> b }";
    let result = ctx.render(dot, Layout::Dot, Format::Svg);

    assert!(result.is_ok(), "Render failed: {:?}", result.err());

    let output = result.unwrap();
    let svg = String::from_utf8_lossy(&output);
    assert!(svg.contains("<svg"), "Output should be SVG");
    assert!(svg.contains("</svg>"), "SVG should be complete");
    assert!(
        svg.contains("a") || svg.contains("node"),
        "SVG should contain node"
    );
}

/// Test DOT layout engine (hierarchical)
#[test]
fn test_layout_dot() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        digraph G {
            rankdir=TB;
            a -> b -> c -> d;
        }
    "#;

    let result = ctx.render(dot, Layout::Dot, Format::Svg);
    assert!(result.is_ok(), "Dot layout failed: {:?}", result.err());
}

/// Test Neato layout engine (spring model)
#[test]
fn test_layout_neato() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        graph G {
            a -- b;
            b -- c;
            c -- a;
        }
    "#;

    let result = ctx.render(dot, Layout::Neato, Format::Svg);
    assert!(result.is_ok(), "Neato layout failed: {:?}", result.err());
}

/// Test FDP layout engine (force-directed)
#[test]
fn test_layout_fdp() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        graph G {
            a -- b -- c -- d -- a;
        }
    "#;

    let result = ctx.render(dot, Layout::Fdp, Format::Svg);
    assert!(result.is_ok(), "FDP layout failed: {:?}", result.err());
}

/// Test Circo layout engine (circular)
#[test]
fn test_layout_circo() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        digraph G {
            a -> b -> c -> d -> e -> a;
        }
    "#;

    let result = ctx.render(dot, Layout::Circo, Format::Svg);
    assert!(result.is_ok(), "Circo layout failed: {:?}", result.err());
}

/// Test Twopi layout engine (radial)
#[test]
fn test_layout_twopi() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        digraph G {
            root -> a;
            root -> b;
            root -> c;
            a -> a1;
            a -> a2;
            b -> b1;
        }
    "#;

    let result = ctx.render(dot, Layout::Twopi, Format::Svg);
    assert!(result.is_ok(), "Twopi layout failed: {:?}", result.err());
}

/// Test HTML-like labels in nodes
#[test]
fn test_html_labels() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        digraph G {
            node [shape=none];
            a [label=<
                <TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">
                    <TR><TD BGCOLOR="lightblue"><B>Header</B></TD></TR>
                    <TR><TD>Row 1</TD></TR>
                    <TR><TD>Row 2</TD></TR>
                </TABLE>
            >];
            b [label=<<B>Bold</B> and <I>Italic</I>>];
            a -> b;
        }
    "#;

    let result = ctx.render(dot, Layout::Dot, Format::Svg);
    assert!(result.is_ok(), "HTML labels failed: {:?}", result.err());

    let output = result.unwrap();
    let svg = String::from_utf8_lossy(&output);
    // The HTML content should be rendered (text will appear in SVG)
    assert!(
        svg.contains("Header") || svg.contains("text"),
        "HTML content should be rendered"
    );
}

/// Test error handling for invalid DOT syntax
#[test]
fn test_invalid_dot_error() {
    let ctx = GraphvizContext::new().unwrap();

    let invalid_dot = "this is not valid DOT syntax {{{";
    let result = ctx.render(invalid_dot, Layout::Dot, Format::Svg);

    assert!(result.is_err(), "Should fail on invalid DOT");
}

/// Test JSON output format
#[test]
fn test_json_output() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = "digraph G { a -> b }";
    let result = ctx.render(dot, Layout::Dot, Format::Json);

    assert!(result.is_ok(), "JSON render failed: {:?}", result.err());

    let output = result.unwrap();
    let json = String::from_utf8_lossy(&output);
    assert!(json.contains("{"), "Output should be JSON");
    assert!(
        json.contains("name") || json.contains("objects"),
        "JSON should have structure"
    );
}

/// Test plain text output format
#[test]
fn test_plain_output() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = "digraph G { a -> b }";
    let result = ctx.render(dot, Layout::Dot, Format::Plain);

    assert!(result.is_ok(), "Plain render failed: {:?}", result.err());

    let output = result.unwrap();
    let plain = String::from_utf8_lossy(&output);
    assert!(
        plain.contains("node") || plain.contains("edge"),
        "Plain output should have elements"
    );
}

/// Test rendering with node and edge attributes
#[test]
fn test_attributes() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        digraph G {
            node [shape=box, color=blue, fontname="Helvetica"];
            edge [color=red, style=dashed];
            
            a [label="Node A", fillcolor=yellow, style=filled];
            b [label="Node B", shape=ellipse];
            
            a -> b [label="edge label", weight=2];
        }
    "#;

    let result = ctx.render(dot, Layout::Dot, Format::Svg);
    assert!(
        result.is_ok(),
        "Attributes render failed: {:?}",
        result.err()
    );
}

/// Test subgraphs and clusters
#[test]
fn test_clusters() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        digraph G {
            subgraph cluster_0 {
                label = "Cluster 0";
                a0 -> a1 -> a2;
            }
            
            subgraph cluster_1 {
                label = "Cluster 1";
                b0 -> b1 -> b2;
            }
            
            a1 -> b1;
        }
    "#;

    let result = ctx.render(dot, Layout::Dot, Format::Svg);
    assert!(result.is_ok(), "Cluster render failed: {:?}", result.err());
}

/// Test rendering to xdot format
#[test]
fn test_xdot_output() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = "digraph G { a -> b }";
    let result = ctx.render(dot, Layout::Dot, Format::Xdot);

    assert!(result.is_ok(), "Xdot render failed: {:?}", result.err());

    let output = result.unwrap();
    let xdot = String::from_utf8_lossy(&output);
    assert!(
        xdot.contains("_draw_") || xdot.contains("pos"),
        "Xdot should have drawing commands"
    );
}

/// Test that context can be reused for multiple renders
#[test]
fn test_context_reuse() {
    let ctx = GraphvizContext::new().unwrap();

    // Render multiple graphs with the same context
    for i in 0..5 {
        let dot = format!("digraph G{} {{ a{} -> b{} }}", i, i, i);
        let result = ctx.render(&dot, Layout::Dot, Format::Svg);
        assert!(result.is_ok(), "Render {} failed: {:?}", i, result.err());
    }
}

/// Test large graph rendering
#[test]
fn test_large_graph() {
    let ctx = GraphvizContext::new().unwrap();

    // Build a larger graph
    let mut dot = String::from("digraph G {\n");
    for i in 0..100 {
        dot.push_str(&format!("    n{} -> n{};\n", i, (i + 1) % 100));
    }
    dot.push_str("}\n");

    let result = ctx.render(&dot, Layout::Dot, Format::Svg);
    assert!(
        result.is_ok(),
        "Large graph render failed: {:?}",
        result.err()
    );
}

/// Test empty graph
#[test]
fn test_empty_graph() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = "digraph G { }";
    let result = ctx.render(dot, Layout::Dot, Format::Svg);
    assert!(
        result.is_ok(),
        "Empty graph render failed: {:?}",
        result.err()
    );
}

/// Test undirected graph
#[test]
fn test_undirected_graph() {
    let ctx = GraphvizContext::new().unwrap();

    let dot = r#"
        graph G {
            a -- b -- c;
            b -- d;
        }
    "#;

    let result = ctx.render(dot, Layout::Neato, Format::Svg);
    assert!(
        result.is_ok(),
        "Undirected graph render failed: {:?}",
        result.err()
    );
}

// Cairo-specific tests (only compiled with cairo feature)
#[cfg(feature = "cairo")]
mod cairo_tests {
    use super::*;

    /// Test PNG output format
    #[test]
    fn test_render_png() {
        let ctx = GraphvizContext::new().unwrap();

        let dot = "digraph G { a -> b }";
        let result = ctx.render(dot, Layout::Dot, Format::Png);

        assert!(result.is_ok(), "PNG render failed: {:?}", result.err());

        let png = result.unwrap();
        // Check PNG magic bytes
        assert!(png.len() > 8, "PNG should have content");
        assert_eq!(
            &png[0..4],
            &[0x89, 0x50, 0x4E, 0x47],
            "Should start with PNG magic"
        );
    }

    /// Test PDF output format
    #[test]
    fn test_render_pdf() {
        let ctx = GraphvizContext::new().unwrap();

        let dot = "digraph G { a -> b }";
        let result = ctx.render(dot, Layout::Dot, Format::Pdf);

        assert!(result.is_ok(), "PDF render failed: {:?}", result.err());

        let pdf = result.unwrap();
        let header = String::from_utf8_lossy(&pdf[0..5.min(pdf.len())]);
        assert!(header.contains("%PDF"), "Should start with PDF header");
    }
}
