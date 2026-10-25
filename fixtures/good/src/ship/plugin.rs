//! Documented fixture module owning one concept.

use crate::stub::{App, Plugin};

use super::engine::EnginePlugin;
use super::hull::HullPlugin;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnginePlugin, HullPlugin));
    }
}
