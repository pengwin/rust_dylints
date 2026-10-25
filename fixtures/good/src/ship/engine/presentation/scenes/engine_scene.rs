use crate::stub::Scene;

/// A scene lives in presentation/scenes.
pub struct EngineScene;

impl Scene for EngineScene {}

/// Builds the engine scene.
pub fn engine_scene() -> impl Scene {
    EngineScene
}
