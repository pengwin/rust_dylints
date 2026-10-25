//! Lint registration entry point called by the Dylint driver.

use crate::driver::lints;
use crate::driver::pass::RustLayout;

/// Registers the lints and the early pass with the Dylint driver.
#[expect(
    unsafe_code,
    reason = "Dylint loads the library through an unmangled entry point"
)]
#[expect(
    clippy::no_mangle_with_rust_abi,
    reason = "Dylint calls this function with the Rust ABI of the pinned toolchain"
)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    dylint_linting::init_config(sess);
    lint_store.register_lints(&[
        lints::LAYOUT_RESTRICTED_VISIBILITY,
        lints::LAYOUT_ROOT_FILE_LOGIC,
        lints::LAYOUT_FORBIDDEN_FILE,
        lints::LAYOUT_OVERSIZED_FILE,
        lints::LAYOUT_CRATE_DEPENDENCY,
        lints::LAYOUT_MISSING_MODULE_DOC,
        lints::LAYOUT_FLAT_CRATE_ROOT,
    ]);
    lint_store.register_early_lint_pass(Box::new(|| Box::new(RustLayout::new())));
}
