use crate::stub::{App, Plugin, Update};

fn tick() {}

pub struct HybridPlugin;

impl Plugin for HybridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}
