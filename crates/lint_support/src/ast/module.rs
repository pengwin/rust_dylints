use std::path::PathBuf;

use rustc_ast::{Attribute, Crate, Inline, Item, ItemKind, ModKind};
use rustc_span::Span;

use crate::ast::attributes::is_cfg_test;
use crate::context::source::Source;

/// A module with its contents resolved.
pub struct Module<'ast> {
    /// The module name; empty for the crate root.
    pub name: String,
    /// Attributes of the module, including inner doc comments.
    pub attrs: &'ast [Attribute],
    /// Items declared in the module.
    pub items: &'ast [Box<Item>],
    /// Declared with a body (`mod x { .. }`) instead of loaded from its own file.
    pub inline: bool,
    /// The file holding the module's contents.
    pub file: Option<PathBuf>,
    /// Span of the `mod` declaration.
    pub decl_span: Span,
    /// Span of the module contents.
    pub inner_span: Span,
    /// Whether this is the crate root.
    pub is_root: bool,
}

impl<'ast> Module<'ast> {
    /// The crate root.
    #[must_use]
    pub fn root(source: &Source<'_, '_>, krate: &'ast Crate) -> Self {
        let inner_span = krate.spans.inner_span;
        Self {
            name: String::new(),
            attrs: &krate.attrs,
            items: &krate.items,
            inline: false,
            file: source.file_of(inner_span),
            decl_span: inner_span,
            inner_span,
            is_root: true,
        }
    }

    /// Child modules, excluding macro-generated and `#[cfg(test)]` ones.
    #[must_use]
    pub fn children(&self, source: &Source<'_, '_>) -> Vec<Self> {
        self.items
            .iter()
            .filter(|item| !item.span.from_expansion() && !is_cfg_test(&item.attrs))
            .filter_map(|item| {
                let ItemKind::Mod(_, ident, ModKind::Loaded(items, inline, spans)) = &item.kind
                else {
                    return None;
                };
                let inline = matches!(inline, Inline::Yes);
                let file = if inline {
                    self.file.clone()
                } else {
                    source.file_of(spans.inner_span)
                };
                Some(Self {
                    name: ident.name.as_str().to_owned(),
                    attrs: &item.attrs,
                    items,
                    inline,
                    file,
                    decl_span: item.span,
                    inner_span: spans.inner_span,
                    is_root: false,
                })
            })
            .collect()
    }
}
