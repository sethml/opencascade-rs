//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Convert a C++ template spelling to a safe Rust/C++ identifier.
/// e.g. `BVH_Builder<double, 3>` → `BVH_Builder_double_3`
/// e.g. `NCollection_Shared<NCollection_List<gp_Pnt2d>>` → `NCollection_Shared_NCollection_List_gp_Pnt2d`
/// e.g. `NCollection_Sequence<const gp_Pnt2d *>` → `NCollection_Sequence_constgp_Pnt2d_ptr`
pub fn template_alias_name(template_spelling: &str) -> String {
    template_spelling
        .replace("::", "_")
        .replace('<', "_")
        .replace('>', "")
        .replace(", ", "_")
        .replace(',', "_")
        .replace('*', "_ptr")
        .replace('&', "_ref")
        .replace(' ', "")
        .trim_end_matches('_')
        .to_string()
}

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// Include headers from these OCCT modules.
    /// Supports glob patterns: "*" matches all modules, "Geom*" matches
    /// Geom, GeomAdaptor, GeomAPI, etc.
    #[serde(default)]
    pub modules: Vec<String>,

    /// Exclude entire modules from binding generation.
    /// Applied after `modules` expansion (including glob matching).
    /// Supports glob patterns.
    #[serde(default)]
    pub exclude_modules: Vec<String>,

    /// Exclude specific headers, even if their module is included.
    #[serde(default)]
    pub exclude_headers: Vec<String>,

    /// Include specific individual headers (from modules not fully listed in `modules`).
    #[serde(default)]
    pub include_headers: Vec<String>,

    /// Exclude specific methods/constructors from binding generation.
    /// Format: "ClassName::MethodName" for instance/static methods,
    /// or "ClassName::ClassName" for constructors (C++ constructor naming convention).
    /// Methods matching these patterns will be skipped during codegen,
    /// even though the rest of the class is bound.
    #[serde(default)]
    pub exclude_methods: Vec<String>,

    /// Methods that are known to produce ambiguous overload errors in generated C++ wrappers.
    /// Format: "ClassName::MethodName".
    #[serde(default)]
    pub ambiguous_methods: Vec<String>,

    /// Non-allocatable classes: suppress constructors and destructors (including
    /// CppDeletable/ToOwned), but keep the opaque struct and all methods.
    /// Use for classes with protected/hidden operator new/delete that can still
    /// be used via pointers obtained from other APIs.
    /// For nested types, use the C++ qualified name: "Parent::Nested".
    #[serde(default)]
    pub non_allocatable_classes: Vec<String>,

    /// Opaque types defined in manual/ files but referenced by auto-generated bindings.
    /// The generator adds these to the known class set so methods using them
    /// aren't skipped as "unknown type".
    /// Format: `TypeName = { header = "Header.hxx" }`
    #[serde(default)]
    pub manual_types: std::collections::HashMap<String, ManualType>,

    /// Template instantiation aliases: declare specific C++ template instantiations
    /// as opaque types so methods using them aren't skipped as "unknown type".
    /// The generator creates C++ typedefs, Rust opaque types, and Handle wrappers.
    /// Format: `"Template<Args>" = { header = "Header.hxx", module = "Module", handle = true }`
    #[serde(default)]
    pub template_instantiations: std::collections::HashMap<String, TemplateInstantiation>,

    /// Canonical OCCT type spellings that should be rewritten to public aliases during parsing.
    /// This is used when clang reports internal template spellings that differ from public typedef names.
    /// Format: "CanonicalTypeSpelling" = "PublicAliasName"
    #[serde(default)]
    pub occt_alias_type_overrides: std::collections::HashMap<String, String>,

    /// C++ typedef names that are platform-dependent pointer types and should map to
    /// `*mut std::ffi::c_void` at the FFI boundary (making methods unsafe).
    /// Use for typedefs like `Aspect_RenderingContext` whose canonical form differs
    /// by platform (void* on Linux, NSOpenGLContext* on macOS, EAGLContext* on iOS).
    #[serde(default)]
    pub void_pointer_types: Vec<String>,

    /// Configuration for splitting generated output files.
    #[serde(default)]
    pub split: SplitConfig,

    /// Constructors that should be marked `unsafe` with a safety note.
    /// Format: `"ClassName" = "Safety note explaining why and what to use instead."`
    /// All constructors (including convenience wrappers) for the class will be `unsafe fn`.
    #[serde(default)]
    pub unsafe_constructors: std::collections::HashMap<String, String>,

    /// Skip OCCT 8.0.0 deprecated backward-compatibility alias headers (those marked
    /// with `Standard_HEADER_DEPRECATED` and the standard "Deprecated typedef for
    /// backward compatibility" banner, e.g. `TColgp_HArray1OfPnt.hxx` which is now
    /// just `typedef NCollection_HArray1<gp_Pnt>`). Generating these produces
    /// duplicate/mis-toolkited bindings for the underlying canonical types, so they
    /// are excluded by default. Defaults to true.
    #[serde(default = "default_true")]
    pub exclude_deprecated_headers: bool,
}

