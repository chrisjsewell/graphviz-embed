//! Embedded Graphviz library for Rust
//!
//! This crate provides a safe, high-level API to render DOT graphs using an embedded
//! Graphviz library. No external tools or system dependencies are required for basic
//! SVG output.
//!
//! # Features
//!
//! - **Zero external dependencies** for SVG, DOT, JSON, and plain text output
//! - **HTML-like labels** supported via bundled Expat library
//! - **All layout engines**: dot, neato, fdp, sfdp, circo, twopi, osage, patchwork
//! - **Optional Cairo support** for PNG and PDF output (enable `cairo` feature)
//!
//! # Example
//!
//! ```rust
//! use graphviz_embed::{GraphvizContext, Layout, Format};
//!
//! let ctx = GraphvizContext::new().unwrap();
//!
//! let dot_source = r#"
//!     digraph G {
//!         a -> b -> c;
//!         b -> d;
//!     }
//! "#;
//!
//! let svg = ctx.render(dot_source, Layout::Dot, Format::Svg).unwrap();
//! println!("{}", String::from_utf8_lossy(&svg));
//! ```
//!
//! # HTML Labels
//!
//! HTML-like labels are supported out of the box:
//!
//! ```rust
//! use graphviz_embed::{GraphvizContext, Layout, Format};
//!
//! let ctx = GraphvizContext::new().unwrap();
//!
//! let dot_source = r#"
//!     digraph G {
//!         node [shape=none];
//!         a [label=<<TABLE><TR><TD>Hello</TD></TR></TABLE>>];
//!     }
//! "#;
//!
//! let svg = ctx.render(dot_source, Layout::Dot, Format::Svg).unwrap();
//! ```
//!
//! # Cairo Feature
//!
//! Enable the `cairo` feature for PNG and PDF output:
//!
//! ```toml
//! [dependencies]
//! graphviz-embed = { version = "0.1", features = ["cairo"] }
//! ```
//!
//! This requires Cairo and Pango libraries to be installed on the system:
//! - **Linux**: `apt install libcairo2-dev libpango1.0-dev`
//! - **macOS**: `brew install cairo pango`
//! - **Windows**: Install via vcpkg

use std::ffi::{CStr, CString};
use std::ptr;

use graphviz_sys as sys;
use thiserror::Error;

