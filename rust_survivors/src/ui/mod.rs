use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app ;
    }
}