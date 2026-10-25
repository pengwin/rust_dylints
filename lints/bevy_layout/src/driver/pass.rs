//! The early lint pass which runs every check once per crate.

use lint_support::{Module, Source, is_exempt_package};
use rustc_ast::Crate;
use rustc_ast::visit::Visitor;
use rustc_lint::{EarlyContext, EarlyLintPass};

use crate::checks::structure::Structure;
use crate::checks::syntax::Syntax;
use crate::driver::config::Config;
use crate::driver::lints::{
    BEVY_DATA_TOO_WIDE, BEVY_EXCLUSIVE_SYSTEM, BEVY_FLAT_LAYER, BEVY_LAYER_DEPENDENCY,
    BEVY_MISPLACED_SCENE, BEVY_MISPLACED_TYPE, BEVY_MIXED_FEATURE, BEVY_PLUGIN_TOO_LARGE,
    BEVY_REGISTRATION_OUTSIDE_PLUGIN, BEVY_SYSTEM_IN_DATA, BEVY_SYSTEM_WRITE_ACCESS,
    BEVY_UNLAYERED_MODULE,
};
use crate::model::access::Bundles;

/// Holds the configuration for the whole run.
pub struct BevyLayout {
    config: Config,
    ignored: lint_support::IgnoredPaths,
}

impl BevyLayout {
    pub fn new() -> Self {
        let config: Config = dylint_linting::config_or_default(env!("CARGO_PKG_NAME"));
        let ignored =
            lint_support::compile_ignored_paths(env!("CARGO_PKG_NAME"), &config.ignored_paths);
        Self { config, ignored }
    }
}

rustc_session::impl_lint_pass!(BevyLayout => [
    BEVY_FLAT_LAYER,
    BEVY_DATA_TOO_WIDE,
    BEVY_PLUGIN_TOO_LARGE,
    BEVY_MIXED_FEATURE,
    BEVY_UNLAYERED_MODULE,
    BEVY_REGISTRATION_OUTSIDE_PLUGIN,
    BEVY_LAYER_DEPENDENCY,
    BEVY_SYSTEM_IN_DATA,
    BEVY_MISPLACED_TYPE,
    BEVY_EXCLUSIVE_SYSTEM,
    BEVY_SYSTEM_WRITE_ACCESS,
    BEVY_MISPLACED_SCENE,
]);

impl EarlyLintPass for BevyLayout {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        if is_exempt_package(&self.config.exempt_packages) {
            return;
        }
        let source = Source::new(cx, &self.ignored);
        let bundles = Bundles::collect(krate, &self.config);
        let mut syntax = Syntax::new(&source, &self.config, &bundles);
        for item in &krate.items {
            syntax.visit_item(item);
        }
        Structure::new(&source, &self.config, syntax.registrations())
            .check_module(&Module::root(&source, krate));
    }
}
