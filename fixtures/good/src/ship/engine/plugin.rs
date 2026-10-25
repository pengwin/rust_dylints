//! Documented fixture module owning one concept.

use crate::stub::{App, Plugin, Update};

fn tick() {}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}
