use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(FixedUpdate, player_movement.run_if(in_state(GameState::InGame)));
    }
}