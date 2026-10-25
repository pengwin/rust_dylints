//! Documented fixture module owning one concept.

use crate::stub::{App, Plugin, Update};

fn tick() {}

pub struct HullPlugin;

impl Plugin for HullPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}
