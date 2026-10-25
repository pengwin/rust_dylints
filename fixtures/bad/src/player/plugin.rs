use crate::stub::{App, Plugin, Update};

fn a() {}
fn b() {}
fn c() {}
fn d() {}
fn e() {}
fn f() {}
fn g() {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (a, b, c, d));
        app.add_systems(Update, (e, f, g));
    }
}
