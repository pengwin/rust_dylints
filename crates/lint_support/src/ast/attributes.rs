use rustc_ast::Attribute;

/// Whether an item is compiled only for tests.
#[must_use]
pub fn is_cfg_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.has_name(rustc_span::sym::cfg)
            && attr
                .meta_item_list()
                .is_some_and(|list| list.iter().any(|meta| meta.has_name(rustc_span::sym::test)))
    })
}
