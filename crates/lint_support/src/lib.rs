//! Helpers shared by the Dylint libraries of this repository. Contains no lints.
#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_lint;
extern crate rustc_span;

mod ast;
mod context;

pub use ast::attributes::is_cfg_test;
pub use ast::module::Module;
pub use context::names::{file_name, is_support_name};
pub use context::package::{compile_ignored_paths, is_exempt_package};
pub use context::source::Source;
pub use ignored_paths::{IgnoredPaths, default_patterns as default_ignored_paths};
