fn main() {
    pyo3_build_config::use_pyo3_cfgs();
    
    // For cdylib builds on Linux, we need to ensure Graphviz symbols are included
    // We allow multiple definitions since Graphviz libraries may have some duplicates
    #[cfg(target_os = "linux")]
    {
        // Allow duplicate symbols (Graphviz libraries have some internal duplicates)
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }
}
