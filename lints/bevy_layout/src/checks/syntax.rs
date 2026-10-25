//! Checks which only need the syntax of one item: visibility, imports,
//! system placement and registration.

use std::collections::HashMap;
use std::path::PathBuf;

use rustc_ast::visit::{Visitor, walk_expr, walk_item};
use rustc_ast::{Expr, ExprKind, Item, ItemKind, TyKind};
use rustc_span::Span;

use lint_support::{Source, file_name, is_cfg_test, is_support_name};

use crate::checks::imports::check_use;
use crate::checks::scenes::Scenes;
use crate::driver::config::Config;
use crate::driver::lints::{
    BEVY_EXCLUSIVE_SYSTEM, BEVY_MISPLACED_TYPE, BEVY_REGISTRATION_OUTSIDE_PLUGIN,
    BEVY_SYSTEM_IN_DATA, BEVY_SYSTEM_WRITE_ACCESS,
};
use crate::model::access::{Access, Bundles, is_exclusive, is_system, measure};
use crate::model::layers::Layer;

/// `add_systems` calls found per file: the call span and the systems it registers.
pub type Registrations = HashMap<PathBuf, Vec<(Span, usize)>>;

/// Walks every item of the crate once.
pub struct Syntax<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
    config: &'a Config,
    bundles: &'a Bundles,
    registrations: Registrations,
}

impl<'a, 'cx> Syntax<'a, 'cx> {
    pub fn new(source: &'a Source<'a, 'cx>, config: &'a Config, bundles: &'a Bundles) -> Self {
        Self {
            source,
            config,
            bundles,
            registrations: HashMap::new(),
        }
    }

    pub fn registrations(self) -> Registrations {
        self.registrations
    }

    fn layer_of(&self, span: Span) -> Option<Layer> {
        let file = self.source.file_of(span)?;
        Layer::of_file(&self.source.relative(&file))
    }

    fn check_system_in_data(&self, item: &Item, function: &rustc_ast::Fn) {
        let Some(layer) = self
            .layer_of(item.span)
            .filter(|layer| layer.is_types_only())
        else {
            return;
        };
        let is_system = function.sig.decl.inputs.iter().any(|param| {
            matches!(&param.ty.kind, TyKind::Path(_, path)
            if path.segments.last().is_some_and(|segment| {
                self.config
                    .system_param_types
                    .iter()
                    .any(|name| name == segment.ident.name.as_str())
            }))
        });
        if is_system {
            self.source.lint(
                BEVY_SYSTEM_IN_DATA,
                item.span,
                format!(
                    "`{}/` holds types only; move this system to `infrastructure/`, `input/`, `simulation/` or `presentation/`",
                    layer.name()
                ),
            );
        }
    }

    fn check_system_access(&self, item: &Item, function: &rustc_ast::Fn) {
        let decl = &function.sig.decl;
        if is_exclusive(decl) && self.layer_of(item.span) != Some(Layer::Infrastructure) {
            self.source.lint(
                BEVY_EXCLUSIVE_SYSTEM,
                item.span,
                "exclusive systems (`&mut World`) block every other system: use queries, or move this to `infrastructure/`"
                    .to_owned(),
            );
        }
        if !is_system(decl, self.bundles, self.config) {
            return;
        }
        let mut access = Access::default();
        for param in &decl.inputs {
            access += measure(&param.ty, self.bundles, self.config);
        }
        let (resources, mutable) = (access.resource_writes, access.mutable_refs);
        let (max_resources, max_mutable) = (
            self.config.max_resource_writes,
            self.config.max_mutable_component_access,
        );
        if resources > max_resources || mutable > max_mutable {
            self.source.lint(
                BEVY_SYSTEM_WRITE_ACCESS,
                item.span,
                format!(
                    "system writes {resources} resources (max {max_resources}) and {mutable} mutable component accesses (max {max_mutable}); split the system or change the data model (tuples, `ParamSet` and `SystemParam` bundles are counted)"
                ),
            );
        }
    }

    fn check_plugin_impl(&self, item: &Item, implementation: &rustc_ast::Impl) {
        let Some(header) = &implementation.of_trait else {
            return;
        };
        let is_plugin = header
            .trait_ref
            .path
            .segments
            .last()
            .is_some_and(|segment| matches!(segment.ident.name.as_str(), "Plugin" | "PluginGroup"));
        if is_plugin {
            self.check_registration_file(item.span, "`impl Plugin`");
        }
    }

