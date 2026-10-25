//! How much a system writes: `ResMut` resources and mutable component access.
//!
//! Wrapping parameters in a tuple, a `ParamSet` or a crate-local `#[derive(SystemParam)]`
//! struct does not hide them: all of them are counted.

use std::collections::{HashMap, HashSet};

use rustc_ast::visit::{Visitor, walk_item};
use rustc_ast::{
    AngleBracketedArg, Crate, FnDecl, GenericArgs, Item, ItemKind, Mutability, Ty, TyKind,
};
use rustc_span::Symbol;

use crate::driver::config::Config;

/// Write access of one parameter, system or bundle.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Access {
    /// `ResMut`/`NonSendMut` parameters.
    pub resource_writes: usize,
    /// `&mut T` and `Mut<T>` occurrences in query data.
    pub mutable_refs: usize,
}

impl std::ops::AddAssign for Access {
    fn add_assign(&mut self, other: Self) {
        self.resource_writes += other.resource_writes;
        self.mutable_refs += other.mutable_refs;
    }
}

/// Crate-local `SystemParam` structs with the write access of their fields.
pub struct Bundles {
    access: HashMap<Symbol, Access>,
}

impl Bundles {
    /// Collect the `SystemParam` structs of a crate.
    pub fn collect(krate: &Crate, config: &Config) -> Self {
        let mut collector = Collector::default();
        for item in &krate.items {
            collector.visit_item(item);
        }
        let mut bundles = Self {
            access: HashMap::new(),
        };
        // Bundles may nest each other; a few rounds resolve any realistic depth.
        for _ in 0..4 {
            for name in &collector.params {
                let mut total = Access::default();
                for field in collector.fields.get(name).into_iter().flatten() {
                    total += measure(field, &bundles, config);
                }
                bundles.access.insert(*name, total);
            }
        }
        bundles
    }

    fn get(&self, name: Symbol) -> Option<Access> {
        self.access.get(&name).copied()
    }

    fn contains(&self, name: Symbol) -> bool {
        self.access.contains_key(&name)
    }
}

#[derive(Default)]
struct Collector<'ast> {
    params: HashSet<Symbol>,
    fields: HashMap<Symbol, Vec<&'ast Ty>>,
}

impl<'ast> Visitor<'ast> for Collector<'ast> {
    fn visit_item(&mut self, item: &'ast Item) {
        match &item.kind {
            ItemKind::Struct(ident, _, data) => {
                self.fields.insert(
                    ident.name,
                    data.fields().iter().map(|field| &*field.ty).collect(),
                );
            }
            ItemKind::Impl(implementation) => {
                if let Some(header) = &implementation.of_trait
                    && header
                        .trait_ref
                        .path
                        .segments
                        .last()
                        .is_some_and(|segment| segment.ident.name.as_str() == "SystemParam")
                    && let Some(name) = last_segment(&implementation.self_ty)
                {
                    self.params.insert(name);
                }
            }
            _ => {}
        }
        walk_item(self, item);
    }
}

/// The write access of a parameter type, descending into tuples, `ParamSet`s and bundles.
pub fn measure(ty: &Ty, bundles: &Bundles, config: &Config) -> Access {
    let mut access = Access::default();
    match &ty.kind {
        TyKind::Ref(_, inner) => {
            if inner.mutbl == Mutability::Mut {
                access.mutable_refs += 1;
            }
            access += measure(&inner.ty, bundles, config);
        }
        TyKind::Tup(types) => {
            for inner in types {
                access += measure(inner, bundles, config);
            }
        }
        TyKind::Paren(inner) => access += measure(inner, bundles, config),
        TyKind::Path(_, path) => {
            let Some(segment) = path.segments.last() else {
                return access;
            };
            let name = segment.ident.name;
            if let Some(bundle) = bundles.get(name) {
                access += bundle;
            } else if config
                .resource_write_types
                .iter()
                .any(|write| write == name.as_str())
            {
                access.resource_writes += 1;
            } else {
                if name.as_str() == "Mut" {
                    access.mutable_refs += 1;
                }
                if let Some(args) = &segment.args
                    && let GenericArgs::AngleBracketed(angle) = &**args
                {
                    for arg in &angle.args {
                        if let AngleBracketedArg::Arg(rustc_ast::GenericArg::Type(inner)) = arg {
                            access += measure(inner, bundles, config);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    access
}

/// Whether the parameter list belongs to a Bevy system.
pub fn is_system(decl: &FnDecl, bundles: &Bundles, config: &Config) -> bool {
    decl.inputs
        .iter()
        .any(|param| mentions_system_param(&param.ty, bundles, config))
}

fn mentions_system_param(ty: &Ty, bundles: &Bundles, config: &Config) -> bool {
    match &ty.kind {
        TyKind::Tup(types) => types
            .iter()
            .any(|inner| mentions_system_param(inner, bundles, config)),
        TyKind::Paren(inner) => mentions_system_param(inner, bundles, config),
        TyKind::Path(_, path) => path.segments.last().is_some_and(|segment| {
            let name = segment.ident.name;
            bundles.contains(name)
                || config
                    .system_param_types
                    .iter()
                    .any(|known| known == name.as_str())
        }),
        _ => false,
    }
}

/// Whether the function takes `&mut World` (an exclusive system).
pub fn is_exclusive(decl: &FnDecl) -> bool {
    decl.inputs.iter().any(|param| {
        matches!(&param.ty.kind, TyKind::Ref(_, inner)
            if inner.mutbl == Mutability::Mut
                && last_segment(&inner.ty).is_some_and(|name| name.as_str() == "World"))
    })
}

fn last_segment(ty: &Ty) -> Option<Symbol> {
    match &ty.kind {
        TyKind::Path(_, path) => path.segments.last().map(|segment| segment.ident.name),
        _ => None,
    }
}
