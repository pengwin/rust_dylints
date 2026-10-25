use std::path::Path;

use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};

/// What is skipped when `ignored_paths` is not configured: build output and generated code.
pub const DEFAULT_PATTERNS: [&str; 3] = ["**/target/**", "**/generated/**", "**/*generated.rs"];

/// The default patterns, as owned strings for a configuration struct.
#[must_use]
pub fn default_patterns() -> Vec<String> {
    DEFAULT_PATTERNS
        .iter()
        .map(|&pattern| (*pattern).to_owned())
        .collect()
}

/// A set of glob patterns matched against paths relative to the package.
#[derive(Debug, Clone)]
pub struct IgnoredPaths(GlobSet);

impl IgnoredPaths {
    /// Compile the patterns.
    ///
    /// # Errors
    ///
    /// The first pattern that is not a valid glob.
    pub fn new(patterns: &[String]) -> Result<Self, globset::Error> {
        let mut set = GlobSetBuilder::new();
        for pattern in patterns {
            let glob: Glob = GlobBuilder::new(pattern).literal_separator(true).build()?;
            set.add(glob);
        }
        Ok(Self(set.build()?))
    }

    /// Whether the path is skipped.
    #[must_use]
    pub fn matches(&self, path: &Path) -> bool {
        self.0.is_match(path)
    }
}
