use std::{
    env::var,
    path::{Path, PathBuf},
};

const LIB_DIR: &str = "lib";
const INCLUDE_DIR: &str = "include";

/// Name of the stamp file (written under [`occt_path`]) recording which OCCT
/// source revision the built artifacts correspond to. See [`occt_source_stamp`].
const STAMP_FILE: &str = ".occt-src-stamp";

/// Get the path to the OCCT library installation directory to be
/// used in build scripts.
///
/// Only valid during build (`cargo clean` removes these files).
pub fn occt_path() -> PathBuf {
    // moves the output into target/TARGET/OCCT
    // this way its less likely to be rebuilt without a cargo clean
    Path::new(&var("OUT_DIR").expect("missing OUT_DIR")).join("../../../../OCCT")
}

/// A short identifier for the OCCT source the current bindings target.
///
/// This is the checked-out commit of the vendored OCCT submodule. It is written
/// into a stamp file after a successful build (see [`build_occt`]) and compared
/// on subsequent builds so that bumping the submodule (e.g. a version upgrade)
/// invalidates a stale prebuilt tree instead of being silently reused. Falls
/// back to the empty string if git is unavailable, which degrades to the old
/// "marker file exists" behaviour.
pub fn occt_source_stamp() -> String {
    let src = env!("OCCT_SRC_DIR");
    std::process::Command::new("git")
        .args(["-C", src, "rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_default()
}

/// Whether the prebuilt OCCT under [`occt_path`] is present and matches the
/// current source revision, so a rebuild can be skipped.
pub fn occt_is_up_to_date() -> bool {
    let path = occt_path();
    let marker_lib = path.join(LIB_DIR).join("libTKernel.a");
    if !marker_lib.exists() {
        return false;
    }
    match std::fs::read_to_string(path.join(STAMP_FILE)) {
        Ok(stamp) => stamp.trim() == occt_source_stamp(),
        // Marker present but no stamp: an older or externally-produced build we
        // can't verify — rebuild to be safe.
        Err(_) => false,
    }
}

/// Build the OCCT library.
pub fn build_occt() {
    cmake::Config::new(Path::new(env!("OCCT_SRC_DIR")))
        .define("BUILD_LIBRARY_TYPE", "Static")
        .define("BUILD_MODULE_ApplicationFramework", "FALSE")
        .define("BUILD_MODULE_Draw", "FALSE")
        .define("BUILD_MODULE_Visualization", "FALSE")
        .define("USE_D3D", "FALSE")
        .define("USE_DRACO", "FALSE")
        .define("USE_EIGEN", "FALSE")
        .define("USE_FFMPEG", "FALSE")
        .define("USE_FREEIMAGE", "FALSE")
        .define("USE_FREETYPE", "FALSE")
        .define("USE_GLES2", "FALSE")
        .define("USE_OPENGL", "FALSE")
        .define("USE_OPENVR", "FALSE")
        .define("USE_RAPIDJSON", "FALSE")
        .define("USE_TBB", "FALSE")
        .define("USE_TCL", "FALSE")
        .define("USE_TK", "FALSE")
        .define("USE_VTK", "FALSE")
        .define("USE_XLIB", "FALSE")
        .define("INSTALL_DIR_LIB", LIB_DIR)
        .define("INSTALL_DIR_INCLUDE", INCLUDE_DIR)
        .profile("Release")
        .out_dir(occt_path())
        .build();

    // Record which source revision this build corresponds to, so a later build
    // with a bumped submodule detects the mismatch and rebuilds. See
    // `occt_is_up_to_date`.
    let _ = std::fs::write(occt_path().join(STAMP_FILE), occt_source_stamp());
}
