//! The test style checks.

use std::collections::HashSet;

use lint_support::{Source, file_name, is_cfg_test, is_support_name};
use rustc_ast::{Block, Expr, ExprKind, Item, ItemKind, LocalKind, ModKind, StmtKind};

use crate::checks::scope::{contains_test, is_test_entry, is_test_marker};
use crate::driver::config::Config;
use crate::driver::lints::{TESTING_HELPER_FN, TESTING_INLINE_ARRANGE, TESTING_PLAIN_TEST};

/// Walks module items, tracking whether they are test code.
pub struct Tests<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
    config: &'a Config,
}

impl<'a, 'cx> Tests<'a, 'cx> {
    pub const fn new(source: &'a Source<'a, 'cx>, config: &'a Config) -> Self {
        Self { source, config }
    }

    pub fn check_items(&self, items: &[Box<Item>], in_test: bool) {
        let test_names: HashSet<_> = items
            .iter()
            .filter(|item| is_test_marker(item) || contains_test(item))
            .filter_map(|item| item.kind.ident().map(|ident| ident.name))
            .collect();
        for item in items {
            match &item.kind {
                ItemKind::Mod(_, ident, ModKind::Loaded(inner, ..)) => {
                    let scope =
                        in_test || is_cfg_test(&item.attrs) || is_support_name(ident.name.as_str());
                    self.check_items(inner, scope);
                }
                ItemKind::Fn(function) => {
                    let name = item.kind.ident().map(|ident| ident.name);
                    if is_test_entry(item) {
                        if !item.span.from_expansion() {
                            self.report(
                                item,
                                TESTING_PLAIN_TEST,
                                "use `#[rstest]` instead of `#[test]`",
                            );
                        }
                        // rstest nests the original function in the generated entry function.
                        if let (Some(name), Some(body)) = (name, &function.body) {
                            self.check_nested_original(name, body);
                        }
                    } else if name.is_some_and(|name| test_names.contains(&name)) {
                        if let Some(body) = &function.body {
                            self.check_arrange(body);
                        }
                    } else if !item.span.from_expansion() && (in_test || self.in_test_dir(item)) {
                        self.report(
                            item,
                            TESTING_HELPER_FN,
                            "test code has no helper functions: make this a `#[fixture]` (or an rstest case) and take it as an argument",
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn check_nested_original(&self, name: rustc_span::Symbol, entry_body: &Block) {
        for statement in &entry_body.stmts {
            let StmtKind::Item(inner) = &statement.kind else {
                continue;
            };
            if let ItemKind::Fn(original) = &inner.kind
                && inner.kind.ident().is_some_and(|ident| ident.name == name)
                && let Some(body) = &original.body
            {
                self.check_arrange(body);
            }
        }
    }

    fn report(&self, item: &Item, lint: &'static rustc_lint::Lint, message: &str) {
        self.source.lint(lint, item.span, message.to_owned());
    }

    fn in_test_dir(&self, item: &Item) -> bool {
        let Some(file) = self.source.file_of(item.span) else {
            return false;
        };
        if self.source.is_ignored(&file) || file_name(&file).is_empty() {
            return false;
        }
        let relative = self.source.relative(&file);
        relative
            .components()
            .next()
            .and_then(|component| component.as_os_str().to_str())
            .is_some_and(|first| self.config.test_dirs.iter().any(|dir| dir == first))
    }

    fn check_arrange(&self, body: &Block) {
        for statement in &body.stmts {
            // rstest wraps the original body in one extra block.
            if let StmtKind::Expr(expr) = &statement.kind
                && let ExprKind::Block(inner, _) = &expr.kind
            {
                self.check_arrange(inner);
                continue;
            }
            let StmtKind::Let(local) = &statement.kind else {
                continue;
            };
            let init = match &local.kind {
                LocalKind::Init(expr) | LocalKind::InitElse(expr, _) => expr,
                LocalKind::Decl => continue,
            };
            if !statement.span.from_expansion() && self.constructs(init) {
                self.source.lint(
                    TESTING_INLINE_ARRANGE,
                    statement.span,
                    "build test data in a `#[fixture]` and take it as an argument".to_owned(),
                );
            }
        }
    }

    /// Struct literals and calls of configured constructor names, through `?`/`.unwrap()` chains.
    fn constructs(&self, expr: &Expr) -> bool {
        match &expr.kind {
            ExprKind::Struct(_) => true,
            ExprKind::Call(callee, _) => matches!(&callee.kind, ExprKind::Path(_, path)
            if path.segments.last().is_some_and(|segment| {
                self.config
                    .constructor_names
                    .iter()
                    .any(|name| name == segment.ident.name.as_str())
            })),
            ExprKind::MethodCall(call) => self.constructs(&call.receiver),
            ExprKind::Try(inner) | ExprKind::Paren(inner) => self.constructs(inner),
            _ => false,
        }
    }
}
