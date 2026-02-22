//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Convert a C++ template spelling to a safe Rust/C++ identifier.
/// e.g. `BVH_Builder<double, 3>` → `BVH_Builder_double_3`
/// e.g. `NCollection_Shared<NCollection_List<gp_Pnt2d>>` → `NCollection_Shared_NCollection_List_gp_Pnt2d`
pub fn template_alias_name(template_spelling: &str) -> String {
    template_spelling
        .replace("::", "_")
        .replace('<', "_")
        .replace('>', "")
        .replace(", ", "_")
        .replace(',', "_")
        .replace(' ', "")
        .trim_end_matches('_')
        .to_string()
}

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

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
}

/// A manually-defined opaque type referenced by auto-generated bindings.
#[derive(Debug, Deserialize)]
pub struct ManualType {
    /// The C++ header that defines this type (for wrappers.cpp includes).
    pub header: String,
}

/// A C++ template instantiation declared as an opaque Rust type.
#[derive(Debug, Deserialize)]
pub struct TemplateInstantiation {
    /// The C++ header that defines the template (for wrappers.cpp includes).
    pub header: String,
    /// The OCCT module this type belongs to (for re-export file placement).
    pub module: String,
    /// Whether this instantiation inherits from Standard_Transient and needs Handle support.
    #[serde(default)]
    pub handle: bool,
}

/// General configuration options.
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    /// Whether to automatically resolve header dependencies.
    #[serde(default = "default_true")]
    pub resolve_deps: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self { resolve_deps: true }
    }
}

fn default_true() -> bool {
    true
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

    Ok(headers)
}