# rust_dylints

Dylint lint libraries for project structure (see README.md): `lints/rust_layout`,
`lints/rust_testing`, `lints/bevy_layout`, with shared helpers in `crates/lint_support`.
The rules for the Bevy architecture they enforce live in the `bevy-workspace` skill, not here.

## Mandatory instruction compliance

Before editing code:

1. Read AGENTS.md and every referenced instruction file completely.
2. Create an internal checklist of applicable requirements.
3. Do not replace an explicitly requested architecture or existing implementation
   with a simplified alternative without user approval.
4. Treat architecture requirements as acceptance criteria alongside build and lint checks.

## Workspace structure

- Libraries (cdylib, one `register_lints` entry point each) in lints/, shared helpers in crates/.
- Fixtures in fixtures/ are standalone crates (own `[workspace]`), never members; `bad` must
  trigger exactly the set in fixtures/expected/<library>.txt, `good` must stay silent.
- Keep shared package metadata, dependencies and lints in root Cargo.toml.
  Every member inherits workspace package metadata and explicitly enables
  `[lints] workspace = true`; inherit shared dependencies with `dependency.workspace = true`.
- Preserve configured lint levels and priorities. Do not add broad allow
  attributes to make checks pass; use a narrow `#[expect(..., reason = "...")]`.
  `unsafe_code` is `deny` (not `forbid`) only because Dylint needs an unmangled entry point.
- The nightly is pinned in rust-toolchain.toml. A toolchain bump is a release: fix
  `rustc_private` API drift, run the fixtures, update CHANGELOG.md.
- Keep Cargo.lock available for version control.

## Rust code design

- Keep lib.rs and mod.rs limited to declarations, module wiring and intended re-exports.
  Put logic in separate implementation files, one concern per file.
- Keep implementation details private; expose only what other crates need. Exposure is decided
  by lib.rs and mod.rs re-exports: pub(crate), pub(super) and pub(in ...) are not used.
- The crate root directory holds only lib.rs or main.rs and module directories (`layout_flat_crate_root`);
  a module is `foo/mod.rs` (declarations only) plus implementation files beside it.
  Group modules by meaning, not one directory per file: each library has `driver/` (lint
  declarations, config, pass, registration), `checks/` (the rules) and, where needed, `model/`.
- Every mod.rs and lib.rs starts with a `//!` comment saying what the module owns.
- Documentation comments on all public items (`missing_docs` is on).
- A lint change is complete only with: the lint, its registration, a `bad` fixture case that
  fires it, a `good` fixture case that stays silent, the expected-list entry, README and
  CHANGELOG entries, and a Cargo.toml version bump. Release by tagging `vX.Y.Z`; consumers pin the tag.

## Development commands

Use Justfile as the interface for development:

- `just fmt` / `just fmt-check` format the workspace.
- `just build [package]` builds the workspace or one package.
- `just lint [package]` runs Clippy (all targets); the whole-workspace form also runs
  `just lint-self`, where rust_layout and rust_testing lint this repository (dogfooding).
- `just test [package]` runs nextest; the whole-workspace form also runs `just test-fixtures`.

Do not invoke Cargo directly to build, test or lint. Cargo commands inside Justfile are the
intended implementation. After changes run `just fmt`, `just build`, `just lint` and `just test`.

## Documentation and completion

- Keep README.md (consume snippet, lint catalog, configuration) and CHANGELOG.md current.
- Before reporting completion verify member inheritance and that lib.rs files contain only
  the allowed wiring and exports.
- Report checks actually run and any unresolved failures or instruction deviations.
  Do not claim completion when applicable requirements remain unmet.
- Do not push or tag without the user's explicit confirmation.

@RTK.md
