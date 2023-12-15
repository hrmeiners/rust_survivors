use bevy::prelude::*;

use super::events::GameOver;
use super::components::*;
use crate::GameState;
use crate::enemy::components::*;
use crate::player::components::*;


pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Thanks for playing!");
        commands.insert_resource(NextState::<GameState>(Some(GameState::GameOver)));
    }
}


pub fn clear_all_entities(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn_recursive();
    }

    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn_recursive();
    }

}


pub fn spawn_game_over_screen(
    mut commands: Commands,
) {
      //spawn camera
      commands.spawn(Camera2dBundle::default()).insert(GameOverScreenItem);

      //spawn play again button
      commands.spawn(NodeBundle {
          style: Style {
              width: Val::Percent(100.0),
              height: Val::Percent(100.0),
              align_items: AlignItems::Center,
              justify_content: JustifyContent::Center,
              ..default()
          },
          ..default()
      })
      .insert(GameOverScreenItem)
      //button shape, color, etc.
      .with_children(|parent| {
          parent.spawn(ButtonBundle {
              style: Style {
                  width: Val::Px(200.0),
                  height: Val::Px(80.0),
                  border: UiRect::all(Val::Px(5.0)),
                  justify_content: JustifyContent::Center,
                  align_items: AlignItems::Center,
                  ..default()
              },
              border_color: BorderColor(Color::BLACK),
              background_color: BackgroundColor(Color::ANTIQUE_WHITE),
              ..default()
          })
          //button text
          .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                  "Play Again?",
                  TextStyle {
                      font_size: 40.0,
                      color: Color::BLACK,
                      ..default()
                  },
              ));
          });
      });
    
/*
      //spawn main menu button
      commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .insert(GameOverScreenItem)
    //button shape, color, etc.
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::ANTIQUE_WHITE),
            ..default()
        })
        //button text
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Main Menu?",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });
    }); 
*/
}


pub fn game_over_button_controls (
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction, 
            &mut BackgroundColor, 
            &mut BorderColor, 
            &Children
        ), 
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState::<GameState>(Some(GameState::InGame)));
                println!("Playing again!");
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Play Again?".to_string();
                *color = Color::GRAY.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}


pub fn clear_game_over_entities(
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverScreenItem>>,
) {
    for entity in game_over_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}