fn default_true() -> bool {
    true
}

/// Configuration for splitting generated output files for build parallelization.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct SplitConfig {
    /// Strategy for C++ wrapper splitting: "none" (default) or "toolkit".
    /// When "toolkit", wrappers.cpp is split into one file per OCCT toolkit (TKernel,
    /// TKMath, etc.) enabling parallel C++ compilation.
    #[serde(default)]
    pub cpp_split: Option<String>,

    /// Strategy for Rust FFI splitting: "none" (default) or "toolkit".
    /// When "toolkit", ffi.rs is split into one file per toolkit enabling parallel
    /// Rust compilation.
    #[serde(default)]
    pub ffi_split: Option<String>,

    /// Path to OCCT source directory containing TK*/PACKAGES files.
    /// Used to discover toolkit→module mappings for splitting.
    /// Relative to the config file directory.
    #[serde(default)]
    pub occt_source_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
}

/// A C++ template instantiation declared as an opaque Rust type.
#[derive(Debug, Deserialize, Clone)]
pub struct TemplateInstantiation {
    /// The C++ header that defines the template (for wrappers.cpp includes).
    pub header: String,
    /// The OCCT module this type belongs to (for re-export file placement).
    pub module: String,
    /// Whether this instantiation inherits from Standard_Transient and needs Handle support.
    #[serde(default)]
    pub handle: bool,
    /// Friendly name to bind this instantiation under — normally the OCCT alias
    /// (e.g. `GeomLProp_SLProps` for `GeomLProp_SLPropsBase<occ::handle<Geom_Surface>>`).
    /// Defaults to the mangled `template_alias_name()` spelling when unset.
    #[serde(default)]
    pub alias: Option<String>,
    /// Opt in to full method generation for this instantiation.
    ///
    /// Off by default: most entries here exist only to register a type (and its
    /// Handle) so signatures mentioning it resolve. Turning this on makes the
    /// parser force an explicit instantiation so libclang materializes the
    /// specialization's members, which are then bound like any other class.
    #[serde(default)]
    pub generate_methods: bool,

    /// Concrete types for the template's parameters, e.g.
    /// `{ SurfaceType = "Handle(Geom_Surface)" }`.
    ///
    /// libclang exposes a specialization's members only on the *primary template*,
    /// with parameters unsubstituted, so members mentioning a parameter would
    /// otherwise be skipped as unknown types. Values are either `Handle(X)` for a
    /// handle to X, or a plain class name. Parameters not listed here are left
    /// alone, which is fine for the many members that never mention them.
    #[serde(default)]
    pub template_params: std::collections::HashMap<String, String>,
}

/// The name an instantiation is bound under: the configured `alias` when given,
/// otherwise the mangled spelling.
pub fn instantiation_binding_name(spelling: &str, inst: &TemplateInstantiation) -> String {
    inst.alias
        .clone()
        .unwrap_or_else(|| template_alias_name(spelling))
}

/// Load and parse a TOML configuration file.
pub fn load_config(path: &Path) -> Result<BindingConfig> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: BindingConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    Ok(config)
}

/// Check if a module name matches a glob pattern.
/// Supports `*` (matches any sequence of characters) and `?` (matches exactly one character).
pub fn module_matches_pattern(module: &str, pattern: &str) -> bool {
    glob_match(module, pattern)
}

