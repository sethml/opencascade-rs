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

use std::path::PathBuf;

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

    // Find the generated wrappers.cpp file
    let wrappers_cpp = gen_dir.join("wrappers.cpp");
    if !wrappers_cpp.exists() {
        panic!("Generated wrappers.cpp not found at {}. Run the binding generator first.", wrappers_cpp.display());
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

    // Build with cc
    let mut build = cc::Build::new();
    build.file(&wrappers_cpp);
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
        .include(&occt_config.include_dir)
        .include(&gen_dir)
        .debug(false)
        .compile("opencascade_sys_wrapper");

    println!("cargo:rustc-link-lib=static=opencascade_sys_wrapper");

    // Rerun if generated or manual files change
    println!("cargo:rerun-if-changed=generated");
    println!("cargo:rerun-if-changed={}", wrappers_cpp.display());
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

