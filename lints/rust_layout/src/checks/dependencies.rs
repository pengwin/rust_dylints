//! Crate dependency direction: only contract crates may be depended upon inside the workspace.

use std::path::{Path, PathBuf};

use lint_support::Source;
use rustc_ast::Crate;
use toml::Value;

use crate::driver::config::Config;
use crate::driver::lints::LAYOUT_CRATE_DEPENDENCY;

const DEPENDENCY_TABLES: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];

/// Reads the manifest of the package being compiled.
pub struct Dependencies<'a, 'cx> {
    source: &'a Source<'a, 'cx>,
    config: &'a Config,
}

impl<'a, 'cx> Dependencies<'a, 'cx> {
    pub const fn new(source: &'a Source<'a, 'cx>, config: &'a Config) -> Self {
        Self { source, config }
    }

    pub fn check_crate(&self, krate: &Crate) {
        if self.config.contract_crates.is_empty() {
            return;
        }
        let package = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
        if self.config.composition_crates.contains(&package) {
            return;
        }
        let Some(manifest_dir) = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from) else {
            return;
        };
        let Some(manifest) = read_toml(&manifest_dir.join("Cargo.toml")) else {
            return;
        };
        let workspace_dependencies = workspace_dependencies(&manifest_dir);
        for table in DEPENDENCY_TABLES {
            let Some(dependencies) = manifest.get(table).and_then(Value::as_table) else {
                continue;
            };
            for (key, value) in dependencies {
                let (name, internal) = resolve(key, value, workspace_dependencies.as_ref());
                if internal && !self.config.contract_crates.contains(&name) {
                    self.source.lint(
                        LAYOUT_CRATE_DEPENDENCY,
                        krate.spans.inner_span.shrink_to_lo(),
                        format!(
                            "`{package}` depends on workspace crate `{name}`; share types through a contract crate ({})",
                            self.config.contract_crates.join(", ")
                        ),
                    );
                }
            }
        }
    }
}

/// The dependency's package name and whether it is a workspace (path) crate.
fn resolve(key: &str, value: &Value, workspace: Option<&toml::Table>) -> (String, bool) {
    let Some(table) = value.as_table() else {
        return (key.to_owned(), false);
    };
    let inherited = table.get("workspace").and_then(Value::as_bool) == Some(true);
    let effective = if inherited {
        workspace
            .and_then(|dependencies| dependencies.get(key))
            .and_then(Value::as_table)
    } else {
        Some(table)
    };
    let name = effective
        .and_then(|entry| entry.get("package"))
        .and_then(Value::as_str)
        .or_else(|| table.get("package").and_then(Value::as_str))
        .unwrap_or(key);
    let internal = effective.is_some_and(|entry| entry.contains_key("path"));
    (name.to_owned(), internal)
}

/// `[workspace.dependencies]` of the nearest enclosing workspace manifest.
fn workspace_dependencies(manifest_dir: &Path) -> Option<toml::Table> {
    manifest_dir.ancestors().find_map(|directory| {
        let manifest = read_toml(&directory.join("Cargo.toml"))?;
        let workspace = manifest.get("workspace")?.as_table()?;
        Some(
            workspace
                .get("dependencies")
                .and_then(Value::as_table)
                .cloned()
                .unwrap_or_default(),
        )
    })
}

fn read_toml(path: &Path) -> Option<toml::Table> {
    std::fs::read_to_string(path).ok()?.parse().ok()
}
