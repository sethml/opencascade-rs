//! Build script for opencascade-sys
//!
//! This script:
//! 1. Uses pre-generated code from the `generated/` directory
//! 2. Compiles wrappers.cpp with cc
//!
//! To regenerate the bindings, run:
//!   DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib cargo run -p opencascade-binding-generator -- \
//!     -I target/OCCT/include \
//!     -o crates/opencascade-sys/generated \
//!     $(cat crates/opencascade-sys/headers.txt | grep -v '^#' | grep -v '^$' | sed 's|^|target/OCCT/include/|')

use std::io::Write;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let gen_dir = manifest_dir.join("generated");

    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    // Detect OCCT installation
    let occt_config = OcctConfig::detect();

    println!("cargo:rustc-link-search=native={}", occt_config.library_dir.to_str().unwrap());
    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    // Find generated wrappers*.cpp files (may be split by toolkit)
    let mut wrapper_cpp_files: Vec<PathBuf> = std::fs::read_dir(&gen_dir)
        .expect("Failed to read generated/ directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().map_or(false, |e| e == "cpp")
                && path.file_stem().map_or(false, |s| s.to_string_lossy().starts_with("wrappers"))
            {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    wrapper_cpp_files.sort();
    if wrapper_cpp_files.is_empty() {
        panic!("No generated wrappers*.cpp files found in {}. Run the binding generator first.", gen_dir.display());
    }

    // Derive OCCT libraries to link from generated wrapper file names.
    // Each wrappers_TK<name>.cpp corresponds to an OCCT library named TK<name>.
    let lib_type = if occt_config.is_dynamic { "dylib" } else { "static" };
    for cpp_file in &wrapper_cpp_files {
        if let Some(stem) = cpp_file.file_stem().and_then(|s| s.to_str()) {
            if let Some(lib_name) = stem.strip_prefix("wrappers_") {
                if lib_name.starts_with("TK") {
                    println!("cargo:rustc-link-lib={lib_type}={lib_name}");
                }
            }
        }
    }

    // Find manual wrapper .cpp files
    let manual_dir = manifest_dir.join("manual");
    let manual_cpp_files: Vec<PathBuf> = if manual_dir.exists() {
        std::fs::read_dir(&manual_dir)
            .expect("Failed to read manual/ directory")
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.extension().map_or(false, |e| e == "cpp") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    // Combine all per-toolkit wrappers into a single compilation unit.
    // This reduces total CPU time by avoiding redundant OCCT header parsing
    // across 50 separate files.
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let combined_cpp = out_dir.join("wrappers_combined.cpp");
    {
        let mut f = std::fs::File::create(&combined_cpp)
            .expect("Failed to create combined wrapper file");
        writeln!(f, "// Auto-generated: combines all per-toolkit wrappers into one TU").unwrap();
        for cpp_file in &wrapper_cpp_files {
            let abs = std::fs::canonicalize(cpp_file)
                .unwrap_or_else(|_| cpp_file.clone());
            writeln!(f, "#include \"{}\"", abs.display()).unwrap();
        }
    }

    // Build with cc
    let mut build = cc::Build::new();
    build.file(&combined_cpp);
    for cpp_file in &manual_cpp_files {
        build.file(cpp_file);
    }

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", "TRUE");
    }

    build
        .cpp(true)
        .flag_if_supported("-std=c++14")
        // Generated wrappers use extern "C" functions that return C++ reference types
        // (e.g. const TopoDS_Shape&). This is technically incompatible with C linkage
        // but works fine for Rust FFI where both sides agree on calling convention.
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-return-type-c-linkage")
        // OCCT classes math_FunctionSample, Poly_MakeLoops, Poly_MakeLoops2D,
        // and Poly_MakeLoops3D have virtual functions but non-virtual destructors.
        // Our generated destructors always delete through the concrete type pointer
        // (e.g. delete static_cast<Poly_MakeLoops3D*>(ptr)), never through a base
        // pointer, so the non-virtual destructor is safe. OwnedPtr<T> ensures the
        // static type always matches the dynamic type.
        .flag_if_supported("-Wno-delete-non-abstract-non-virtual-dtor")
        .flag_if_supported("-Wno-delete-abstract-non-virtual-dtor")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include(&gen_dir)
        .debug(false);

    // Treat the OCCT include directory as a system header path so the compiler
    // suppresses warnings from third-party OCCT code (e.g. sprintf deprecation).
    // MSVC does not support -isystem; on GCC/Clang it is standard.
    if target.contains("msvc") {
        build.include(&occt_config.include_dir);
    } else {
        build.flag("-isystem");
        build.flag(occt_config.include_dir.to_str().unwrap());
    }

    build.compile("opencascade_sys_wrapper");

    println!("cargo:rustc-link-lib=static=opencascade_sys_wrapper");

    // Rerun if generated or manual files change
    println!("cargo:rerun-if-changed=generated");
    for cpp_file in &wrapper_cpp_files {
        println!("cargo:rerun-if-changed={}", cpp_file.display());
    }
    println!("cargo:rerun-if-changed=manual");
    for cpp_file in &manual_cpp_files {
        println!("cargo:rerun-if-changed={}", cpp_file.display());
    }
}

struct OcctConfig {
    include_dir: PathBuf,
    library_dir: PathBuf,
    is_dynamic: bool,
}

impl OcctConfig {
    /// Find OpenCASCADE library using cmake
    fn detect() -> Self {
        println!("cargo:rerun-if-env-changed=DEP_OCCT_ROOT");

        // Add path to builtin OCCT
        #[cfg(feature = "builtin")]
        {
            let builtin_occt_path = occt_sys::occt_path();

            // Skip building if OCCT appears to be already installed
            // Check for a key library file to determine if build is needed
            let marker_lib = builtin_occt_path.join("lib").join("libTKernel.a");
            if !marker_lib.exists() {
                occt_sys::build_occt();
            }
            std::env::set_var("DEP_OCCT_ROOT", builtin_occt_path.as_os_str());

            // With builtin feature, skip cmake detection and use built OCCT directly
            // Headers can be in either build/include (during build) or include (after install)
            let include_dir = if builtin_occt_path.join("build").join("include").exists() {
                builtin_occt_path.join("build").join("include")
            } else if builtin_occt_path.join("include").exists() {
                builtin_occt_path.join("include")
            } else {
                panic!("Builtin OCCT include directory not found at {:?}", builtin_occt_path);
            };

            let library_dir = if builtin_occt_path.join("lib").exists() {
                builtin_occt_path.join("lib")
            } else if builtin_occt_path.join("build").join("lib").exists() {
                builtin_occt_path.join("build").join("lib")
            } else {
                panic!("Builtin OCCT library directory not found at {:?}", builtin_occt_path);
            };

            return Self {
                include_dir,
                library_dir,
                is_dynamic: false  // Built OCCT is static
            };
        }

        // Non-builtin: use cmake detection
        #[cfg(not(feature = "builtin"))]
        {
            /// Minimum compatible version of OpenCASCADE library (major, minor)
            const OCCT_VERSION: (u8, u8) = (7, 8);

            let dst =
                std::panic::catch_unwind(|| cmake::Config::new("OCCT").register_dep("occt").build());
            let dst = dst.expect("Pre-installed OpenCASCADE library not found. You can use `builtin` feature if you do not want to install OCCT libraries system-wide.");

            let cfg = std::fs::read_to_string(dst.join("share").join("occ_info.txt"))
                .expect("Something went wrong when detecting OpenCASCADE library.");

            let mut version_major: Option<u8> = None;
            let mut version_minor: Option<u8> = None;
            let mut include_dir: Option<PathBuf> = None;
            let mut library_dir: Option<PathBuf> = None;
            let mut is_dynamic: bool = false;

            for line in cfg.lines() {
                if let Some((var, val)) = line.split_once('=') {
                    match var {
                        "VERSION_MAJOR" => version_major = val.parse().ok(),
                        "VERSION_MINOR" => version_minor = val.parse().ok(),
                        "INCLUDE_DIR" => include_dir = val.parse().ok(),
                        "LIBRARY_DIR" => library_dir = val.parse().ok(),
                        "BUILD_SHARED_LIBS" => is_dynamic = val == "ON",
                        _ => (),
                    }
                }
            }

            if let (Some(version_major), Some(version_minor), Some(include_dir), Some(library_dir)) =
                (version_major, version_minor, include_dir, library_dir)
            {
                if version_major != OCCT_VERSION.0 || version_minor < OCCT_VERSION.1 {
                    panic!("Pre-installed OpenCASCADE library found but version is not met (found {}.{} but {}.{} required). Please provide required version or use `builtin` feature.",
                           version_major, version_minor, OCCT_VERSION.0, OCCT_VERSION.1);
                }

                Self { include_dir, library_dir, is_dynamic }
            } else {
                panic!("OpenCASCADE library found but something wrong with config.");
            }
        }
    }
}
