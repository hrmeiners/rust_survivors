use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;
use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), clear_main_menu)
            .add_systems(FixedUpdate, main_menu_button_controls.run_if(in_state(GameState::MainMenu)));
    }
}