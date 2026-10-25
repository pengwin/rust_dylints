//! Compiling glob patterns and matching paths against them.

mod ignored;
#[cfg(test)]
mod ignored_tests;

pub use ignored::{DEFAULT_PATTERNS, IgnoredPaths, default_patterns};
