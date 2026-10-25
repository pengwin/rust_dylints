//! Lint registration entry point called by the Dylint driver.

use crate::driver::lints;
use crate::driver::pass::BevyLayout;

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
        lints::BEVY_FLAT_LAYER,
        lints::BEVY_DATA_TOO_WIDE,
        lints::BEVY_PLUGIN_TOO_LARGE,
        lints::BEVY_MIXED_FEATURE,
        lints::BEVY_UNLAYERED_MODULE,
        lints::BEVY_REGISTRATION_OUTSIDE_PLUGIN,
        lints::BEVY_LAYER_DEPENDENCY,
        lints::BEVY_SYSTEM_IN_DATA,
        lints::BEVY_MISPLACED_TYPE,
        lints::BEVY_EXCLUSIVE_SYSTEM,
        lints::BEVY_SYSTEM_WRITE_ACCESS,
        lints::BEVY_MISPLACED_SCENE,
    ]);
    lint_store.register_early_lint_pass(Box::new(|| Box::new(BevyLayout::new())));
}