/// Simple glob matching: `*` matches any sequence, `?` matches one char.
fn glob_match(text: &str, pattern: &str) -> bool {
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    let mut ti = 0;
    let mut pi = 0;
    let mut star_pi = usize::MAX;
    let mut star_ti = 0;

    while ti < text.len() {
        if pi < pattern.len() && (pattern[pi] == b'?' || pattern[pi] == text[ti]) {
            ti += 1;
            pi += 1;
        } else if pi < pattern.len() && pattern[pi] == b'*' {
            star_pi = pi;
            star_ti = ti;
            pi += 1;
        } else if star_pi != usize::MAX {
            pi = star_pi + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }
    while pi < pattern.len() && pattern[pi] == b'*' {
        pi += 1;
    }
    pi == pattern.len()
}

/// Discover all unique module names present in the OCCT include directory.
/// A module is identified by the filename prefix before the first `_` in `.hxx` files,
/// or by a bare `{Module}.hxx` file with no underscore.
///
/// Headers with non-standard names (e.g. containing dots like `step.tab.hxx`) are
/// skipped — they are parser tables or other internal files, not real OCCT modules.
fn discover_all_modules(occt_include_dir: &Path) -> Result<Vec<String>> {
    let mut modules: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let entries = std::fs::read_dir(occt_include_dir)
        .with_context(|| format!("Failed to read OCCT include directory: {}", occt_include_dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();
        if !filename.ends_with(".hxx") {
            continue;
        }
        let stem = filename.trim_end_matches(".hxx");
        // Skip headers with non-standard names (e.g. step.tab.hxx, exptocas.tab.hxx).
        // Valid OCCT header stems contain only alphanumeric chars and underscores.
        if !stem.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
            continue;
        }
        // Module is the part before the first underscore, or the whole stem if no underscore
        let module = if let Some(pos) = stem.find('_') {
            &stem[..pos]
        } else {
            stem
        };
        modules.insert(module.to_string());
    }
    Ok(modules.into_iter().collect())
}

/// Expand the configuration into a list of header file paths.
///
/// - Expands `modules` (with glob support) against discovered OCCT modules.
/// - Removes modules matching `exclude_modules` patterns.
/// - For each matched module, discovers all matching headers in `occt_include_dir`.
/// - Adds all `include_headers`.
/// - Removes any `exclude_headers`.
///
/// Returns the list of full paths to header files.
pub fn expand_headers(config: &BindingConfig, occt_include_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut headers: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Discover all modules in the OCCT include directory
    let all_modules = discover_all_modules(occt_include_dir)?;

    // 1. Expand module patterns: resolve globs against discovered modules
    let mut matched_modules: Vec<String> = Vec::new();
    let mut matched_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for pattern in &config.modules {
        let mut found_match = false;
        for module in &all_modules {
            if module_matches_pattern(module, pattern) {
                found_match = true;
                if matched_set.insert(module.clone()) {
                    matched_modules.push(module.clone());
                }
            }
        }
        if !found_match {
            eprintln!("Warning: Module pattern '{}' did not match any OCCT modules", pattern);
        }
    }

    // 2. Apply module exclusions
    if !config.exclude_modules.is_empty() {
        let before = matched_modules.len();
        matched_modules.retain(|module| {
            !config.exclude_modules.iter().any(|pattern| module_matches_pattern(module, pattern))
        });
        let excluded = before - matched_modules.len();
        if excluded > 0 {
            println!("  Excluded {} modules via exclude_modules", excluded);
        }
    }

    // 3. Collect headers for each matched module
    for module in &matched_modules {
        let mut module_headers = Vec::new();

        // Look for {Module}.hxx
        let main_header = format!("{}.hxx", module);
        let main_path = occt_include_dir.join(&main_header);
        if main_path.exists() {
            module_headers.push((main_header.clone(), main_path));
        }

        // Look for {Module}_*.hxx
        let prefix = format!("{}_", module);
        let entries = std::fs::read_dir(occt_include_dir)
            .with_context(|| format!("Failed to read OCCT include directory: {}", occt_include_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(&prefix) && filename.ends_with(".hxx") {
                module_headers.push((filename, entry.path()));
            }
        }

        module_headers.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, path) in module_headers {
            if seen.insert(name) {
                headers.push(path);
            }
        }
    }

    // 4. Add individual headers
    for header_name in &config.include_headers {
        if seen.insert(header_name.clone()) {
            let path = occt_include_dir.join(header_name);
            if path.exists() {
                headers.push(path);
            } else {
                eprintln!("Warning: Header not found: {}", path.display());
            }
        }
    }

    // 5. Remove excluded headers
    if !config.exclude_headers.is_empty() {
        let exclude_set: std::collections::HashSet<&str> =
            config.exclude_headers.iter().map(|s| s.as_str()).collect();
        headers.retain(|path| {
            let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
            !exclude_set.contains(filename)
        });
    }

    // 6. Remove OCCT 8.0 deprecated backward-compatibility alias headers by content.
    //    These carry a standard banner comment; excluding them here avoids listing
    //    each of the ~970 alias headers individually in `exclude_headers`.
    if config.exclude_deprecated_headers {
        let before = headers.len();
        headers.retain(|path| !is_deprecated_alias_header(path));
        let excluded = before - headers.len();
        if excluded > 0 {
            println!("  Excluded {} deprecated alias headers (OCCT 8.0.0)", excluded);
        }
    }

    Ok(headers)
}

/// Detect an OCCT 8.0.0 deprecated backward-compatibility alias header (e.g.
/// `TColgp_HArray1OfPnt.hxx`, `TopTools_HSequenceOfShape.hxx`). These consist solely
/// of a `Standard_DEPRECATED` typedef forwarding to the canonical NCollection type and
/// carry the banner comment below. Matching the banner (rather than the raw
/// `Standard_HEADER_DEPRECATED` macro) avoids false positives such as `Standard_Macro.hxx`,
/// which merely defines that macro.
fn is_deprecated_alias_header(path: &Path) -> bool {
    match std::fs::read_to_string(path) {
        Ok(content) => content.contains("Deprecated typedef for backward compatibility"),
        Err(_) => false,
    }
}

/// Discover OCCT package→toolkit mappings.
///
/// Returns a map from C++ package name (e.g., "gp", "TopoDS") to toolkit name
/// (e.g., "TKMath", "TKBRep"). The binding generator uses this to split
/// generated wrappers and extern blocks by toolkit.
///
/// Supports both V8's hierarchical `src/<Module>/<Toolkit>/PACKAGES.cmake`
/// layout and V7's flat `src/<Toolkit>/PACKAGES` layout. V8 is detected by
/// the presence of `adm/MODULES`; if absent, falls back to V7 behavior.
pub fn discover_toolkit_modules(occt_source_dir: &Path) -> Result<std::collections::HashMap<String, String>> {
    use std::collections::HashMap;
    let mut module_to_toolkit: HashMap<String, String> = HashMap::new();

    // V8 layout: adm/MODULES lists module→toolkits; per-toolkit PACKAGES.cmake
    // lists packages. Located one level up from `src/`.
    let occt_root = occt_source_dir.parent().unwrap_or_else(|| Path::new("."));
    let adm_modules = occt_root.join("adm").join("MODULES");

    if adm_modules.exists() {
        let adm_content = std::fs::read_to_string(&adm_modules)
            .with_context(|| format!("Failed to read {}", adm_modules.display()))?;
        for line in adm_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.split_whitespace();
            let _module = match parts.next() {
                Some(m) => m,
                None => continue,
            };
            for toolkit in parts {
                // Look for src/<Module>/<Toolkit>/PACKAGES.cmake
                let packages_cmake = occt_source_dir
                    .join(_module)
                    .join(toolkit)
                    .join("PACKAGES.cmake");
                if packages_cmake.exists() {
                    if let Ok(content) = std::fs::read_to_string(&packages_cmake) {
                        for pkg in parse_packages_cmake(&content) {
                            module_to_toolkit.insert(pkg, toolkit.to_string());
                        }
                    }
                }
            }
        }
        if !module_to_toolkit.is_empty() {
            return Ok(module_to_toolkit);
        }
    }

    // Fallback: V7 flat layout - src/<Toolkit>/PACKAGES (plain text)
    let entries = std::fs::read_dir(occt_source_dir)
        .with_context(|| format!("Failed to read OCCT source directory: {}", occt_source_dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("TK") || !entry.path().is_dir() {
            continue;
        }
        let packages_file = entry.path().join("PACKAGES");
        if !packages_file.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&packages_file)
            .with_context(|| format!("Failed to read {}", packages_file.display()))?;
        for line in content.lines() {
            let module = line.trim();
            if !module.is_empty() {
                module_to_toolkit.insert(module.to_string(), name.clone());
            }
        }
    }

    Ok(module_to_toolkit)
}

