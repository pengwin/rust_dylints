//! Lint declarations.

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects `pub(crate)`, `pub(super)` and `pub(in path)`.
    ///
    /// ### Why is this bad?
    /// Exposure is decided by what `lib.rs` and `mod.rs` re-export. Restricted
    /// visibility papers over unclear module boundaries.
    pub LAYOUT_RESTRICTED_VISIBILITY,
    Warn,
    "restricted visibility instead of a deliberate export boundary"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects items other than module declarations and re-exports in `mod.rs`,
    /// `lib.rs` and `main.rs` (a one-statement `fn main` may delegate).
    pub LAYOUT_ROOT_FILE_LOGIC,
    Warn,
    "logic in a module root file"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects file names listed in `forbidden_file_names`.
    pub LAYOUT_FORBIDDEN_FILE,
    Warn,
    "forbidden file name"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Limits source file length (`max_file_lines`, `max_file_lines_by_dir`).
    pub LAYOUT_OVERSIZED_FILE,
    Warn,
    "source file too long"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects dependencies from a package on another workspace crate which is not
    /// listed in `contract_crates`.
    ///
    /// ### Why is this bad?
    /// Feature crates communicate through narrow contract crates; sibling
    /// dependencies tangle features together.
    pub LAYOUT_CRATE_DEPENDENCY,
    Warn,
    "dependency on a workspace crate which is not a contract crate"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Requires a `//!` module comment of at least `min_doc_words` words in the files named
    /// by `documented_files` (`mod.rs`, `lib.rs`, `plugin.rs` by default).
    ///
    /// ### Why is this bad?
    /// `missing_docs` only covers public items. The module comment states what the module
    /// owns, which is the contract readers and agents rely on to place code.
    pub LAYOUT_MISSING_MODULE_DOC,
    Warn,
    "module file without a descriptive `//!` comment"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Requires the crate root directory (`src/`) to hold only `lib.rs` or `main.rs` and
    /// module directories: a module loaded from `src/foo.rs` must be `src/foo/mod.rs`.
    /// Files named in `allowed_root_files` are exempt. `mod.rs` directories may hold
    /// other files, and `build.rs` lives outside `src/`.
    ///
    /// ### Why is this bad?
    /// A flat file next to `lib.rs` hides the module's structure and invites a growing pile of
    /// root files; a directory gives every module a place for its parts.
    pub LAYOUT_FLAT_CRATE_ROOT,
    Warn,
    "module file next to the crate root file instead of a module directory"
}
