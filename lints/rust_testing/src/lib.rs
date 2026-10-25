//! Test style lints: rstest tests and fixtures instead of plain tests and inline setup.
#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

dylint_linting::dylint_library!();

mod checks;
mod driver;

pub use driver::registry::register_lints;
