//! Configuration read from the `[rust_layout]` table of `dylint.toml`.

use std::collections::BTreeMap;

/// Thresholds and name lists, all overridable from `dylint.toml`.
#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// Cargo packages which are not subject to the lints (tools, generated code).
    pub exempt_packages: Vec<String>,
    /// Glob patterns, relative to the package, of generated or vendored code which no lint
    /// inspects: `**/generated/**` skips a directory, `**/*generated.rs` files by name.
    pub ignored_paths: Vec<String>,
    /// Contract crates. Every other non-exempt package may depend on workspace
    /// (path) crates only if they are listed here. Empty disables the check.
    pub contract_crates: Vec<String>,
    /// Packages that compose features (an application, a transport layer): they may depend on
    /// any workspace crate, but every other rule still applies to them.
    pub composition_crates: Vec<String>,
    /// File names allowed next to the crate root file (for example `plugin.rs` in a
    /// feature crate whose root is its feature).
    pub allowed_root_files: Vec<String>,
    /// File names which need a `//!` module comment.
    pub documented_files: Vec<String>,
    /// Fewest words a module comment must contain (rejects placeholders).
    pub min_doc_words: usize,
    /// Longest allowed source file, in lines.
    pub max_file_lines: usize,
    /// Stricter limits for files below a directory, keyed by directory name (`data = 200`).
    pub max_file_lines_by_dir: BTreeMap<String, usize>,
    /// File names which are never allowed (for example `utils.rs`).
    pub forbidden_file_names: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exempt_packages: Vec::new(),
            ignored_paths: lint_support::default_ignored_paths(),
            contract_crates: Vec::new(),
            composition_crates: Vec::new(),
            allowed_root_files: Vec::new(),
            documented_files: vec![
                "mod.rs".to_owned(),
                "lib.rs".to_owned(),
                "plugin.rs".to_owned(),
            ],
            min_doc_words: 3,
            max_file_lines: 500,
            max_file_lines_by_dir: BTreeMap::new(),
            forbidden_file_names: Vec::new(),
        }
    }
}
