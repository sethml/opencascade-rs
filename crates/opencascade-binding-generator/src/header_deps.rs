//! Header dependency resolution
//!
//! This module provides functionality to automatically discover and include
//! OCCT header dependencies. When a header includes another OCCT header,
//! we want to include that dependency as well so that all required types
//! are available for binding generation.

use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolves header dependencies by scanning `#include` directives.
/// 
/// Given a set of explicit headers and the OCCT include directory,
/// this function returns the expanded set including all transitive
/// OCCT header dependencies.
pub fn resolve_header_dependencies(
    explicit_headers: &[PathBuf],
    occt_include_dir: &Path,
    verbose: bool,
) -> Result<Vec<PathBuf>> {
    // Set of all header paths we've seen (canonical paths)
    let mut seen: HashSet<PathBuf> = HashSet::new();
    
    // Queue of headers to process
    let mut queue: Vec<PathBuf> = Vec::new();
    
    // Result list preserving order (explicit headers first)
    let mut result: Vec<PathBuf> = Vec::new();
    
    // Regex to match #include directives
    // Matches: #include <Header.hxx> or #include "Header.hxx"
    let include_regex = Regex::new(r#"^\s*#\s*include\s*[<"]([^>"]+)[>"]"#)?;
    
    // Add explicit headers to queue
    for header in explicit_headers {
        let canonical = header.canonicalize().unwrap_or_else(|_| header.clone());
        if seen.insert(canonical.clone()) {
            queue.push(canonical.clone());
            result.push(canonical);
        }
    }
    
    // Process queue, discovering dependencies
    let mut processed = 0;
    while let Some(header_path) = queue.pop() {
        processed += 1;
        
        // Read the header file
        let content = match std::fs::read_to_string(&header_path) {
            Ok(c) => c,
            Err(e) => {
                if verbose {
                    eprintln!("  Warning: Could not read {}: {}", header_path.display(), e);
                }
                continue;
            }
        };
        
        // Find all #include directives
        for line in content.lines() {
            if let Some(caps) = include_regex.captures(line) {
                let included_header = &caps[1];
                
                // Skip non-OCCT headers (system headers, etc.)
                // OCCT headers typically have patterns like: gp_Pnt.hxx, TopoDS_Shape.hxx, etc.
                if !is_likely_occt_header(included_header) {
                    continue;
                }
                
                // Try to find this header in the OCCT include directory
                let dep_path = occt_include_dir.join(included_header);
                if !dep_path.exists() {
                    // Header might be in a subdirectory or not an OCCT header
                    continue;
                }
                
                let canonical = dep_path.canonicalize().unwrap_or(dep_path);
                
                // Add to queue if not seen
                if seen.insert(canonical.clone()) {
                    queue.push(canonical.clone());
                    result.push(canonical);
                }
            }
        }
    }
    
    if verbose {
        eprintln!(
            "Header dependency resolution: {} explicit -> {} total ({} dependencies added)",
            explicit_headers.len(),
            result.len(),
            result.len() - explicit_headers.len()
        );
    }
    
    Ok(result)
}

/// Check if a header name looks like an OCCT header that should be parsed
fn is_likely_occt_header(name: &str) -> bool {
    // OCCT headers typically:
    // 1. End with .hxx (main headers)
    // 2. Don't start with lowercase (system headers like <string>)
    // 3. Often have underscore separating module and class name
    //
    // Note: We exclude .lxx and .gxx files as they are inline implementation
    // files that are #include'd by .hxx files. Parsing them separately causes
    // redefinition errors.
    
    // Only process .hxx files
    if !name.ends_with(".hxx") {
        return false;
    }
    
    // OCCT headers usually start with uppercase
    let first_char = name.chars().next().unwrap_or('a');
    if !first_char.is_ascii_uppercase() {
        return false;
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_likely_occt_header() {
        assert!(is_likely_occt_header("gp_Pnt.hxx"));
        assert!(is_likely_occt_header("TopoDS_Shape.hxx"));
        assert!(is_likely_occt_header("BRepPrimAPI_MakeBox.hxx"));
        assert!(is_likely_occt_header("Standard_Handle.hxx"));
        assert!(is_likely_occt_header("NCollection_Array1.hxx"));
        
        // Should NOT include .lxx or .gxx files (inline implementations)
        assert!(!is_likely_occt_header("gp_Pnt.lxx"));
        assert!(!is_likely_occt_header("Standard_HashUtils.lxx"));
        assert!(!is_likely_occt_header("TCollection_AsciiString.lxx"));
        assert!(!is_likely_occt_header("SomeFile.gxx"));
        
        // Non-OCCT headers
        assert!(!is_likely_occt_header("string"));
        assert!(!is_likely_occt_header("vector"));
        assert!(!is_likely_occt_header("stdio.h"));
        assert!(!is_likely_occt_header("memory"));
    }
}
