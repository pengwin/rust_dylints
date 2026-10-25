//! Import rules: layers only import downwards, and rendering stays out of render-free layers.

use lint_support::Source;
use rustc_ast::{Item, UseTree, UseTreeKind};

use crate::driver::config::Config;
use crate::driver::lints::BEVY_LAYER_DEPENDENCY;
use crate::model::layers::Layer;

/// Check every path of a `use` item written in a file belonging to `layer`.
pub fn check_use(
    source: &Source<'_, '_>,
    config: &Config,
    item: &Item,
    tree: &UseTree,
    layer: Layer,
) {
    let mut paths = Vec::new();
    flatten(&mut Vec::new(), tree, &mut paths);
    for path in paths {
        let relative = matches!(
            path.first().map(String::as_str),
            Some("crate" | "super" | "self")
        );
        if relative
            && let Some(forbidden) = path
                .iter()
                .skip(1)
                .find(|segment| layer.forbidden_imports().contains(&segment.as_str()))
        {
            source.lint(
                BEVY_LAYER_DEPENDENCY,
                item.span,
                format!(
                    "`{}/` must not import from `{forbidden}/`; depend on `data/` types or move the code to the right layer",
                    layer.name()
                ),
            );
        }
        let joined = path.join("::");
        if layer.is_render_free()
            && config
                .render_import_prefixes
                .iter()
                .any(|prefix| joined == *prefix || joined.starts_with(&format!("{prefix}::")))
        {
            source.lint(
                BEVY_LAYER_DEPENDENCY,
                item.span,
                format!(
                    "`{}/` must not use rendering API `{joined}`; rendering belongs to `presentation/`",
                    layer.name()
                ),
            );
        }
    }
}

/// Collect every full import path of a use tree.
fn flatten(prefix: &mut Vec<String>, tree: &UseTree, out: &mut Vec<Vec<String>>) {
    let depth = prefix.len();
    prefix.extend(
        tree.prefix
            .segments
            .iter()
            .map(|segment| segment.ident.name.as_str().to_owned()),
    );
    match &tree.kind {
        UseTreeKind::Simple(_) | UseTreeKind::Glob(_) => out.push(prefix.clone()),
        UseTreeKind::Nested { items, .. } => {
            for (nested, _) in items {
                flatten(prefix, nested, out);
            }
        }
    }
    prefix.truncate(depth);
}
