use bevy_pbr::Mesh;

use super::super::simulation::burn;

pub fn glow(_mesh: Mesh) {
    burn(super::super::data::Thrust(1.0));
}
