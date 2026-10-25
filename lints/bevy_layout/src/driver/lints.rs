//! Lint declarations.

dylint_linting::declare_lint! {
    /// ### What it does
    /// Requires `data`, `input`, `simulation` and `presentation` to be directories.
    pub BEVY_FLAT_LAYER,
    Warn,
    "layer module is a single file"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Limits the number of concept modules in a feature's `data/` layer.
    pub BEVY_DATA_TOO_WIDE,
    Warn,
    "too many modules in a data layer"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Limits the systems registered by one leaf `plugin.rs`.
    pub BEVY_PLUGIN_TOO_LARGE,
    Warn,
    "leaf plugin registers too many systems"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Requires a feature to be either a leaf (layers) or a composite (child features).
    pub BEVY_MIXED_FEATURE,
    Warn,
    "feature has both layers and child features"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects modules in a feature which are neither `plugin`, a layer,
    /// a child feature, nor test support.
    pub BEVY_UNLAYERED_MODULE,
    Warn,
    "module outside the feature layout"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects `add_systems` and `impl Plugin` outside `plugin.rs`, and
    /// `add_systems` inside a composite plugin.
    pub BEVY_REGISTRATION_OUTSIDE_PLUGIN,
    Warn,
    "plugin or system registration outside its place"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects layer imports against the `data <- input/simulation <- presentation`
    /// direction and rendering imports in render-free layers.
    pub BEVY_LAYER_DEPENDENCY,
    Warn,
    "import crosses a layer boundary"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects systems (functions taking Bevy system parameters) in `data/`.
    pub BEVY_SYSTEM_IN_DATA,
    Warn,
    "system defined in a data layer"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Requires `SystemSet` types in `infrastructure/` and `Message`/`Event` types in `messaging/`.
    pub BEVY_MISPLACED_TYPE,
    Warn,
    "scheduling or messaging type outside its layer"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Rejects exclusive systems (`&mut World` parameter) outside `infrastructure/`.
    ///
    /// ### Why is this bad?
    /// An exclusive system blocks every other system while it runs.
    pub BEVY_EXCLUSIVE_SYSTEM,
    Warn,
    "exclusive system outside infrastructure"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Limits the write access of one system: `ResMut`-style parameters
    /// (`max_resource_writes`) and mutable component access (`max_mutable_component_access`),
    /// counted through tuples, `ParamSet` and crate-local `SystemParam` bundles.
    ///
    /// ### Why is this bad?
    /// Systems run in parallel only when their data access does not conflict; a system which
    /// writes a lot blocks many others. Write less, or split the system.
    pub BEVY_SYSTEM_WRITE_ACCESS,
    Warn,
    "system writes too much"
}

dylint_linting::declare_lint! {
    /// ### What it does
    /// Requires scenes to live in `presentation/scenes/`: functions returning
    /// `impl Scene`/`impl SceneList`, and `Scene`, `SceneList` and `SceneComponent` implementors
    /// (including `#[derive(SceneComponent)]`).
    ///
    /// ### Why is this bad?
    /// A scene is assembly of what the player sees. Keeping it in one place makes spawning
    /// consistent, and keeps rendering types out of `data/`.
    pub BEVY_MISPLACED_SCENE,
    Warn,
    "scene outside presentation/scenes"
}
