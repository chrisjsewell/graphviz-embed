//! Python bindings for graphviz-embed
//!
//! This crate provides Python bindings for the graphviz-embed Rust library,
//! allowing Python users to render DOT graphs without external dependencies.

use graphviz_embed::{Format as RustFormat, GraphvizContext, Layout as RustLayout};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

/// Python wrapper for the Graphviz context
#[pyclass(name = "GraphvizContext")]
struct PyGraphvizContext {
    ctx: GraphvizContext,
}

#[pymethods]
impl PyGraphvizContext {
    /// Create a new Graphviz context
    ///
    /// Returns:
    ///     GraphvizContext: A new Graphviz rendering context
    ///
    /// Raises:
    ///     RuntimeError: If Graphviz initialization fails
    #[new]
    fn new() -> PyResult<Self> {
        GraphvizContext::new()
            .map(|ctx| Self { ctx })
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to initialize Graphviz: {}", e)))
    }

    /// Render a DOT graph to the specified format
    ///
    /// Args:
    ///     dot_source (str): The DOT language source code for the graph
    ///     layout (str): The layout algorithm to use (e.g., "dot", "neato", "fdp")
    ///     format (str): The output format (e.g., "svg", "png", "pdf")
    ///
    /// Returns:
    ///     bytes: The rendered output as bytes
    ///
    /// Raises:
    ///     ValueError: If the layout or format is invalid
    ///     RuntimeError: If rendering fails
    ///
    /// Example:
    ///     >>> ctx = GraphvizContext()
    ///     >>> svg = ctx.render("digraph { a -> b }", "dot", "svg")
    ///     >>> print(svg.decode('utf-8'))
    fn render(&self, dot_source: &str, layout: &str, format: &str) -> PyResult<Vec<u8>> {
        let rust_layout = parse_layout(layout)?;
        let rust_format = parse_format(format)?;

        self.ctx
            .render(dot_source, rust_layout, rust_format)
            .map_err(|e| PyRuntimeError::new_err(format!("Rendering failed: {}", e)))
    }

    /// Render a DOT graph and save to a file
    ///
    /// Args:
    ///     dot_source (str): The DOT language source code
    ///     layout (str): The layout algorithm to use
    ///     format (str): The output format
    ///     path (str): Path to the output file
    ///
    /// Raises:
    ///     ValueError: If the layout or format is invalid
    ///     RuntimeError: If rendering or file writing fails
    ///
    /// Example:
    ///     >>> ctx = GraphvizContext()
    ///     >>> ctx.render_to_file("digraph { a -> b }", "dot", "svg", "output.svg")
    fn render_to_file(
        &self,
        dot_source: &str,
        layout: &str,
        format: &str,
        path: &str,
    ) -> PyResult<()> {
        let rust_layout = parse_layout(layout)?;
        let rust_format = parse_format(format)?;
        let file_path = std::path::Path::new(path);

        self.ctx
            .render_to_file(dot_source, rust_layout, rust_format, file_path)
            .map_err(|e| PyRuntimeError::new_err(format!("Rendering to file failed: {}", e)))
    }

    fn __repr__(&self) -> String {
        "GraphvizContext()".to_string()
    }
}

/// Parse a layout string into a Rust Layout enum
fn parse_layout(layout: &str) -> PyResult<RustLayout> {
    match layout.to_lowercase().as_str() {
        "dot" => Ok(RustLayout::Dot),
        "neato" => Ok(RustLayout::Neato),
        "fdp" => Ok(RustLayout::Fdp),
        "sfdp" => Ok(RustLayout::Sfdp),
        "circo" => Ok(RustLayout::Circo),
        "twopi" => Ok(RustLayout::Twopi),
        "osage" => Ok(RustLayout::Osage),
        "patchwork" => Ok(RustLayout::Patchwork),
        _ => Err(PyValueError::new_err(format!(
            "Invalid layout '{}'. Valid options: dot, neato, fdp, sfdp, circo, twopi, osage, patchwork",
            layout
        ))),
    }
}

