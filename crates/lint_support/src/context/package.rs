/// Whether the package being compiled is listed in a library's `exempt_packages`.
#[must_use]
pub fn is_exempt_package(exempt: &[String]) -> bool {
    std::env::var("CARGO_PKG_NAME").is_ok_and(|package| exempt.contains(&package))
}

/// Compile a library's `ignored_paths` patterns.
///
/// # Panics
///
/// With the library name and the pattern, when a pattern is not a valid glob: a lint library has
/// no better way to report a broken `dylint.toml`.
#[must_use]
pub fn compile_ignored_paths(library: &str, patterns: &[String]) -> ignored_paths::IgnoredPaths {
    ignored_paths::IgnoredPaths::new(patterns).unwrap_or_else(|failure| {
        panic!("{library}: invalid pattern in `ignored_paths`: {failure}")
    })
}
