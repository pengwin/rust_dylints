//! Lint configuration read from the `[bevy_layout]` table of `dylint.toml`.

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Thresholds and name lists, all overridable from `dylint.toml`.
#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// Cargo packages that are not Bevy feature crates (tools, benches, generated code).
    pub exempt_packages: Vec<String>,
    /// Most concept modules a feature may declare in its `data/` layer.
    pub max_data_modules: usize,
    /// Most systems a leaf `plugin.rs` may register.
    pub max_registered_systems: usize,
    /// Glob patterns, relative to the package, of generated or vendored code which no lint
    /// inspects: `**/generated/**` skips a directory, `**/*generated.rs` files by name.
    pub ignored_paths: Vec<String>,
    /// File names allowed to call `add_systems` and to implement `Plugin`.
    pub registration_files: Vec<String>,
    /// App methods which, like `add_systems`, may only be called from `registration_files`.
    pub registration_methods: Vec<String>,
    /// Traits whose implementors belong in `infrastructure/` (`SystemSet`, `AssetLoader`).
    pub infrastructure_traits: Vec<String>,
    /// Traits whose implementors belong in `messaging/` (derived `Message`, `Event`).
    pub messaging_traits: Vec<String>,
    /// Traits marking a scene: implementors and functions returning `impl` of them must live in
    /// `presentation/scenes/`.
    pub scene_traits: Vec<String>,
    /// Traits whose implementors must not live in the types-only layers (`data/`, `messaging/`):
    /// derived `SystemParam` bundles borrow from the world and belong next to their systems.
    pub system_bound_traits: Vec<String>,
    /// Most `ResMut`-style parameters one system may take (counted through tuples, `ParamSet`
    /// and `SystemParam` bundles). Write less, or split the system.
    pub max_resource_writes: usize,
    /// Most mutable component accesses (`&mut T`, `Mut<T>` in query data) one system may take.
    pub max_mutable_component_access: usize,
    /// Parameter types that write a resource.
    pub resource_write_types: Vec<String>,
    /// Type names which mark a function as a Bevy system.
    pub system_param_types: Vec<String>,
    /// Import prefixes reserved for the `presentation/` layer.
    pub render_import_prefixes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exempt_packages: Vec::new(),
            max_data_modules: 8,
            max_registered_systems: 6,
            ignored_paths: lint_support::default_ignored_paths(),
            registration_files: strings(&["plugin.rs"]),
            infrastructure_traits: strings(&["SystemSet", "AssetLoader"]),
            registration_methods: strings(&[
                "init_asset",
                "init_asset_loader",
                "register_asset_loader",
                "preregister_asset_loader",
            ]),
            messaging_traits: strings(&["Message", "Event", "EntityEvent"]),
            max_resource_writes: 1,
            max_mutable_component_access: 2,
            resource_write_types: strings(&["ResMut", "NonSendMut"]),
            scene_traits: strings(&["Scene", "SceneList", "SceneComponent"]),
            system_bound_traits: strings(&["SystemParam"]),
            system_param_types: strings(&[
                "Query",
                "Single",
                "Populated",
                "Res",
                "ResMut",
                "Commands",
                "ParamSet",
                "MessageReader",
                "MessageWriter",
                "EventReader",
                "EventWriter",
                "Local",
                "NonSend",
                "NonSendMut",
            ]),
            render_import_prefixes: strings(&[
                "bevy::pbr",
                "bevy::render",
                "bevy::ui",
                "bevy::sprite",
                "bevy::text",
                "bevy::mesh",
                "bevy::gizmos",
                "bevy::scene",
                "bevy::camera",
                "bevy::light",
                "bevy_pbr",
                "bevy_render",
                "bevy_ui",
                "bevy_sprite",
                "bevy_text",
                "bevy_mesh",
                "bevy_gizmos",
                "bevy_scene",
                "bevy_hanabi",
            ]),
        }
    }
}
