use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod enemy;
mod player;
mod main_menu;
mod game_over;
mod ui;


#[derive(Debug, Hash, States, Default, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    MainMenu,
    Paused,
    InGame,
    GameOver,
}


pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState::<GameState>(Some(GameState::GameOver)));
    }
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                                    title: "Rust Survivors".to_string(),
                                    ..default()
                                }),
            ..default()
        }))

        .add_state::<GameState>()

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())

        .add_plugins(main_menu::MainMenuPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(game_over::GameOverPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(player::PlayerPlugin)

        .add_systems(FixedUpdate, exit_game)

    .run();
}
