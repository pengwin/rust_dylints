use crate::stub::{App, Plugin, Update};

fn tick() {}

pub struct ScatterPlugin;

impl Plugin for ScatterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}

#[cfg(test)]
mod tests {
    pub(crate) fn helper() {}
}

/// Loader in the wrong place.
pub struct WrongLoader;

impl crate::stub::AssetLoader for WrongLoader {}

/// Registers outside plugin.rs.
pub fn register(app: &mut App) {
    app.init_asset_loader(WrongLoader);
}
