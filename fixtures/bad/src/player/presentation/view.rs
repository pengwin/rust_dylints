pub struct View;

/// A scene outside presentation/scenes.
pub struct Level;

impl crate::stub::Scene for Level {}

/// Returns a scene from the wrong place.
pub fn level() -> impl crate::stub::Scene {
    Level
}