/// Parse a PACKAGES.cmake file's contents, returning the list of package names.
///
/// The format is:
/// ```text
/// # Auto-generated list of packages for TKernel toolkit
/// set(OCCT_TKernel_LIST_OF_PACKAGES
///   FSD
///   OSD
///   ...
/// )
/// ```
/// We extract everything between `set(...)` and split on whitespace.
fn parse_packages_cmake(content: &str) -> Vec<String> {
    // Find the `set(OCCT_..._LIST_OF_PACKAGES ... )` form.
    let marker = "_LIST_OF_PACKAGES";
    let start_idx = match content.find(marker) {
        Some(i) => i + marker.len(),
        None => return Vec::new(),
    };
    // Find the opening `(` of `set(`. We need to be robust here: the variable
    // name follows immediately. After the variable name, the rest until the
    // closing `)` is the package list.
    let rest = &content[start_idx..];
    // Skip whitespace and any optional characters until we hit the package
    // list. In cmake `set(VAR a b c)`, after `VAR` there is whitespace then
    // the values. We look for the first newline (packages are listed one per
    // line) but fall back to any whitespace.
    let close_paren = match rest.rfind(')') {
        Some(i) => i,
        None => return Vec::new(),
    };
    let body = &rest[..close_paren];
    // Skip leading whitespace/newlines, then parse package names separated by
    // whitespace. Each non-empty token is a package name.
    body.split_whitespace().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packages_cmake_basic() {
        let content = "# Auto-generated list of packages for TKernel toolkit
set(OCCT_TKernel_LIST_OF_PACKAGES
  FSD
  OSD
  Plugin
  Standard
)
";
        let pkgs = parse_packages_cmake(content);
        assert_eq!(pkgs, vec!["FSD", "OSD", "Plugin", "Standard"]);
    }

    #[test]
    fn test_parse_packages_cmake_single_line() {
        let content = "set(OCCT_TKBRep_LIST_OF_PACKAGES BinTools BRep BRepAdaptor)\n";
        let pkgs = parse_packages_cmake(content);
        assert_eq!(pkgs, vec!["BinTools", "BRep", "BRepAdaptor"]);
    }

    #[test]
    fn test_parse_packages_cmake_empty() {
        let content = "set(OCCT_TKempty_LIST_OF_PACKAGES
)
";
        let pkgs = parse_packages_cmake(content);
        assert!(pkgs.is_empty());
    }

    #[test]
    fn test_parse_packages_cmake_no_marker() {
        let content = "some other cmake content";
        let pkgs = parse_packages_cmake(content);
        assert!(pkgs.is_empty());
    }

    #[test]
    fn test_discover_toolkit_modules_v8() {
        // Use the real OCCT source tree checked out as a submodule.
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()       // crates/
            .parent().unwrap();      // repo root
        let occt_src = repo_root.join("crates").join("occt-sys").join("OCCT").join("src");
        if !occt_src.exists() {
            eprintln!("Skipping: OCCT source not available at {}", occt_src.display());
            return;
        }
        let map = discover_toolkit_modules(&occt_src).expect("discovery should succeed");
        // Spot checks against the known V8 layout (these packages should exist
        // and map to the indicated toolkits, per adm/MODULES + PACKAGES.cmake).
        assert_eq!(map.get("gp"), Some(&"TKMath".to_string()), "gp should be in TKMath");
        assert_eq!(map.get("Standard"), Some(&"TKernel".to_string()), "Standard should be in TKernel");
        assert_eq!(map.get("TopoDS"), Some(&"TKBRep".to_string()), "TopoDS should be in TKBRep");
        assert_eq!(map.get("BRepBuilderAPI"), Some(&"TKTopAlgo".to_string()), "BRepBuilderAPI should be in TKTopAlgo");
        assert_eq!(map.get("BRepPrimAPI"), Some(&"TKPrim".to_string()), "BRepPrimAPI should be in TKPrim");
        // Sanity-check that the map is non-trivial.
        assert!(map.len() > 50, "expected many package entries, got {}", map.len());
    }
}