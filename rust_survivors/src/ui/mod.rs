use bevy::prelude::*;

mod components;
mod systems;

use systems::*;
use crate::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app .add_systems(OnEnter(GameState::MainMenu), spawn_ui);
    }
}