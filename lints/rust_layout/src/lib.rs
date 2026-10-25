//! Source layout hygiene lints for any Rust project.
#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_lint;
extern crate rustc_session;

dylint_linting::dylint_library!();

mod checks;
mod driver;

pub use driver::registry::register_lints;
