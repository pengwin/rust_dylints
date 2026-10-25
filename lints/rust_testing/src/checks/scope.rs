//! How the test harness and rstest look after macro expansion.
//!
//! Every test, plain or rstest-generated, leaves a `const` marked `rustc_test_marker` and an
//! entry function marked `rustc_test_entrypoint_marker`. Only a hand-written `#[test]` keeps a
//! real (non-expansion) span on that entry function; rstest and `#[fixture]` re-emit their
//! functions from a macro, so their spans come from an expansion.

use rustc_ast::{Attribute, Item, ItemKind};

pub fn has_attr(attrs: &[Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| {
        attr.name()
            .is_some_and(|attr_name| attr_name.as_str() == name)
    })
}

/// A test entry function emitted by the test harness.
pub fn is_test_entry(item: &Item) -> bool {
    matches!(item.kind, ItemKind::Fn(_)) && has_attr(&item.attrs, "rustc_test_entrypoint_marker")
}

/// The marker constant the harness emits for each test.
pub fn is_test_marker(item: &Item) -> bool {
    matches!(item.kind, ItemKind::Const(_)) && has_attr(&item.attrs, "rustc_test_marker")
}

/// Whether a module (at any depth) contains test markers: rstest puts the cases of one
/// test into a module named after it.
pub fn contains_test(item: &Item) -> bool {
    match &item.kind {
        ItemKind::Mod(_, _, rustc_ast::ModKind::Loaded(items, ..)) => items
            .iter()
            .any(|inner| is_test_marker(inner) || contains_test(inner)),
        _ => false,
    }
}
