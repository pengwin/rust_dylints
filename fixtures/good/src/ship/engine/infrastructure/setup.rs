use crate::stub::World;

/// Exclusive systems are allowed in infrastructure.
pub fn exclusive_setup(_world: &mut World) {}

/// Asset loaders belong in infrastructure.
pub struct EngineLoader;

impl crate::stub::AssetLoader for EngineLoader {}
