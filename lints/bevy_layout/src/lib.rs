//! Bevy feature, plugin and layer layout lints.
#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

dylint_linting::dylint_library!();

mod checks;
mod driver;
mod model;

pub use driver::registry::register_lints;