/// Errors that can occur when using Graphviz
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to initialize Graphviz context
    #[error("failed to initialize Graphviz context")]
    InitFailed,

    /// Failed to parse DOT source
    #[error("failed to parse DOT source: {0}")]
    ParseFailed(String),

    /// Failed to apply layout
    #[error("failed to apply layout '{0}'")]
    LayoutFailed(String),

    /// Failed to render output
    #[error("failed to render to format '{0}'")]
    RenderFailed(String),

    /// Invalid input string (contains null bytes)
    #[error("invalid input: {0}")]
    InvalidInput(#[from] std::ffi::NulError),

    /// Feature not enabled
    #[error("feature not enabled: {0}")]
    FeatureNotEnabled(&'static str),
}

/// Result type for Graphviz operations
pub type Result<T> = std::result::Result<T, Error>;

/// Layout algorithms available in Graphviz
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layout {
    /// Hierarchical layout for directed graphs (default)
    Dot,
    /// Spring model layout
    Neato,
    /// Force-directed placement
    Fdp,
    /// Scalable force-directed placement
    Sfdp,
    /// Circular layout
    Circo,
    /// Radial layout
    Twopi,
    /// Clustered layout
    Osage,
    /// Treemap layout
    Patchwork,
}

impl Layout {
    /// Get the layout name as used by Graphviz
    pub fn as_str(&self) -> &'static str {
        match self {
            Layout::Dot => "dot",
            Layout::Neato => "neato",
            Layout::Fdp => "fdp",
            Layout::Sfdp => "sfdp",
            Layout::Circo => "circo",
            Layout::Twopi => "twopi",
            Layout::Osage => "osage",
            Layout::Patchwork => "patchwork",
        }
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Output formats available in Graphviz
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// SVG vector graphics (no external dependencies)
    Svg,
    /// DOT source with layout information
    Dot,
    /// Extended DOT format
    Xdot,
    /// JSON output
    Json,
    /// Plain text coordinates
    Plain,
    /// Plain text with extended info
    PlainExt,
    /// Canonical DOT
    Canon,

    /// PNG raster image (requires `cairo` feature)
    #[cfg(feature = "cairo")]
    Png,

    /// PDF document (requires `cairo` feature)
    #[cfg(feature = "cairo")]
    Pdf,

    /// PostScript (requires `cairo` feature)
    #[cfg(feature = "cairo")]
    Ps,

    /// Encapsulated PostScript (requires `cairo` feature)
    #[cfg(feature = "cairo")]
    Eps,
}

impl Format {
    /// Get the format name as used by Graphviz
    pub fn as_str(&self) -> &'static str {
        match self {
            Format::Svg => "svg",
            Format::Dot => "dot",
            Format::Xdot => "xdot",
            Format::Json => "json",
            Format::Plain => "plain",
            Format::PlainExt => "plain-ext",
            Format::Canon => "canon",
            #[cfg(feature = "cairo")]
            Format::Png => "png",
            #[cfg(feature = "cairo")]
            Format::Pdf => "pdf",
            #[cfg(feature = "cairo")]
            Format::Ps => "ps",
            #[cfg(feature = "cairo")]
            Format::Eps => "eps",
        }
    }

    /// Check if this format requires Cairo
    pub fn requires_cairo(&self) -> bool {
        match self {
            Format::Svg
            | Format::Dot
            | Format::Xdot
            | Format::Json
            | Format::Plain
            | Format::PlainExt
            | Format::Canon => false,
            #[cfg(feature = "cairo")]
            Format::Png | Format::Pdf | Format::Ps | Format::Eps => true,
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Global mutex to serialize Graphviz operations (the C library has global state)
static GRAPHVIZ_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

// Global shared context - Graphviz has issues with repeated context creation/destruction
// on some platforms (notably macOS Intel), so we use a single shared context that's never freed.
// This is safe because all operations are serialized via GRAPHVIZ_MUTEX.
struct GvcPtr(*mut sys::GVC_t);
unsafe impl Send for GvcPtr {}
unsafe impl Sync for GvcPtr {}

static GLOBAL_CONTEXT: std::sync::OnceLock<GvcPtr> = std::sync::OnceLock::new();

fn get_global_context() -> Result<*mut sys::GVC_t> {
    let ctx = GLOBAL_CONTEXT.get_or_init(|| {
        let gvc = unsafe { sys::gv_init() };
        GvcPtr(gvc)
    });
    if ctx.0.is_null() {
        Err(Error::InitFailed)
    } else {
        Ok(ctx.0)
    }
}

/// Graphviz rendering context
///
/// This is the main entry point for rendering DOT graphs. Create a context
/// with [`GraphvizContext::new()`] and use [`GraphvizContext::render()`] to
/// generate output.
///
/// # Thread Safety
///
/// `GraphvizContext` is both `Send` and `Sync`. The underlying Graphviz library
/// has global state, so all operations are serialized internally using a mutex.
///
/// # Note
///
/// Internally, all `GraphvizContext` instances share the same underlying Graphviz
/// context. This is because the Graphviz C library has global state and issues with
/// repeated context creation/destruction on some platforms. Creating multiple
/// `GraphvizContext` instances is safe and has negligible overhead.
pub struct GraphvizContext {
    // We store a copy of the pointer, but all instances share the same global context
    gvc: *mut sys::GVC_t,
}

// GraphvizContext can be sent to another thread
unsafe impl Send for GraphvizContext {}
// Operations are serialized via mutex
unsafe impl Sync for GraphvizContext {}

impl GraphvizContext {
    /// Create a new Graphviz context
    ///
    /// This initializes the Graphviz library (on first call) and returns a handle
    /// to the shared rendering context.
    ///
    /// # Errors
    ///
    /// Returns an error if Graphviz initialization fails.
    pub fn new() -> Result<Self> {
        let gvc = get_global_context()?;
        Ok(Self { gvc })
    }

    /// Render a DOT graph to the specified format
    ///
    /// # Arguments
    ///
    /// * `dot_source` - The DOT language source code for the graph
    /// * `layout` - The layout algorithm to use
    /// * `format` - The output format
    ///
    /// # Returns
    ///
    /// The rendered output as a byte vector.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The DOT source is invalid
    /// - The layout algorithm fails
    /// - Rendering fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use graphviz_embed::{GraphvizContext, Layout, Format};
    ///
    /// let ctx = GraphvizContext::new().unwrap();
    /// let svg = ctx.render("digraph { a -> b }", Layout::Dot, Format::Svg).unwrap();
    /// ```
    pub fn render(&self, dot_source: &str, layout: Layout, format: Format) -> Result<Vec<u8>> {
        // Serialize access to Graphviz (C library has global state)
        let _lock = GRAPHVIZ_MUTEX.lock().unwrap();

        let dot_cstring = CString::new(dot_source)?;
        let layout_cstring = CString::new(layout.as_str()).unwrap();
        let format_cstring = CString::new(format.as_str()).unwrap();

        unsafe {
            // Parse the DOT source
            let graph = sys::agmemread(dot_cstring.as_ptr());
            if graph.is_null() {
                return Err(Error::ParseFailed(get_last_error()));
            }

            // Apply layout
            let result = sys::gvLayout(self.gvc, graph, layout_cstring.as_ptr());
            if result != 0 {
                sys::agclose(graph);
                return Err(Error::LayoutFailed(layout.to_string()));
            }

            // Render to memory
            let mut data: *mut libc::c_char = ptr::null_mut();
            let mut len: libc::c_uint = 0;

            let result = sys::gvRenderData(
                self.gvc,
                graph,
                format_cstring.as_ptr(),
                &mut data,
                &mut len,
            );

            if result != 0 || data.is_null() {
                sys::gvFreeLayout(self.gvc, graph);
                sys::agclose(graph);
                return Err(Error::RenderFailed(format.to_string()));
            }

            // Copy data to Rust Vec
            let output = std::slice::from_raw_parts(data as *const u8, len as usize).to_vec();

            // Cleanup
            sys::gvFreeRenderData(data);
            sys::gvFreeLayout(self.gvc, graph);
            sys::agclose(graph);

            Ok(output)
        }
    }

    /// Render a DOT graph and save to a file
    ///
    /// # Arguments
    ///
    /// * `dot_source` - The DOT language source code
    /// * `layout` - The layout algorithm to use
    /// * `format` - The output format
    /// * `path` - Path to the output file
    ///
    /// # Errors
    ///
    /// Returns an error if parsing, layout, or file writing fails.
    pub fn render_to_file(
        &self,
        dot_source: &str,
        layout: Layout,
        format: Format,
        path: &std::path::Path,
    ) -> Result<()> {
        let output = self.render(dot_source, layout, format)?;
        std::fs::write(path, output).map_err(|e| Error::RenderFailed(e.to_string()))?;
        Ok(())
    }
}

// Note: We intentionally don't implement Drop to free the context.
// The global context is shared across all GraphvizContext instances and
// is never freed. This avoids issues with Graphviz's global state on
// platforms like macOS Intel where repeated context creation/destruction
// can cause crashes.

impl Default for GraphvizContext {
    fn default() -> Self {
        Self::new().expect("failed to create GraphvizContext")
    }
}

/// Get the last Graphviz error message
fn get_last_error() -> String {
    unsafe {
        let err = sys::aglasterr();
        if err.is_null() {
            "unknown error".to_string()
        } else {
            CStr::from_ptr(err).to_string_lossy().into_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let ctx = GraphvizContext::new();
        assert!(ctx.is_ok());
    }

    #[test]
    fn test_simple_render() {
        let ctx = GraphvizContext::new().unwrap();
        let result = ctx.render("digraph { a -> b }", Layout::Dot, Format::Svg);
        assert!(result.is_ok());
        let output = result.unwrap();
        let svg = String::from_utf8_lossy(&output);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_invalid_dot() {
        let ctx = GraphvizContext::new().unwrap();
        let result = ctx.render("this is not valid DOT", Layout::Dot, Format::Svg);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_layouts() {
        let ctx = GraphvizContext::new().unwrap();
        let dot = "digraph { a -> b -> c }";

        for layout in [
            Layout::Dot,
            Layout::Neato,
            Layout::Fdp,
            Layout::Circo,
            Layout::Twopi,
        ] {
            let result = ctx.render(dot, layout, Format::Svg);
            assert!(result.is_ok(), "Layout {:?} failed", layout);
        }
    }

    #[test]
    fn test_json_output() {
        let ctx = GraphvizContext::new().unwrap();
        let result = ctx.render("digraph { a -> b }", Layout::Dot, Format::Json);
        assert!(result.is_ok());
        let output = result.unwrap();
        let json = String::from_utf8_lossy(&output);
        assert!(json.contains("\"name\""));
    }
}
