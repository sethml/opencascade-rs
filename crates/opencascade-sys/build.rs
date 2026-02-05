//! Build script for opencascade-sys
//!
//! This script:
//! 1. Uses pre-generated code from the `generated/` directory
//! 2. Builds with cxx_build
//!
//! To regenerate the bindings, run:
//!   DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib cargo run -p opencascade-binding-generator -- \
//!     -I target/OCCT/include \
//!     -o crates/opencascade-sys/generated \
//!     $(cat crates/opencascade-sys/headers.txt | grep -v '^#' | grep -v '^$' | sed 's|^|target/OCCT/include/|')

use std::path::PathBuf;

/// Minimum compatible version of OpenCASCADE library (major, minor)
const OCCT_VERSION: (u8, u8) = (7, 8);

/// The list of used OpenCASCADE libraries which needs to be linked with.
const OCCT_LIBS: &[&str] = &[
    "TKMath",
    "TKernel",
    "TKDE",
    "TKFeat",
    "TKGeomBase",
    "TKG2d",
    "TKG3d",
    "TKTopAlgo",
    "TKGeomAlgo",
    "TKBRep",
    "TKPrim",
    "TKDESTEP",
    "TKDEIGES",
    "TKDESTL",
    "TKMesh",
    "TKShHealing",
    "TKFillet",
    "TKBool",
    "TKBO",
    "TKOffset",
    "TKXSBase",
    "TKCAF",
    "TKLCAF",
    "TKXCAF",
];

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let gen_dir = manifest_dir.join("generated");
    let src_dir = manifest_dir.join("src");

    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    // Detect OCCT installation
    let occt_config = OcctConfig::detect();

    println!("cargo:rustc-link-search=native={}", occt_config.library_dir.to_str().unwrap());

    let lib_type = if occt_config.is_dynamic { "dylib" } else { "static" };
    for lib in OCCT_LIBS {
        println!("cargo:rustc-link-lib={lib_type}={lib}");
    }

    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    // Find all generated .rs files in generated/ directory
    // In unified mode, only ffi.rs contains the CXX bridge
    let ffi_rs_path = gen_dir.join("ffi.rs");
    let rust_files: Vec<PathBuf> = if ffi_rs_path.exists() {
        // Unified mode: only ffi.rs has the CXX bridge
        vec![ffi_rs_path]
    } else {
        // Legacy per-module mode: all .rs files have CXX bridges
        std::fs::read_dir(&gen_dir)
            .expect("Failed to read generated directory")
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "rs" && path.file_name()? != "lib.rs" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    };

    if rust_files.is_empty() {
        panic!("No generated .rs files found in {}. Run the binding generator first.", gen_dir.display());
    }

    // Build with CXX
    let mut build = cxx_build::bridges(&rust_files);

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", "TRUE");
    }

    build
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include(&occt_config.include_dir)
        .include(&gen_dir)
        .include(&src_dir)
        .compile("opencascade_sys_wrapper");

    println!("cargo:rustc-link-lib=static=opencascade_sys_wrapper");

    // Rerun if generated files change
    println!("cargo:rerun-if-changed=generated");
    for rs_file in &rust_files {
        println!("cargo:rerun-if-changed={}", rs_file.display());
    }
    // Rerun if common.hxx changes
    println!("cargo:rerun-if-changed={}", src_dir.join("common.hxx").display());
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
        println!("cargo:warning=OcctConfig::detect() called");

        // Add path to builtin OCCT
        #[cfg(feature = "builtin")]
        {
            println!("cargo:warning=Builtin feature is enabled");

            occt_sys::build_occt();
            let builtin_occt_path = occt_sys::occt_path();
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

            println!("cargo:warning=Using builtin OCCT from {}", builtin_occt_path.display());
            println!("cargo:warning=  Include: {}", include_dir.display());
            println!("cargo:warning=  Library: {}", library_dir.display());

            return Self {
                include_dir,
                library_dir,
                is_dynamic: false  // Built OCCT is static
            };
        }

        // Non-builtin: use cmake detection
        #[cfg(not(feature = "builtin"))]
        {
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

