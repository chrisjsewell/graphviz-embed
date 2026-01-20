//! Low-level FFI bindings to Graphviz
//!
//! This crate provides raw bindings to the Graphviz C library with vendored source.
//! For a safe, high-level API, use the `graphviz-embed` crate instead.
//!
//! # Features
//!
//! - `cairo` - Enable Cairo/Pango rendering for PNG, PDF output formats

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_char, c_int, c_uint, c_void, FILE};

/// Graphviz context handle
pub type GVC_t = c_void;

/// Graph handle
pub type Agraph_t = c_void;

/// Node handle
pub type Agnode_t = c_void;

/// Edge handle
pub type Agedge_t = c_void;

/// Graph descriptor for directed graphs
pub type Agdesc_t = c_void;

/// Plugin library structure
#[repr(C)]
pub struct gvplugin_library_t {
    pub packagename: *const c_char,
    pub apis: *const gvplugin_api_t,
}

/// Plugin API entry
#[repr(C)]
pub struct gvplugin_api_t {
    pub api: c_int,
    pub types: *const gvplugin_installed_t,
}

/// Installed plugin info
#[repr(C)]
pub struct gvplugin_installed_t {
    pub id: c_int,
    pub type_name: *const c_char,
    pub quality: c_int,
    pub engine: *const c_void,
    pub features: *const c_void,
}

// External plugin library symbols for static linking
extern "C" {
    pub static gvplugin_dot_layout_LTX_library: gvplugin_library_t;
    pub static gvplugin_neato_layout_LTX_library: gvplugin_library_t;
    pub static gvplugin_core_LTX_library: gvplugin_library_t;

    #[cfg(has_cairo)]
    pub static gvplugin_pango_LTX_library: gvplugin_library_t;
}

extern "C" {
    // GVC (Graphviz Context) functions

    /// Create a new Graphviz context
    pub fn gvContext() -> *mut GVC_t;

    /// Create a new Graphviz context with plugin builtins
    pub fn gvContextPlugins(
        builtins: *const gvplugin_library_t,
        demand_loading: c_int,
    ) -> *mut GVC_t;

    /// Free a Graphviz context
    pub fn gvFreeContext(gvc: *mut GVC_t) -> c_int;

    /// Add a plugin library to the context
    pub fn gvAddLibrary(gvc: *mut GVC_t, lib: *const gvplugin_library_t);

    // Graph parsing functions

    /// Read a graph from a string
    pub fn agmemread(cp: *const c_char) -> *mut Agraph_t;

    /// Read a graph from a file
    pub fn agread(fp: *mut FILE, disc: *mut c_void) -> *mut Agraph_t;

    /// Close/free a graph
    pub fn agclose(g: *mut Agraph_t) -> c_int;

    // Layout functions

    /// Apply a layout algorithm to a graph
    pub fn gvLayout(gvc: *mut GVC_t, g: *mut Agraph_t, engine: *const c_char) -> c_int;

    /// Free layout data
    pub fn gvFreeLayout(gvc: *mut GVC_t, g: *mut Agraph_t) -> c_int;

    // Rendering functions

    /// Render a graph to a file
    pub fn gvRenderFilename(
        gvc: *mut GVC_t,
        g: *mut Agraph_t,
        format: *const c_char,
        filename: *const c_char,
    ) -> c_int;

    /// Render a graph to memory
    pub fn gvRenderData(
        gvc: *mut GVC_t,
        g: *mut Agraph_t,
        format: *const c_char,
        result: *mut *mut c_char,
        length: *mut c_uint,
    ) -> c_int;

    /// Free rendered data
    pub fn gvFreeRenderData(data: *mut c_char);

    // Error handling

    /// Get last error
    pub fn aglasterr() -> *mut c_char;

    /// Set error callback
    pub fn agseterr(err: c_int) -> c_int;

    /// Error levels
    pub static AGWARN: c_int;
    pub static AGERR: c_int;
    pub static AGMAX: c_int;
    pub static AGPREV: c_int;
}

/// Initialize Graphviz with statically linked plugins
///
/// This must be called before using any Graphviz functions.
/// Returns a configured GVC context with all layout engines registered.
///
/// # Safety
///
/// This function initializes global state in the Graphviz library.
/// It should only be called once per process.
pub unsafe fn gv_init() -> *mut GVC_t {
    let gvc = gvContext();
    if gvc.is_null() {
        return gvc;
    }

    // Register statically linked plugins
    gvAddLibrary(gvc, &gvplugin_dot_layout_LTX_library);
    gvAddLibrary(gvc, &gvplugin_neato_layout_LTX_library);
    gvAddLibrary(gvc, &gvplugin_core_LTX_library);

    #[cfg(has_cairo)]
    gvAddLibrary(gvc, &gvplugin_pango_LTX_library);

    gvc
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_context_creation() {
        unsafe {
            let gvc = gv_init();
            assert!(!gvc.is_null());
            gvFreeContext(gvc);
        }
    }

    #[test]
    fn test_parse_simple_graph() {
        unsafe {
            let gvc = gv_init();
            assert!(!gvc.is_null());

            let dot = CString::new("digraph { a -> b }").unwrap();
            let graph = agmemread(dot.as_ptr());
            assert!(!graph.is_null());

            agclose(graph);
            gvFreeContext(gvc);
        }
    }

    #[test]
    fn test_layout_and_render() {
        unsafe {
            let gvc = gv_init();
            assert!(!gvc.is_null());

            let dot = CString::new("digraph { a -> b }").unwrap();
            let graph = agmemread(dot.as_ptr());
            assert!(!graph.is_null());

            let layout = CString::new("dot").unwrap();
            let result = gvLayout(gvc, graph, layout.as_ptr());
            assert_eq!(result, 0);

            let format = CString::new("svg").unwrap();
            let mut data: *mut c_char = ptr::null_mut();
            let mut len: c_uint = 0;

            let result = gvRenderData(gvc, graph, format.as_ptr(), &mut data, &mut len);
            assert_eq!(result, 0);
            assert!(!data.is_null());
            assert!(len > 0);

            gvFreeRenderData(data);
            gvFreeLayout(gvc, graph);
            agclose(graph);
            gvFreeContext(gvc);
        }
    }
}
