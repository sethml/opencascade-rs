//! Configuration file parsing for the binding generator.
//!
//! Reads a TOML configuration file that specifies which OCCT headers to process.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Top-level configuration structure.
#[derive(Debug, Deserialize)]
pub struct BindingConfig {
    /// General settings.
    #[serde(default)]
    pub general: GeneralConfig,

    /// Include ALL headers from these OCCT modules.
    /// Every header matching `{Module}.hxx` and `{Module}_*.hxx` in the
    /// OCCT include directory will be processed.
    #[serde(default)]
    pub modules: Vec<String>,

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

/// Expand the configuration into a list of header file paths.
///
/// - For each module in `modules`, discovers all matching headers in `occt_include_dir`.
/// - Adds all `include_headers`.
/// - Removes any `exclude_headers`.
///
/// Returns the list of full paths to header files.
pub fn expand_headers(config: &BindingConfig, occt_include_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut headers: Vec<PathBuf> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    // 1. Expand modules: find all headers matching {Module}.hxx and {Module}_*.hxx
    for module in &config.modules {
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

    // 2. Add individual headers
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

    // 3. Remove excluded headers
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
