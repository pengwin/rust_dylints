//! Scenes belong in `presentation/scenes/`: implementors and functions returning one.

use lint_support::Source;
use rustc_ast::{FnRetTy, GenericBound, Impl, Item, Ty, TyKind};
use rustc_span::Span;

use crate::driver::config::Config;
use crate::driver::lints::BEVY_MISPLACED_SCENE;
use crate::model::layers::is_scene_file;

/// The scene rules, applied to implementors and function signatures.
pub struct Scenes<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
    config: &'a Config,
}

impl<'a, 'cx> Scenes<'a, 'cx> {
    /// Create the checker for one crate.
    #[must_use]
    pub const fn new(source: &'a Source<'a, 'cx>, config: &'a Config) -> Self {
        Self { source, config }
    }

    /// `impl Scene`/`SceneList`/`SceneComponent for X`, including derives.
    pub fn check_impl(&self, implementation: &Impl) {
        let Some(header) = &implementation.of_trait else {
            return;
        };
        let Some(segment) = header.trait_ref.path.segments.last() else {
            return;
        };
        if self.is_scene_trait(segment.ident.name.as_str()) {
            // Derives on generic types give `Self` an expansion span; the call site is the derive.
            self.report(
                implementation.self_ty.span.source_callsite(),
                segment.ident.name.as_str(),
            );
        }
    }

    /// Functions returning `impl Scene`/`impl SceneList` or `Box<dyn Scene>`.
    pub fn check_fn(&self, item: &Item, function: &rustc_ast::Fn) {
        let FnRetTy::Ty(output) = &function.sig.decl.output else {
            return;
        };
        if let Some(name) = self.scene_trait_in(output) {
            self.report(item.span, &name);
        }
    }

    fn scene_trait_in(&self, ty: &Ty) -> Option<String> {
        let bounds = match &ty.kind {
            TyKind::ImplTrait(_, bounds) | TyKind::TraitObject(bounds, _) => bounds,
            TyKind::Path(_, path) => {
                // `Box<dyn Scene>`: look through the generic arguments.
                let args = path.segments.last()?.args.as_deref()?;
                let rustc_ast::GenericArgs::AngleBracketed(angle) = args else {
                    return None;
                };
                return angle.args.iter().find_map(|arg| match arg {
                    rustc_ast::AngleBracketedArg::Arg(rustc_ast::GenericArg::Type(inner)) => {
                        self.scene_trait_in(inner)
                    }
                    _ => None,
                });
            }
            _ => return None,
        };
        bounds.iter().find_map(|bound| match bound {
            GenericBound::Trait(poly) => poly
                .trait_ref
                .path
                .segments
                .last()
                .map(|segment| segment.ident.name.as_str())
                .filter(|name| self.is_scene_trait(name))
                .map(str::to_owned),
            _ => None,
        })
    }

    fn is_scene_trait(&self, name: &str) -> bool {
        self.config.scene_traits.iter().any(|known| known == name)
    }

    fn report(&self, span: Span, name: &str) {
        if span.from_expansion() {
            return;
        }
        let in_scenes = self
            .source
            .file_of(span)
            .is_some_and(|file| is_scene_file(&self.source.relative(&file)));
        if !in_scenes {
            self.source.lint(
                BEVY_MISPLACED_SCENE,
                span,
                format!("`{name}` scenes belong in `presentation/scenes/`"),
            );
        }
    }
}
