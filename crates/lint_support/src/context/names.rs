use std::path::Path;

/// File name of a path as text.
#[must_use]
pub fn file_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
}

/// Whether a module name belongs to test scaffolding rather than to production code.
#[must_use]
pub fn is_support_name(name: &str) -> bool {
    name == "tests" || name == "test_support" || name.ends_with("_tests") || name.ends_with("_test")
}
