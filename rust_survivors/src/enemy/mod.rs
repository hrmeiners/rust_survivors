use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;
use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_enemies)
            .add_systems(FixedUpdate, enemy_movement.run_if(in_state(GameState::InGame)));
    }
}