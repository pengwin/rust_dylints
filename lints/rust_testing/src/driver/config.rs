//! Configuration read from the `[rust_testing]` table of `dylint.toml`.

/// Name lists, overridable from `dylint.toml`.
#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// Cargo packages which are not subject to the lints.
    pub exempt_packages: Vec<String>,
    /// Glob patterns, relative to the package, of generated code which no lint inspects.
    pub ignored_paths: Vec<String>,
    /// Directories (relative to the package) holding integration tests.
    pub test_dirs: Vec<String>,
    /// Associated functions which build test data and belong in a `#[fixture]`.
    pub constructor_names: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let strings = |values: &[&str]| values.iter().map(|value| (*value).to_owned()).collect();
        Self {
            exempt_packages: Vec::new(),
            ignored_paths: lint_support::default_ignored_paths(),
            test_dirs: strings(&["tests"]),
            constructor_names: strings(&["new", "default", "build", "from", "with_capacity"]),
        }
    }
}
