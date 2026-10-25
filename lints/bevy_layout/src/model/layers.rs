//! The layers of a leaf feature and the dependencies between them.
//!
//! `data`, `messaging` and `infrastructure` are the foundation every other layer may use;
//! `input`, `simulation` and `presentation` hold the systems.

use std::path::Path;

/// A layer directory of a leaf feature.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Layer {
    Data,
    Messaging,
    Infrastructure,
    Input,
    Simulation,
    Presentation,
}

/// Directory names of the layers, in dependency order.
pub const LAYER_NAMES: [&str; 6] = [
    "data",
    "messaging",
    "infrastructure",
    "input",
    "simulation",
    "presentation",
];

impl Layer {
    /// The layer which owns a module or directory name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "data" => Some(Self::Data),
            "messaging" => Some(Self::Messaging),
            "infrastructure" => Some(Self::Infrastructure),
            "input" => Some(Self::Input),
            "simulation" => Some(Self::Simulation),
            "presentation" => Some(Self::Presentation),
            _ => None,
        }
    }

    /// The deepest layer directory containing a source file.
    pub fn of_file(relative: &Path) -> Option<Self> {
        let stem = relative.file_stem().and_then(|stem| stem.to_str());
        let directories = relative
            .parent()
            .into_iter()
            .flat_map(Path::components)
            .filter_map(|component| component.as_os_str().to_str());
        directories
            .chain(stem)
            .filter_map(Self::from_name)
            .next_back()
    }

    /// Layers this layer must not import from.
    pub const fn forbidden_imports(self) -> &'static [&'static str] {
        match self {
            Self::Data => &[
                "messaging",
                "infrastructure",
                "input",
                "simulation",
                "presentation",
            ],
            Self::Messaging => &["infrastructure", "input", "simulation", "presentation"],
            Self::Infrastructure => &["input", "simulation", "presentation"],
            Self::Input => &["simulation", "presentation"],
            Self::Simulation => &["input", "presentation"],
            Self::Presentation => &["input"],
        }
    }

    /// Whether the layer must stay free of rendering imports. Infrastructure may wire
    /// rendering (material plugins, asset loaders); the rest of the game logic may not.
    pub const fn is_render_free(self) -> bool {
        !matches!(self, Self::Presentation | Self::Infrastructure)
    }

    /// Whether the layer holds types only, with no systems.
    pub const fn is_types_only(self) -> bool {
        matches!(self, Self::Data | Self::Messaging)
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::Messaging => "messaging",
            Self::Infrastructure => "infrastructure",
            Self::Input => "input",
            Self::Simulation => "simulation",
            Self::Presentation => "presentation",
        }
    }
}

/// Whether a source file is inside `presentation/scenes/` (relative to the package).
#[must_use]
pub fn is_scene_file(relative: &Path) -> bool {
    let mut components = relative
        .parent()
        .into_iter()
        .flat_map(Path::components)
        .filter_map(|component| component.as_os_str().to_str());
    components
        .by_ref()
        .any(|component| component == "presentation")
        && components.any(|component| component == "scenes")
}