/// Parse a format string into a Rust Format enum
fn parse_format(format: &str) -> PyResult<RustFormat> {
    match format.to_lowercase().as_str() {
        "svg" => Ok(RustFormat::Svg),
        "dot" => Ok(RustFormat::Dot),
        "xdot" => Ok(RustFormat::Xdot),
        "json" => Ok(RustFormat::Json),
        "plain" => Ok(RustFormat::Plain),
        "plain-ext" | "plainext" => Ok(RustFormat::PlainExt),
        "canon" => Ok(RustFormat::Canon),
        #[cfg(feature = "cairo")]
        "png" => Ok(RustFormat::Png),
        #[cfg(feature = "cairo")]
        "pdf" => Ok(RustFormat::Pdf),
        #[cfg(feature = "cairo")]
        "ps" => Ok(RustFormat::Ps),
        #[cfg(feature = "cairo")]
        "eps" => Ok(RustFormat::Eps),
        #[cfg(not(feature = "cairo"))]
        "png" | "pdf" | "ps" | "eps" => Err(PyValueError::new_err(format!(
            "Format '{}' requires the 'cairo' feature to be enabled",
            format
        ))),
        _ => Err(PyValueError::new_err(format!(
            "Invalid format '{}'. Valid options: svg, dot, xdot, json, plain, plain-ext, canon{}",
            format,
            if cfg!(feature = "cairo") { ", png, pdf, ps, eps" } else { " (png, pdf, ps, eps require cairo feature)" }
        ))),
    }
}

/// A convenience function to render DOT source to SVG
///
/// Args:
///     dot_source (str): The DOT language source code for the graph
///     layout (str, optional): The layout algorithm to use. Defaults to "dot"
///
/// Returns:
///     bytes: The rendered SVG as bytes
///
/// Raises:
///     ValueError: If the layout is invalid
///     RuntimeError: If rendering fails
///
/// Example:
///     >>> svg = render_svg("digraph { a -> b }")
///     >>> print(svg.decode('utf-8'))
#[pyfunction]
#[pyo3(signature = (dot_source, layout="dot"))]
fn render_svg(dot_source: &str, layout: &str) -> PyResult<Vec<u8>> {
    let ctx = PyGraphvizContext::new()?;
    ctx.render(dot_source, layout, "svg")
}

/// A convenience function to render DOT source to PNG
///
/// Args:
///     dot_source (str): The DOT language source code for the graph
///     layout (str, optional): The layout algorithm to use. Defaults to "dot"
///
/// Returns:
///     bytes: The rendered PNG as bytes
///
/// Raises:
///     ValueError: If the layout is invalid or cairo feature is not enabled
///     RuntimeError: If rendering fails
///
/// Example:
///     >>> png = render_png("digraph { a -> b }")
///     >>> with open("output.png", "wb") as f:
///     ...     f.write(png)
#[pyfunction]
#[pyo3(signature = (dot_source, layout="dot"))]
fn render_png(dot_source: &str, layout: &str) -> PyResult<Vec<u8>> {
    let ctx = PyGraphvizContext::new()?;
    ctx.render(dot_source, layout, "png")
}

/// Python module for graphviz-embed
#[pymodule]
fn _graphviz_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGraphvizContext>()?;
    m.add_function(wrap_pyfunction!(render_svg, m)?)?;
    m.add_function(wrap_pyfunction!(render_png, m)?)?;

    // Module-level constants
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Layout algorithms
    m.add("LAYOUT_DOT", "dot")?;
    m.add("LAYOUT_NEATO", "neato")?;
    m.add("LAYOUT_FDP", "fdp")?;
    m.add("LAYOUT_SFDP", "sfdp")?;
    m.add("LAYOUT_CIRCO", "circo")?;
    m.add("LAYOUT_TWOPI", "twopi")?;
    m.add("LAYOUT_OSAGE", "osage")?;
    m.add("LAYOUT_PATCHWORK", "patchwork")?;

    // Output formats
    m.add("FORMAT_SVG", "svg")?;
    m.add("FORMAT_DOT", "dot")?;
    m.add("FORMAT_XDOT", "xdot")?;
    m.add("FORMAT_JSON", "json")?;
    m.add("FORMAT_PLAIN", "plain")?;
    m.add("FORMAT_PLAIN_EXT", "plain-ext")?;
    m.add("FORMAT_CANON", "canon")?;

    #[cfg(feature = "cairo")]
    {
        m.add("FORMAT_PNG", "png")?;
        m.add("FORMAT_PDF", "pdf")?;
        m.add("FORMAT_PS", "ps")?;
        m.add("FORMAT_EPS", "eps")?;
        m.add("HAS_CAIRO", true)?;
    }

    #[cfg(not(feature = "cairo"))]
    {
        m.add("HAS_CAIRO", false)?;
    }

    Ok(())
}
