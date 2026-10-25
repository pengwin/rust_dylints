# Changelog

## 0.0.1

First public release: three Dylint libraries over one pinned nightly.

- `rust_layout` (any Rust project): `layout_restricted_visibility`, `layout_root_file_logic`,
  `layout_forbidden_file`, `layout_oversized_file`, `layout_crate_dependency`,
  `layout_missing_module_doc`, `layout_flat_crate_root`. Options include `max_file_lines` (default
  500), `max_file_lines_by_dir`, `contract_crates` and `composition_crates`.
- `rust_testing` (rstest): `testing_plain_test`, `testing_helper_fn`, `testing_inline_arrange`.
- `bevy_layout` (Bevy feature crates): `bevy_flat_layer`, `bevy_data_too_wide`,
  `bevy_plugin_too_large`, `bevy_mixed_feature`, `bevy_unlayered_module`,
  `bevy_registration_outside_plugin`, `bevy_layer_dependency`, `bevy_system_in_data`,
  `bevy_misplaced_type`, `bevy_exclusive_system`, `bevy_system_write_access`,
  `bevy_misplaced_scene`.
- `ignored_paths` (all libraries): glob patterns (`globset`) for the files no lint inspects, by default
  `**/target/**`, `**/generated/**` and `**/*generated.rs`.
- Toolchain: `nightly-2026-08-20` (see `rust-toolchain.toml`).
- License: MIT.