    fn check_misplaced_type(&self, implementation: &rustc_ast::Impl) {
        let Some(header) = &implementation.of_trait else {
            return;
        };
        let Some(trait_name) = header
            .trait_ref
            .path
            .segments
            .last()
            .map(|segment| segment.ident.name.as_str())
        else {
            return;
        };
        // Derives on generic types give `Self` an expansion span; the call site is the derive.
        let span = implementation.self_ty.span.source_callsite();
        if span.from_expansion() {
            return;
        }
        if self
            .config
            .system_bound_traits
            .iter()
            .any(|t| t == trait_name)
            && self.layer_of(span).is_some_and(Layer::is_types_only)
        {
            self.source.lint(
                BEVY_MISPLACED_TYPE,
                span,
                format!(
                    "`{trait_name}` types borrow from the world: keep them next to their systems in `input/`, `simulation/` or `presentation/`, or in `infrastructure/` when several layers share them"
                ),
            );
            return;
        }
        let (wanted, traits) = if self
            .config
            .infrastructure_traits
            .iter()
            .any(|t| t == trait_name)
        {
            (Layer::Infrastructure, format!("`{trait_name}`"))
        } else if self.config.messaging_traits.iter().any(|t| t == trait_name) {
            (Layer::Messaging, format!("`{trait_name}`"))
        } else {
            return;
        };
        if self.layer_of(span) != Some(wanted) {
            self.source.lint(
                BEVY_MISPLACED_TYPE,
                span,
                format!("{traits} types belong in `{}/`", wanted.name()),
            );
        }
    }

    fn check_registration_file(&self, span: Span, what: &str) {
        let Some(file) = self.source.file_of(span) else {
            return;
        };
        let name = file_name(&file);
        let stem = name.trim_end_matches(".rs");
        if self.source.is_ignored(&file)
            || is_support_name(stem)
            || self
                .config
                .registration_files
                .iter()
                .any(|allowed| allowed == name)
        {
            return;
        }
        self.source.lint(
            BEVY_REGISTRATION_OUTSIDE_PLUGIN,
            span,
            format!("{what} belongs in `plugin.rs`, not in `{name}`"),
        );
    }
}

impl<'ast> Visitor<'ast> for Syntax<'_, '_> {
    fn visit_item(&mut self, item: &'ast Item) {
        if is_cfg_test(&item.attrs) {
            return;
        }
        // Derives expand to impls with expansion spans; their `Self` type keeps the real span.
        if let ItemKind::Impl(implementation) = &item.kind {
            self.check_plugin_impl(item, implementation);
            self.check_misplaced_type(implementation);
            Scenes::new(self.source, self.config).check_impl(implementation);
        }
        if item.span.from_expansion() {
            // Derives may wrap their impls in a generated `const _: () = { .. }` block.
            walk_item(self, item);
            return;
        }
        match &item.kind {
            ItemKind::Use(tree) => {
                if let Some(layer) = self.layer_of(item.span) {
                    check_use(self.source, self.config, item, tree, layer);
                }
            }
            ItemKind::Fn(function) => {
                self.check_system_in_data(item, function);
                self.check_system_access(item, function);
                Scenes::new(self.source, self.config).check_fn(item, function);
            }
            _ => {}
        }
        walk_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let ExprKind::MethodCall(call) = &expr.kind
            && !expr.span.from_expansion()
            && call.seg.ident.name.as_str() == "add_systems"
            && call.args.len() >= 2
        {
            let count = count_systems(&call.args[1]);
            let span = call.seg.ident.span;
            if let Some(file) = self.source.file_of(span) {
                self.registrations
                    .entry(file)
                    .or_default()
                    .push((span, count));
            }
            self.check_registration_file(span, "`add_systems`");
        }
        if let ExprKind::MethodCall(call) = &expr.kind
            && !expr.span.from_expansion()
            && self
                .config
                .registration_methods
                .iter()
                .any(|name| name == call.seg.ident.name.as_str())
        {
            self.check_registration_file(
                call.seg.ident.span,
                &format!("`{}`", call.seg.ident.name.as_str()),
            );
        }
        walk_expr(self, expr);
    }
}

/// Number of systems a `add_systems` argument registers.
fn count_systems(expr: &Expr) -> usize {
    match &expr.kind {
        ExprKind::Tup(items) => items.iter().map(|item| count_systems(item)).sum(),
        ExprKind::Paren(inner) => count_systems(inner),
        ExprKind::MethodCall(call) => count_systems(&call.receiver),
        _ => 1,
    }
}
