//! Minimal stand-ins for the Bevy types the lints look at.
pub struct App;
pub struct Update;
pub struct Query<T>(pub T);
pub struct Commands;
pub trait Plugin {
    fn build(&self, app: &mut App);
}
impl App {
    pub fn add_systems<S>(&mut self, _schedule: Update, _systems: S) -> &mut Self {
        self
    }
    pub fn init_asset_loader<L>(&mut self, _loader: L) -> &mut Self {
        self
    }
    pub fn add_plugins<P>(&mut self, _plugins: P) -> &mut Self {
        self
    }
}

/// Marker trait standing in for Bevy `SystemSet`.
pub trait SystemSet {}
/// Marker trait standing in for Bevy `Message`.
pub trait Message {}
/// Marker trait standing in for Bevy `SystemParam`.
pub trait SystemParam {}
/// Stand-in for Bevy `ResMut`.
pub struct ResMut<T>(pub T);
/// Stand-in for Bevy `Mut`.
pub struct Mut<T>(pub T);
/// Stand-in for Bevy `World`.
pub struct World;
/// Stand-in for Bevy `ParamSet`.
pub struct ParamSet<T>(pub T);
/// Stand-in for Bevy `AssetLoader`.
pub trait AssetLoader {}
/// Stand-in for Bevy `Scene`.
pub trait Scene {}
/// Stand-in for Bevy `SceneList`.
pub trait SceneList {}
/// Stand-in for Bevy `SceneComponent`.
pub trait SceneComponent {}
