//! The files a lint library does not inspect, as glob patterns (`globset`, the matcher of ripgrep).
//!
//! Patterns follow gitignore-like rules: `*` stays within one path component, `**` crosses any
//! number of directories, and `**/` at the start matches in any directory, so
//! `**/generated/*.rs` skips the `.rs` files directly in any `generated` directory, `**/generated/**`
//! everything below it and `**/*generated.rs` any file whose name ends in `generated.rs`.

mod matcher;

pub use matcher::{DEFAULT_PATTERNS, IgnoredPaths, default_patterns};
