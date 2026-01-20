//! Build script for graphviz-sys
//!
//! This script:
//! 1. Applies patches to use pre-generated Bison/Flex parser files
//! 2. Builds vendored Expat library
//! 3. Builds vendored Graphviz library with static linking
//! 4. Optionally links Cairo/Pango for raster output support

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_dir = manifest_dir.parent().unwrap();

    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let host = env::var("HOST").unwrap();

    // Paths to vendored sources
    let graphviz_src = workspace_dir.join("vendor/graphviz");
    let expat_src = workspace_dir.join("vendor/expat/expat");
    let generated_dir = workspace_dir.join("generated");
    let patches_dir = workspace_dir.join("patches");

    // Check that submodules are initialized
    if !graphviz_src.join("CMakeLists.txt").exists() {
        panic!(
            "Graphviz source not found at {:?}. \
             Please initialize git submodules with: \
             git submodule update --init --recursive",
            graphviz_src
        );
    }

    if !expat_src.join("CMakeLists.txt").exists() {
        panic!(
            "Expat source not found at {:?}. \
             Please initialize git submodules with: \
             git submodule update --init --recursive",
            expat_src
        );
    }

    // Create a working copy of graphviz source to apply patches
    let graphviz_build_src = out_dir.join("graphviz-src");
    if graphviz_build_src.exists() {
        fs::remove_dir_all(&graphviz_build_src).unwrap();
    }
    copy_dir_recursive(&graphviz_src, &graphviz_build_src).expect("Failed to copy Graphviz source");

    // Copy pre-generated parser files
    copy_generated_files(&generated_dir, &graphviz_build_src)
        .expect("Failed to copy generated parser files");

    // Apply patches to use pre-generated parsers
    apply_patches(&patches_dir, &graphviz_build_src);

    // Build Expat first
    let expat_install = build_expat(
        &expat_src,
        &out_dir,
        &target,
        &target_os,
        &target_arch,
        &host,
    );

    // Build Graphviz
    let graphviz_install = build_graphviz(
        &graphviz_build_src,
        &out_dir,
        &expat_install,
        &target,
        &target_os,
        &target_arch,
        &host,
    );

    // Emit linker directives
    emit_link_directives(&graphviz_install, &expat_install, &target_os);

    // Handle Cairo feature
    #[cfg(feature = "cairo")]
    handle_cairo_feature(&target_os);

    // Rerun if sources change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../vendor/graphviz");
    println!("cargo:rerun-if-changed=../vendor/expat");
    println!("cargo:rerun-if-changed=../generated");
    println!("cargo:rerun-if-changed=../patches");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            // Skip .git directory
            if path.file_name().map(|n| n == ".git").unwrap_or(false) {
                continue;
            }
            copy_dir_recursive(&path, &dest_path)?;
        } else if file_type.is_symlink() {
            // For symlinks, try to read the target and copy it
            // If the symlink is relative, it should still work in the new location
            // For simplicity, just copy the symlink target's contents
            match fs::read_link(&path) {
                Ok(target) => {
                    let resolved = if target.is_absolute() {
                        target.clone()
                    } else {
                        path.parent().unwrap().join(&target)
                    };
                    if resolved.exists() {
                        if resolved.is_file() {
                            fs::copy(&resolved, &dest_path)?;
                        } else if resolved.is_dir() {
                            copy_dir_recursive(&resolved, &dest_path)?;
                        }
                    } else {
                        // Create a symlink in the destination
                        #[cfg(unix)]
                        std::os::unix::fs::symlink(&target, &dest_path)?;
                        #[cfg(windows)]
                        {
                            // On Windows, just skip broken symlinks
                        }
                    }
                }
                Err(_) => {
                    // Skip broken symlinks
                }
            }
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn copy_generated_files(generated_dir: &Path, graphviz_src: &Path) -> Result<(), std::io::Error> {
    // Copy cgraph generated files
    let cgraph_gen = generated_dir.join("cgraph");
    let cgraph_dst = graphviz_src.join("lib/cgraph");
    if cgraph_gen.exists() && cgraph_dst.exists() {
        for file in &["grammar.c", "grammar.h", "scan.c"] {
            let src = cgraph_gen.join(file);
            if src.exists() {
                fs::copy(&src, cgraph_dst.join(file))?;
            }
        }
    }

    // Copy common generated files
    let common_gen = generated_dir.join("common");
    let common_dst = graphviz_src.join("lib/common");
    if common_gen.exists() && common_dst.exists() {
        for file in &["htmlparse.c", "htmlparse.h"] {
            let src = common_gen.join(file);
            if src.exists() {
                fs::copy(&src, common_dst.join(file))?;
            }
        }
    }

    // Copy expr generated files
    let expr_gen = generated_dir.join("expr");
    let expr_dst = graphviz_src.join("lib/expr");
    if expr_gen.exists() && expr_dst.exists() {
        for file in &["exparse.c", "exparse.h"] {
            let src = expr_gen.join(file);
            if src.exists() {
                fs::copy(&src, expr_dst.join(file))?;
            }
        }
    }

    Ok(())
}

fn apply_patches(_patches_dir: &Path, graphviz_src: &Path) {
    // Instead of using patch files, directly modify CMakeLists.txt to use pre-generated parsers
    // This is more reliable across platforms

    // Patch the main CMakeLists.txt to make BISON and FLEX optional
    let main_cmake = graphviz_src.join("CMakeLists.txt");
    if main_cmake.exists() {
        let content = fs::read_to_string(&main_cmake).unwrap();
        let patched = patch_main_cmake(&content);
        fs::write(&main_cmake, patched).unwrap();
    }

    // Patch cgraph/CMakeLists.txt
    let cgraph_cmake = graphviz_src.join("lib/cgraph/CMakeLists.txt");
    if cgraph_cmake.exists() {
        let content = fs::read_to_string(&cgraph_cmake).unwrap();
        let patched = patch_cgraph_cmake(&content);
        fs::write(&cgraph_cmake, patched).unwrap();
    }

    // Patch common/CMakeLists.txt
    let common_cmake = graphviz_src.join("lib/common/CMakeLists.txt");
    if common_cmake.exists() {
        let content = fs::read_to_string(&common_cmake).unwrap();
        let patched = patch_common_cmake(&content);
        fs::write(&common_cmake, patched).unwrap();
    }

    // Patch expr/CMakeLists.txt
    let expr_cmake = graphviz_src.join("lib/expr/CMakeLists.txt");
    if expr_cmake.exists() {
        let content = fs::read_to_string(&expr_cmake).unwrap();
        let patched = patch_expr_cmake(&content);
        fs::write(&expr_cmake, patched).unwrap();
    }
}

fn patch_main_cmake(content: &str) -> String {
    // Make BISON and FLEX optional instead of REQUIRED
    // Also disable GTS and other external dependencies
    // We use pre-generated parser files so these tools are not needed at build time
    let mut result = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Change BISON from REQUIRED to OPTIONAL
        if trimmed.contains("set_package_properties(BISON") && trimmed.contains("TYPE REQUIRED") {
            result.push_str("# BISON not required - using pre-generated parser files\n");
            result.push_str("# set_package_properties(BISON PROPERTIES TYPE REQUIRED)\n");
        }
        // Change FLEX from REQUIRED to OPTIONAL
        else if trimmed.contains("set_package_properties(FLEX")
            && trimmed.contains("TYPE REQUIRED")
        {
            result.push_str("# FLEX not required - using pre-generated parser files\n");
            result.push_str("# set_package_properties(FLEX PROPERTIES TYPE REQUIRED)\n");
        }
        // Disable GTS (GNU Triangulated Surface) - requires glib
        else if trimmed.starts_with("find_package(GTS") {
            result.push_str("# GTS disabled for embedded build\n");
            result.push_str("# ");
            result.push_str(line);
            result.push('\n');
        }
        // Disable LTDL lookups that could reference system libraries
        else if trimmed.starts_with("find_package(LTDL") {
            result.push_str("# LTDL disabled for embedded build\n");
            result.push_str("# ");
            result.push_str(line);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

fn patch_cgraph_cmake(content: &str) -> String {
    // Replace BISON_TARGET, FLEX_TARGET, and ADD_FLEX_BISON_DEPENDENCY with static file references
    let mut result = String::new();
    let mut skip_until_empty = false;
    let mut added_replacement = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip BISON_TARGET block
        if trimmed.starts_with("BISON_TARGET(Grammar")
            || trimmed.starts_with("BISON_TARGET(") && trimmed.contains("grammar.y")
        {
            skip_until_empty = true;
            if !added_replacement {
                result.push_str(
                    "# Use pre-generated parser files (no Bison/Flex required at build time)\n",
                );
                result.push_str("set(BISON_Grammar_OUTPUTS\n");
                result.push_str("  ${CMAKE_CURRENT_SOURCE_DIR}/grammar.c\n");
                result.push_str("  ${CMAKE_CURRENT_SOURCE_DIR}/grammar.h\n");
                result.push_str(")\n");
                result.push_str("set(FLEX_Scan_OUTPUTS\n");
                result.push_str("  ${CMAKE_CURRENT_SOURCE_DIR}/scan.c\n");
                result.push_str(")\n\n");
                added_replacement = true;
            }
            continue;
        }

        // Skip FLEX_TARGET block
        if trimmed.starts_with("FLEX_TARGET(Scan")
            || trimmed.starts_with("FLEX_TARGET(") && trimmed.contains("scan.l")
        {
            skip_until_empty = true;
            continue;
        }

        // Skip ADD_FLEX_BISON_DEPENDENCY
        if trimmed.starts_with("ADD_FLEX_BISON_DEPENDENCY") {
            continue;
        }

        // Handle multi-line commands
        if skip_until_empty {
            if trimmed.ends_with(")") {
                skip_until_empty = false;
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

fn patch_common_cmake(content: &str) -> String {
    let mut result = String::new();
    let mut skip_until_close = false;
    let mut added_replacement = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip BISON_TARGET block for HTMLparse
        if trimmed.starts_with("BISON_TARGET(HTMLparse")
            || (trimmed.starts_with("BISON_TARGET(") && trimmed.contains("htmlparse.y"))
        {
            skip_until_close = true;
            if !added_replacement {
                result.push_str(
                    "# Use pre-generated parser files (no Bison required at build time)\n",
                );
                result.push_str("set(BISON_HTMLparse_OUTPUTS\n");
                result.push_str("  ${CMAKE_CURRENT_SOURCE_DIR}/htmlparse.c\n");
                result.push_str(")\n\n");
                added_replacement = true;
            }
            continue;
        }

        if skip_until_close {
            if trimmed.ends_with(")") {
                skip_until_close = false;
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

fn patch_expr_cmake(content: &str) -> String {
    // For expr, the BISON_TARGET is multi-line, so we need to replace the whole block
    // Original pattern:
    // BISON_TARGET(
    //   Exparse
    //   ...
    // )
    let mut result = String::new();
    let mut skip_until_close = false;
    let mut paren_depth = 0;
    let mut added_replacement = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Start of BISON_TARGET block (could just be "BISON_TARGET(" on its own line)
        if trimmed.starts_with("BISON_TARGET(") {
            skip_until_close = true;
            // Count parentheses on this line
            paren_depth = trimmed.matches('(').count() as i32 - trimmed.matches(')').count() as i32;
            if paren_depth <= 0 {
                skip_until_close = false;
            }
            if !added_replacement {
                result.push_str(
                    "# Use pre-generated parser files (no Bison required at build time)\n",
                );
                result.push_str("set(BISON_Exparse_OUTPUTS\n");
                result.push_str("  ${CMAKE_CURRENT_SOURCE_DIR}/exparse.c\n");
                result.push_str("  ${CMAKE_CURRENT_SOURCE_DIR}/exparse.h\n");
                result.push_str(")\n\n");
                added_replacement = true;
            }
            continue;
        }

        if skip_until_close {
            paren_depth += trimmed.matches('(').count() as i32;
            paren_depth -= trimmed.matches(')').count() as i32;
            if paren_depth <= 0 {
                skip_until_close = false;
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

fn build_expat(
    src: &Path,
    out_dir: &Path,
    target: &str,
    target_os: &str,
    target_arch: &str,
    host: &str,
) -> PathBuf {
    let mut config = cmake::Config::new(src);

    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("EXPAT_BUILD_TOOLS", "OFF")
        .define("EXPAT_BUILD_EXAMPLES", "OFF")
        .define("EXPAT_BUILD_TESTS", "OFF")
        .define("EXPAT_BUILD_DOCS", "OFF")
        .define("EXPAT_SHARED_LIBS", "OFF")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .out_dir(out_dir.join("expat-build"));

    // On Windows MSVC, use static CRT to match Graphviz build
    if target.contains("msvc") {
        config.define("EXPAT_MSVC_STATIC_CRT", "ON");
    }

    configure_cmake_for_target(&mut config, target, target_os, target_arch, host);

    config.build()
}

fn build_graphviz(
    src: &Path,
    out_dir: &Path,
    expat_install: &Path,
    target: &str,
    target_os: &str,
    target_arch: &str,
    host: &str,
) -> PathBuf {
    let mut config = cmake::Config::new(src);

    // Core build options
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        // Disable CLI tools - we only want the library
        .define("GRAPHVIZ_CLI", "OFF")
        // Disable plugin loading - we'll statically link plugins
        .define("ENABLE_LTDL", "OFF")
        // Point to our built expat
        .define("WITH_EXPAT", "ON")
        .define(
            "EXPAT_INCLUDE_DIR",
            expat_install.join("include").to_str().unwrap(),
        )
        .define("EXPAT_LIBRARY", find_expat_lib(expat_install, target_os))
        // Disable optional features we don't need
        .define("WITH_GVEDIT", "OFF")
        .define("WITH_SMYRNA", "OFF")
        .define("WITH_ZLIB", "OFF")
        // Disable features that require C++ (avoiding C++ stdlib issues)
        .define("with_ipsepcola", "OFF")
        // Disable all SWIG language bindings
        .define("ENABLE_TCL", "OFF")
        .define("ENABLE_SWIG", "OFF")
        .define("ENABLE_SHARP", "OFF")
        .define("ENABLE_D", "OFF")
        .define("ENABLE_GO", "OFF")
        .define("ENABLE_GUILE", "OFF")
        .define("ENABLE_JAVA", "OFF")
        .define("ENABLE_JAVASCRIPT", "OFF")
        .define("ENABLE_LUA", "OFF")
        .define("ENABLE_PERL", "OFF")
        .define("ENABLE_PHP", "OFF")
        .define("ENABLE_PYTHON", "OFF")
        .define("ENABLE_R", "OFF")
        .define("ENABLE_RUBY", "OFF")
        .out_dir(out_dir.join("graphviz-build"));

    // Cairo/Pango handling
    #[cfg(feature = "cairo")]
    {
        // Let CMake auto-detect Cairo/Pango
    }

    #[cfg(not(feature = "cairo"))]
    {
        // Use CMAKE_DISABLE_FIND_PACKAGE_* to prevent CMake from finding these
        // (standard CMake mechanism to skip find_package() calls)
        config.define("CMAKE_DISABLE_FIND_PACKAGE_PANGOCAIRO", "TRUE");
        config.define("CMAKE_DISABLE_FIND_PACKAGE_CAIRO", "TRUE");
        config.define("CMAKE_DISABLE_FIND_PACKAGE_GD", "TRUE");
        config.define("WITH_GDK", "OFF");
        config.define("WITH_RSVG", "OFF");
    }

    configure_cmake_for_target(&mut config, target, target_os, target_arch, host);

    config.build()
}

fn configure_cmake_for_target(
    config: &mut cmake::Config,
    target: &str,
    target_os: &str,
    target_arch: &str,
    host: &str,
) {
    match target_os {
        "windows" => {
            if target.contains("msvc") {
                config.static_crt(true);
            }
        }
        "macos" => {
            let arch = if target_arch == "aarch64" {
                "arm64"
            } else {
                "x86_64"
            };
            config.define("CMAKE_OSX_ARCHITECTURES", arch);
        }
        "linux" => {
            // Cross-compilation setup for Linux arm64
            if target_arch == "aarch64" && host.contains("x86_64") {
                config.define("CMAKE_SYSTEM_NAME", "Linux");
                config.define("CMAKE_SYSTEM_PROCESSOR", "aarch64");
                config.define("CMAKE_C_COMPILER", "aarch64-linux-gnu-gcc");
                config.define("CMAKE_CXX_COMPILER", "aarch64-linux-gnu-g++");
            }
        }
        _ => {}
    }
}

fn find_expat_lib(expat_install: &Path, target_os: &str) -> String {
    let lib_dir = expat_install.join("lib");

    // On Windows MSVC with EXPAT_MSVC_STATIC_CRT=ON, the library is named libexpatMT.lib
    let lib_name = match target_os {
        "windows" => "libexpatMT.lib",
        _ => "libexpat.a",
    };

    // Also check lib64 directory
    let lib_path = lib_dir.join(lib_name);
    if lib_path.exists() {
        return lib_path.to_str().unwrap().to_string();
    }

    let lib64_path = expat_install.join("lib64").join(lib_name);
    if lib64_path.exists() {
        return lib64_path.to_str().unwrap().to_string();
    }

    // Return the expected path anyway, CMake will error if not found
    lib_path.to_str().unwrap().to_string()
}

fn emit_link_directives(graphviz_install: &Path, expat_install: &Path, target_os: &str) {
    // Add library search paths
    // The main install directory
    println!(
        "cargo:rustc-link-search=native={}/lib",
        graphviz_install.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib64",
        graphviz_install.display()
    );

    // Plugins are built in the build directory, not installed
    let build_dir = graphviz_install.join("build");
    println!(
        "cargo:rustc-link-search=native={}/plugin/dot_layout",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/plugin/neato_layout",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/plugin/core",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/plugin/pango",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/plugin/quartz",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/plugin/kitty",
        build_dir.display()
    );

    // Some internal libraries are also in the build dir
    println!(
        "cargo:rustc-link-search=native={}/lib/dotgen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/neatogen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/fdpgen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/sfdpgen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/twopigen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/patchwork",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/circogen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/osage",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/common",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/label",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/pack",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/ortho",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/rbtree",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/sparse",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/edgepaint",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/mingle",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/sfio",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/ast",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/expr",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/util",
        build_dir.display()
    );

    // Expat directories
    println!(
        "cargo:rustc-link-search=native={}/lib",
        expat_install.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib64",
        expat_install.display()
    );

    // Link Graphviz libraries
    // List libraries in reverse dependency order (dependencies last)

    // Expat library name differs on Windows with static CRT
    match target_os {
        "windows" => println!("cargo:rustc-link-lib=static=expatMT"),
        _ => println!("cargo:rustc-link-lib=static=expat"),
    }
    println!("cargo:rustc-link-lib=static=util");
    println!("cargo:rustc-link-lib=static=ast");
    println!("cargo:rustc-link-lib=static=sfio");
    println!("cargo:rustc-link-lib=static=cdt");
    println!("cargo:rustc-link-lib=static=cgraph");
    println!("cargo:rustc-link-lib=static=pathplan");
    println!("cargo:rustc-link-lib=static=xdot");
    println!("cargo:rustc-link-lib=static=common");
    println!("cargo:rustc-link-lib=static=label");
    println!("cargo:rustc-link-lib=static=pack");
    println!("cargo:rustc-link-lib=static=ortho");
    println!("cargo:rustc-link-lib=static=rbtree");
    println!("cargo:rustc-link-lib=static=sparse");
    println!("cargo:rustc-link-lib=static=expr");
    println!("cargo:rustc-link-lib=static=dotgen");
    println!("cargo:rustc-link-lib=static=neatogen");
    println!("cargo:rustc-link-lib=static=fdpgen");
    println!("cargo:rustc-link-lib=static=sfdpgen");
    println!("cargo:rustc-link-lib=static=twopigen");
    println!("cargo:rustc-link-lib=static=patchwork");
    println!("cargo:rustc-link-lib=static=circogen");
    println!("cargo:rustc-link-lib=static=osage");
    println!("cargo:rustc-link-lib=static=gvc");
    println!("cargo:rustc-link-lib=static=gvplugin_core");
    println!("cargo:rustc-link-lib=static=gvplugin_dot_layout");
    println!("cargo:rustc-link-lib=static=gvplugin_neato_layout");

    // Platform-specific system libraries
    match target_os {
        "linux" => {
            println!("cargo:rustc-link-lib=m");
            println!("cargo:rustc-link-lib=pthread");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=m");
        }
        "windows" => {
            // Windows system libraries if needed
        }
        _ => {}
    }

    // Emit cfg flags for conditional compilation
    println!("cargo:rustc-check-cfg=cfg(has_cairo)");

    #[cfg(feature = "cairo")]
    println!("cargo:rustc-cfg=has_cairo");
}

#[cfg(feature = "cairo")]
fn handle_cairo_feature(target_os: &str) {
    match target_os {
        "windows" => {
            // Use vcpkg on Windows
            #[cfg(windows)]
            {
                let lib = vcpkg::find_package("cairo").expect("Cairo not found via vcpkg");
                for path in &lib.link_paths {
                    println!("cargo:rustc-link-search=native={}", path.display());
                }

                if let Ok(pango) = vcpkg::find_package("pango") {
                    for path in &pango.link_paths {
                        println!("cargo:rustc-link-search=native={}", path.display());
                    }
                }
            }
        }
        _ => {
            // Use pkg-config on Unix-like systems
            if let Ok(cairo) = pkg_config::probe_library("cairo") {
                for path in &cairo.link_paths {
                    println!("cargo:rustc-link-search=native={}", path.display());
                }
            }

            if let Ok(pangocairo) = pkg_config::probe_library("pangocairo") {
                for path in &pangocairo.link_paths {
                    println!("cargo:rustc-link-search=native={}", path.display());
                }
            }
        }
    }

    // Link Cairo and Pango
    println!("cargo:rustc-link-lib=cairo");
    println!("cargo:rustc-link-lib=pango-1.0");
    println!("cargo:rustc-link-lib=pangocairo-1.0");

    // On Linux, we need pangoft2 for Fontconfig-based font support
    // macOS uses CoreText backend which doesn't need this
    if target_os == "linux" {
        println!("cargo:rustc-link-lib=pangoft2-1.0");
    }

    println!("cargo:rustc-link-lib=gvplugin_pango");
}
