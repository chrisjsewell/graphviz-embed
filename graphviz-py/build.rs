fn main() {
    pyo3_build_config::use_pyo3_cfgs();
    
    // For Linux cdylib builds, we need to force whole-archive linking of Graphviz libraries
    // We do this by finding the static library paths and adding them with linker arguments
    #[cfg(target_os = "linux")]
    {
        use std::env;
        use std::path::PathBuf;
        
        // Find the graphviz-sys OUT_DIR
        let out_dir = env::var("DEP_GRAPHVIZ_OUT_DIR");
        
        if let Ok(out_dir) = out_dir {
            let lib_dir = PathBuf::from(&out_dir).join("graphviz-build/lib");
            let build_lib_dir = PathBuf::from(&out_dir).join("graphviz-build/build/lib");
            let plugin_dir = PathBuf::from(&out_dir).join("graphviz-build/build/plugin");
            
            // Add rdynamic to export all symbols
            println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
            
            // Use start-group/end-group to handle circular dependencies
            println!("cargo:rustc-link-arg=-Wl,--start-group");
            println!("cargo:rustc-link-arg=-Wl,--whole-archive");
            
            // Add all graphviz libraries in dependency order (reverse of link order)
            let libs = vec![
                // Plugins (in build/plugin subdirs)
                ("gvplugin_neato_layout", plugin_dir.join("neato_layout")),
                ("gvplugin_dot_layout", plugin_dir.join("dot_layout")),
                ("gvplugin_core", plugin_dir.join("core")),
                // GVC (in lib/)
                ("gvc", lib_dir.clone()),
                // Layout engines (in build/lib subdirs)
                ("osage", build_lib_dir.join("osage")),
                ("circogen", build_lib_dir.join("circogen")),
                ("patchwork", build_lib_dir.join("patchwork")),
                ("twopigen", build_lib_dir.join("twopigen")),
                ("sfdpgen", build_lib_dir.join("sfdpgen")),
                ("fdpgen", build_lib_dir.join("fdpgen")),
                ("neatogen", build_lib_dir.join("neatogen")),
                ("dotgen", build_lib_dir.join("dotgen")),
                // Mid-level libraries (in build/lib subdirs)
                ("expr", build_lib_dir.join("expr")),
                ("sparse", build_lib_dir.join("sparse")),
                ("rbtree", build_lib_dir.join("rbtree")),
                ("ortho", build_lib_dir.join("ortho")),
                ("pack", build_lib_dir.join("pack")),
                ("label", build_lib_dir.join("label")),
                ("common", build_lib_dir.join("common")),
                ("xdot", lib_dir.clone()),
                ("pathplan", lib_dir.clone()),
                ("cgraph", lib_dir.clone()),
                ("cdt", lib_dir.clone()),
                // Low-level utilities (in build/lib subdirs)
                ("sfio", build_lib_dir.join("sfio")),
                ("ast", build_lib_dir.join("ast")),
                ("util", build_lib_dir.join("util")),
            ];
            
            for (lib, dir) in &libs {
                let lib_path = dir.join(format!("lib{}.a", lib));
                if lib_path.exists() {
                    println!("cargo:rustc-link-arg={}", lib_path.display());
                }
            }
            
            // Close whole-archive
            println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
            println!("cargo:rustc-link-arg=-Wl,--end-group");
        }
    }
}
