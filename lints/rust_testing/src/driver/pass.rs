//! The early lint pass which runs every check once per crate.

use lint_support::{Source, is_exempt_package};
use rustc_ast::Crate;
use rustc_lint::{EarlyContext, EarlyLintPass};

use crate::checks::test_style::Tests;
use crate::driver::config::Config;
use crate::driver::lints::{TESTING_HELPER_FN, TESTING_INLINE_ARRANGE, TESTING_PLAIN_TEST};

/// Holds the configuration for the whole run.
pub struct RustTesting {
    config: Config,
    ignored: lint_support::IgnoredPaths,
}

impl RustTesting {
    pub fn new() -> Self {
        let config: Config = dylint_linting::config_or_default(env!("CARGO_PKG_NAME"));
        let ignored =
            lint_support::compile_ignored_paths(env!("CARGO_PKG_NAME"), &config.ignored_paths);
        Self { config, ignored }
    }
}

rustc_session::impl_lint_pass!(RustTesting => [
    TESTING_PLAIN_TEST,
    TESTING_HELPER_FN,
    TESTING_INLINE_ARRANGE,
]);

impl EarlyLintPass for RustTesting {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        if is_exempt_package(&self.config.exempt_packages) {
            return;
        }
        let source = Source::new(cx, &self.ignored);
        Tests::new(&source, &self.config).check_items(&krate.items, false);
    }
}
