use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod events;
mod systems;

use systems::*;
use crate::GameState;
use events::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app .add_event::<CharacterChoice>()
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(FixedUpdate, player_movement.run_if(in_state(GameState::InGame)))
            .add_systems(FixedUpdate, player_check_collision.run_if(in_state(GameState::InGame)));
    }
}