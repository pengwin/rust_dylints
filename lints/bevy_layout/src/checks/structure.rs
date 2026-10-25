//! Checks over the module tree: feature shape, layers and plugin size.

use lint_support::{Module, Source, file_name, is_support_name};

use crate::checks::syntax::Registrations;
use crate::driver::config::Config;
use crate::driver::lints::{
    BEVY_DATA_TOO_WIDE, BEVY_FLAT_LAYER, BEVY_MIXED_FEATURE, BEVY_PLUGIN_TOO_LARGE,
    BEVY_REGISTRATION_OUTSIDE_PLUGIN, BEVY_UNLAYERED_MODULE,
};
use crate::model::layers::LAYER_NAMES;

/// Walks the module tree of a crate.
pub struct Structure<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
    config: &'a Config,
    registrations: Registrations,
}

impl<'a, 'cx> Structure<'a, 'cx> {
    pub const fn new(
        source: &'a Source<'a, 'cx>,
        config: &'a Config,
        registrations: Registrations,
    ) -> Self {
        Self {
            source,
            config,
            registrations,
        }
    }

    pub fn check_module(&self, module: &Module<'_>) {
        let children = module.children(self.source);
        if children.iter().any(|child| child.name == "plugin") {
            self.check_feature(module, &children);
        }
        for child in &children {
            self.check_module(child);
        }
    }

    fn check_feature(&self, feature: &Module<'_>, children: &[Module<'_>]) {
        let is_feature = |child: &Module<'_>| {
            child
                .children(self.source)
                .iter()
                .any(|inner| inner.name == "plugin")
        };
        let layers: Vec<_> = children
            .iter()
            .filter(|child| LAYER_NAMES.contains(&child.name.as_str()))
            .collect();
        let sub_features: Vec<_> = children.iter().filter(|child| is_feature(child)).collect();
        let label = if feature.is_root {
            "crate root"
        } else {
            feature.name.as_str()
        };

        if !layers.is_empty() && !sub_features.is_empty() {
            self.source.lint(
                BEVY_MIXED_FEATURE,
                feature.decl_span,
                format!(
                    "feature `{label}` has layers ({}) and child features ({}); make it a leaf with layers or a composite of child features",
                    names(&layers),
                    names(&sub_features),
                ),
            );
        }
        for child in children {
            let allowed = child.name == "plugin"
                || LAYER_NAMES.contains(&child.name.as_str())
                || is_feature(child)
                || is_support_name(&child.name);
            if !allowed {
                self.source.lint(
                    BEVY_UNLAYERED_MODULE,
                    child.decl_span,
                    format!(
                        "module `{}` sits directly in feature `{label}`; put it in `data/`, `input/`, `simulation/` or `presentation/`, or make it a child feature with its own `plugin.rs`",
                        child.name
                    ),
                );
            }
        }
        for layer in &layers {
            if !layer.inline && layer.file.as_deref().map(file_name) != Some("mod.rs") {
                self.source.lint(
                    BEVY_FLAT_LAYER,
                    layer.decl_span,
                    format!(
                        "layer `{}` must be a directory (`{}/mod.rs`)",
                        layer.name, layer.name
                    ),
                );
            }
            if layer.name == "data" {
                self.check_data_width(layer);
            }
        }
        if let Some(plugin) = children.iter().find(|child| child.name == "plugin") {
            self.check_plugin_size(plugin, label, !sub_features.is_empty());
        }
    }

    fn check_data_width(&self, data: &Module<'_>) {
        let width = data.children(self.source).len();
        let limit = self.config.max_data_modules;
        if width > limit {
            self.source.lint(
                BEVY_DATA_TOO_WIDE,
                data.decl_span,
                format!("`data/` declares {width} modules (limit {limit}); split the feature into child plugins"),
            );
        }
    }

    fn check_plugin_size(&self, plugin: &Module<'_>, label: &str, composite: bool) {
        let Some(calls) = plugin
            .file
            .as_ref()
            .and_then(|file| self.registrations.get(file))
        else {
            return;
        };
        let total: usize = calls.iter().map(|(_, count)| count).sum();
        if composite {
            self.source.lint(
                BEVY_REGISTRATION_OUTSIDE_PLUGIN,
                calls[0].0,
                format!("composite feature `{label}` registers {total} systems; it must only add child plugins, so move them to a child feature"),
            );
            return;
        }
        let limit = self.config.max_registered_systems;
        if total > limit {
            self.source.lint(
                BEVY_PLUGIN_TOO_LARGE,
                calls[0].0,
                format!("plugin `{label}` registers {total} systems (limit {limit}); split it into child plugins"),
            );
        }
    }
}

fn names(modules: &[&Module<'_>]) -> String {
    modules
        .iter()
        .map(|module| module.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}
