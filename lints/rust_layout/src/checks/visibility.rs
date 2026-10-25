//! Rejects restricted visibility everywhere outside test code.

use lint_support::{Source, is_cfg_test};
use rustc_ast::visit::{Visitor, walk_item};
use rustc_ast::{Item, Visibility as AstVisibility, VisibilityKind};

use crate::driver::lints::LAYOUT_RESTRICTED_VISIBILITY;

/// Walks every item once.
pub struct Visibility<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
}

impl<'a, 'cx> Visibility<'a, 'cx> {
    pub const fn new(source: &'a Source<'a, 'cx>) -> Self {
        Self { source }
    }
}

impl<'ast> Visitor<'ast> for Visibility<'_, '_> {
    fn visit_item(&mut self, item: &'ast Item) {
        if item.span.from_expansion() || is_cfg_test(&item.attrs) {
            return;
        }
        walk_item(self, item);
    }

    fn visit_vis(&mut self, vis: &'ast AstVisibility) {
        if let VisibilityKind::Restricted { path, .. } = &vis.kind
            && !vis.span.from_expansion()
            && path
                .segments
                .last()
                .is_some_and(|segment| segment.ident.name.as_str() != "self")
        {
            self.source.lint(
                LAYOUT_RESTRICTED_VISIBILITY,
                vis.span,
                "use plain `pub` and control exposure through `mod.rs`/`lib.rs` exports, or keep the item private"
                    .to_owned(),
            );
        }
    }
}
