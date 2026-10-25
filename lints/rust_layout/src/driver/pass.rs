//! The early lint pass which runs every check once per crate.

use lint_support::{Module, Source, is_exempt_package};
use rustc_ast::Crate;
use rustc_ast::visit::Visitor;
use rustc_lint::{EarlyContext, EarlyLintPass};

use crate::checks::dependencies::Dependencies;
use crate::checks::files::Files;
use crate::checks::visibility::Visibility;
use crate::driver::config::Config;
use crate::driver::lints::{
    LAYOUT_CRATE_DEPENDENCY, LAYOUT_FLAT_CRATE_ROOT, LAYOUT_FORBIDDEN_FILE,
    LAYOUT_MISSING_MODULE_DOC, LAYOUT_OVERSIZED_FILE, LAYOUT_RESTRICTED_VISIBILITY,
    LAYOUT_ROOT_FILE_LOGIC,
};

/// Holds the configuration for the whole run.
pub struct RustLayout {
    config: Config,
    ignored: lint_support::IgnoredPaths,
}

impl RustLayout {
    pub fn new() -> Self {
        let config: Config = dylint_linting::config_or_default(env!("CARGO_PKG_NAME"));
        let ignored =
            lint_support::compile_ignored_paths(env!("CARGO_PKG_NAME"), &config.ignored_paths);
        Self { config, ignored }
    }
}

rustc_session::impl_lint_pass!(RustLayout => [
    LAYOUT_RESTRICTED_VISIBILITY,
    LAYOUT_ROOT_FILE_LOGIC,
    LAYOUT_FORBIDDEN_FILE,
    LAYOUT_OVERSIZED_FILE,
    LAYOUT_CRATE_DEPENDENCY,
    LAYOUT_MISSING_MODULE_DOC,
    LAYOUT_FLAT_CRATE_ROOT,
]);

impl EarlyLintPass for RustLayout {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        if is_exempt_package(&self.config.exempt_packages) {
            return;
        }
        let source = Source::new(cx, &self.ignored);
        let mut visibility = Visibility::new(&source);
        for item in &krate.items {
            visibility.visit_item(item);
        }
        Dependencies::new(&source, &self.config).check_crate(krate);
        Files::new(&source, &self.config).check_module(&Module::root(&source, krate));
    }
}
