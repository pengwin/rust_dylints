//! Checks over the module tree: file names, root-file contents and file size.

use lint_support::{Module, Source, file_name, is_support_name};
use std::path::Path;

use rustc_ast::{AttrStyle, Attribute, Inline, ItemKind, ModKind};

use crate::driver::config::Config;
use crate::driver::lints::{
    LAYOUT_FLAT_CRATE_ROOT, LAYOUT_FORBIDDEN_FILE, LAYOUT_MISSING_MODULE_DOC,
    LAYOUT_OVERSIZED_FILE, LAYOUT_ROOT_FILE_LOGIC,
};

/// Walks the module tree of a crate.
pub struct Files<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
    config: &'a Config,
}

impl<'a, 'cx> Files<'a, 'cx> {
    pub const fn new(source: &'a Source<'a, 'cx>, config: &'a Config) -> Self {
        Self { source, config }
    }

    pub fn check_module(&self, module: &Module<'_>) {
        if module.is_root {
            self.check_flat_root(module);
        }
        if !module.inline {
            self.check_file(module);
        }
        for child in module.children(self.source) {
            self.check_module(&child);
        }
    }

    fn check_file(&self, module: &Module<'_>) {
        let Some(file) = &module.file else {
            return;
        };
        if self.source.is_ignored(file) {
            return;
        }
        let name = file_name(file);
        if is_support_name(name.trim_end_matches(".rs")) {
            return;
        }
        if !module.is_root
            && self
                .config
                .forbidden_file_names
                .iter()
                .any(|forbidden| forbidden == name)
        {
            self.source.lint(
                LAYOUT_FORBIDDEN_FILE,
                module.decl_span,
                format!("`{name}` is not allowed: split by concept into a directory with one file per concept"),
            );
        }
        if module.is_root || matches!(name, "mod.rs" | "lib.rs" | "main.rs") {
            self.check_root_file_logic(module, name);
        }
        self.check_module_doc(module, name);
        self.check_size(module, file, name);
    }

    /// Children of the crate root which live in a file beside the root file.
    fn check_flat_root(&self, root: &Module<'_>) {
        let Some(root_dir) = root.file.as_deref().and_then(Path::parent) else {
            return;
        };
        for item in root.items {
            let ItemKind::Mod(_, ident, ModKind::Loaded(_, Inline::No { .. }, spans)) = &item.kind
            else {
                continue;
            };
            let Some(file) = self.source.file_of(spans.inner_span) else {
                continue;
            };
            let name = file_name(&file);
            if item.span.from_expansion()
                || self.source.is_ignored(&file)
                || file.parent() != Some(root_dir)
                || self
                    .config
                    .allowed_root_files
                    .iter()
                    .any(|allowed| allowed == name)
            {
                continue;
            }
            self.source.lint(
                LAYOUT_FLAT_CRATE_ROOT,
                item.span,
                format!(
                    "`{name}` sits next to the crate root file: make it `{}/mod.rs`; the crate root directory holds only lib.rs or main.rs and module directories",
                    ident.name.as_str()
                ),
            );
        }
    }

    fn check_module_doc(&self, module: &Module<'_>, name: &str) {
        if !self
            .config
            .documented_files
            .iter()
            .any(|documented| documented == name)
        {
            return;
        }
        let words: usize = module
            .attrs
            .iter()
            .filter(|attr| attr.style == AttrStyle::Inner)
            .filter_map(Attribute::doc_str)
            .map(|doc| doc.as_str().split_whitespace().count())
            .sum();
        if words < self.config.min_doc_words {
            self.source.lint(
                LAYOUT_MISSING_MODULE_DOC,
                module.inner_span.shrink_to_lo(),
                format!(
                    "`{name}` needs a `//!` comment of at least {} words saying what the module owns",
                    self.config.min_doc_words
                ),
            );
        }
    }

    fn check_size(&self, module: &Module<'_>, file: &std::path::Path, name: &str) {
        let relative = self.source.relative(file);
        let stem = relative.file_stem().and_then(|stem| stem.to_str());
        let components = relative
            .parent()
            .into_iter()
            .flat_map(std::path::Path::components)
            .filter_map(|component| component.as_os_str().to_str())
            .chain(stem);
        let limit = components
            .filter_map(|component| self.config.max_file_lines_by_dir.get(component))
            .next_back()
            .copied()
            .unwrap_or(self.config.max_file_lines);
        let lines = self.source.line_count(module.inner_span);
        if lines > limit {
            self.source.lint(
                LAYOUT_OVERSIZED_FILE,
                module.inner_span.shrink_to_lo(),
                format!("`{name}` has {lines} lines (limit {limit}); split it by concept"),
            );
        }
    }

    fn check_root_file_logic(&self, module: &Module<'_>, name: &str) {
        for item in module.items {
            if item.span.from_expansion() {
                continue;
            }
            let allowed = match &item.kind {
                ItemKind::Mod(..) | ItemKind::Use(_) | ItemKind::ExternCrate(..) => true,
                ItemKind::Fn(function) => {
                    item.kind
                        .ident()
                        .is_some_and(|ident| ident.name.as_str() == "main")
                        && (name == "main.rs" || module.is_root)
                        && function
                            .body
                            .as_ref()
                            .is_none_or(|body| body.stmts.len() <= 1)
                }
                _ => false,
            };
            if !allowed {
                self.source.lint(
                    LAYOUT_ROOT_FILE_LOGIC,
                    item.span,
                    format!("`{name}` may only declare modules and re-export; move this item to its own file"),
                );
            }
        }
    }
}
