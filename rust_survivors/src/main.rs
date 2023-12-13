use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod enemy;
mod player;
mod main_menu;


#[derive(Debug, Hash, States, Default, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    MainMenu,
    PauseMenu,
    InGame,
}


fn main() {
    App::new()
        //window setup and default plugins
        .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "Rust Survivors".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
            }))

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(player::PlayerPlugin)

        //add game states
        .add_state::<GameState>()

    .run();
}
