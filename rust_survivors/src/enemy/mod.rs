use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;
use resources::*;
use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app .init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(GameState::InGame), spawn_enemies)
            .add_systems(FixedUpdate, tick_enemy_spawn_timer.run_if(in_state(GameState::InGame)))
            .add_systems(FixedUpdate, spawn_enemies_over_time.run_if(in_state(GameState::InGame)))
            .add_systems(FixedUpdate, enemy_movement.run_if(in_state(GameState::InGame)))
            .add_systems(FixedUpdate, enemy_check_collisions.run_if(in_state(GameState::InGame)));
    }
}