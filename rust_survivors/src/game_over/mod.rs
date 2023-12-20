use bevy::prelude::*;

pub mod components;
mod systems;
pub mod events;

use events::*;
use systems::*;
use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app .add_event::<GameOver>()
            .add_systems(FixedUpdate, handle_game_over)
            .add_systems(OnEnter(GameState::GameOver), (clear_all_entities, spawn_game_over_screen))
            .add_systems(OnExit(GameState::GameOver), clear_game_over_entities)
            .add_systems(FixedUpdate, game_over_button_controls.run_if(in_state(GameState::GameOver)));
    }
}