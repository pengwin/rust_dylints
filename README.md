# rust_dylints

[Dylint](https://github.com/trailofbits/dylint) lints for project structure. Three
libraries share one pinned nightly (`rust-toolchain.toml`) and the helpers in
`crates/lint_support` (source map, module tree) and `crates/ignored_paths` (glob matching of the files
the lints skip):

| Library | Applies to | Lints |
|---|---|---|
| `rust_layout` | any Rust project | `layout_restricted_visibility`, `layout_root_file_logic`, `layout_forbidden_file`, `layout_oversized_file`, `layout_crate_dependency`, `layout_missing_module_doc`, `layout_flat_crate_root` |
| `rust_testing` | any Rust project using rstest | `testing_plain_test`, `testing_helper_fn`, `testing_inline_arrange` |
| `bevy_layout` | Bevy feature crates | `bevy_flat_layer`, `bevy_data_too_wide`, `bevy_plugin_too_large`, `bevy_mixed_feature`, `bevy_unlayered_module`, `bevy_registration_outside_plugin`, `bevy_layer_dependency`, `bevy_system_in_data`, `bevy_misplaced_type`, `bevy_exclusive_system`, `bevy_system_write_access`, `bevy_misplaced_scene` |

`rust_testing` limits: it flags hand-written `#[test]`, plain helper functions in `#[cfg(test)]` modules
and `tests/` files, and `let` bindings in rstest bodies that call a constructor name or build a struct
literal. Merging near-identical tests into one `#[case]` test is a review rule (not detectable).

The rules behind `bevy_layout` are described in the `bevy-workspace` skill
(`docs/BEVY_ARCHITECTURE.md`); this repository holds only the lints.

## Use

Install the tools once (version 6 or newer):

```sh
cargo install cargo-dylint dylint-link --locked
```

In the linted workspace `Cargo.toml`, pin a tag:

```toml
[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/pengwin/rust_dylints", tag = "v0.0.1", pattern = ["lints/rust_layout", "lints/rust_testing", "lints/bevy_layout"] },
]
```

Run `cargo dylint --all --workspace -- --all-targets` (`rust_testing` inspects test code, which only
exists in test targets). The first run builds the libraries and makes rustup
install the nightly from `rust-toolchain.toml`. Dylint compiles the linted packages with
that nightly, so the workspace `rust-version` must not exceed it.

Allow a lint narrowly without an "unknown lint" warning when Dylint is absent:

```rust
#[cfg_attr(dylint_lib = "rust_layout", allow(layout_restricted_visibility))]
```

## Configuration (`dylint.toml` in the linted workspace root)

`ignored_paths` (all three libraries) lists glob patterns for files no lint inspects, matched on the path
relative to the package with the [`globset`](https://docs.rs/globset) crate of ripgrep. `*` stays within
one path component, `**` crosses directories, a leading `**/` matches at any depth:

| Pattern | Skips |
|---|---|
| `**/generated/**` | everything below any `generated` directory |
| `**/generated/*.rs` | the `.rs` files directly in any `generated` directory |
| `**/*generated.rs` | `api_generated.rs`, `api.generated.rs`, `generated.rs` at any depth |
| `src/vendor/**` | only `src/vendor` of the package |

Setting `ignored_paths` replaces the defaults, so keep the ones you still want. A pattern that is not a valid
glob stops the lint with a message naming it.

```toml
[rust_layout]
exempt_packages = ["tools"]            # packages skipped entirely
ignored_paths = ["**/target/**", "**/generated/**", "**/*generated.rs"]   # globs, see the table above (these are the defaults)
max_file_lines = 500                    # the default; a file longer than this is layout_oversized_file
max_file_lines_by_dir = { data = 200 }  # stricter limit below a directory
forbidden_file_names = ["components.rs", "systems.rs", "data.rs"]
documented_files = ["mod.rs", "lib.rs", "plugin.rs"]   # need a //! comment
allowed_root_files = ["plugin.rs"]     # exempt from layout_flat_crate_root (src/ holds lib.rs/main.rs + dirs)
min_doc_words = 3
contract_crates = ["my-game-core"]     # packages may depend on workspace crates only if listed (empty = off)
composition_crates = ["my-game"]       # these packages may depend on any workspace crate; other rules still apply

[rust_testing]
exempt_packages = []
ignored_paths = ["**/target/**", "**/generated/**", "**/*generated.rs"]
test_dirs = ["tests"]            # integration test directories (helper-fn rule applies there)
constructor_names = ["new", "default", "build", "from", "with_capacity"]

[bevy_layout]
exempt_packages = ["my-game"]
max_data_modules = 8
max_registered_systems = 6
registration_files = ["plugin.rs"]
infrastructure_traits = ["SystemSet", "AssetLoader"]   # must live in infrastructure/
registration_methods = ["init_asset", "init_asset_loader"]   # plugin.rs only, like add_systems
messaging_traits = ["Message", "Event", "EntityEvent"]   # must live in messaging/
system_bound_traits = ["SystemParam"]                    # never in data/ or messaging/
scene_traits = ["Scene", "SceneList", "SceneComponent"]    # must live in presentation/scenes/
max_resource_writes = 1                                  # ResMut per system, through ParamSet/bundles too
max_mutable_component_access = 2                         # &mut T / Mut<T> in query data per system
# system_param_types, render_import_prefixes: see lints/bevy_layout/src/driver/config.rs
```

## Develop

Rust code rules and the review checklist are in AGENTS.md. Commands (via `just`):

```sh
just build          # the libraries, pinned nightly
just lint           # Clippy (strict) + the libraries linting this repository (dogfooding)
just test           # nextest + fixtures
just test-fixtures  # lint fixtures/good (must be silent) and fixtures/bad (must fire fixtures/expected/*)
```

CI (`.github/workflows/ci.yml`) is disabled to save build minutes; start it manually from the
Actions tab or restore its triggers.

Adding a lint: declare it in the library's `lints.rs`, register it in `registry.rs` and
`pass.rs` (keep files under 500 lines, the default limit: split by concept), make `fixtures/bad` trigger it and `fixtures/good` stay silent, add its name to
`fixtures/expected/<library>.txt`, and record it in `CHANGELOG.md`.

Bumping the toolchain: edit `rust-toolchain.toml`, fix `rustc_private` API drift, run the
fixtures, and tag a new version (consumers upgrade by changing their `tag`).

Dylint caches: after changing a library, `cargo dylint` rebuilds it automatically.

## License

[MIT](LICENSE).
