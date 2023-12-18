use bevy::prelude::*;

use super::components::*;
use crate::GameState;
use crate::player::{events::*, components::*};


pub fn spawn_main_menu(
    mut commands: Commands,
) {
    //spawn camera
    commands.spawn(Camera2dBundle::default()).insert(MainMenuItem);

    //spawn poe button
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
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PoeButton",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        })
        .insert(MyButton {target: Buttons::PoeButton});
    })
    .insert(MainMenuItem);

    //spawn Dog Button
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        },
        ..default()
    })
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
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "DogButton",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        })
        .insert(MyButton {target: Buttons::DogButton});
    })
    .insert(MainMenuItem); 

}


pub fn main_menu_button_controls (
    mut commands: Commands,
    mut interaction_query: Query<
        (   
            &Interaction, 
            &mut BackgroundColor, 
            &mut BorderColor, 
            &Children,
            &MyButton
        ),
        (Changed<Interaction>, With<Button>,),
    >,
    mut text_query: Query<&mut Text>,
    mut character_choice_event_writer: EventWriter<CharacterChoice>
) {
    for (interaction, mut color, mut border_color, children, button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match button.target {
            Buttons::PoeButton => {
                match *interaction {
                    Interaction::Pressed => {
                        commands.insert_resource(NextState::<GameState>(Some(GameState::InGame)));
                        character_choice_event_writer.send(CharacterChoice {character: Characters::Poe});
                    }
                    Interaction::Hovered => {
                        *color = Color::BLUE.into();
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        text.sections[0].value = "Poe".to_string();
                        *color = Color::GRAY.into();
                        border_color.0 = Color::BLACK;
                    }
                }
            },
            
            Buttons::DogButton => {
                match *interaction {
                    Interaction::Pressed => {
                        commands.insert_resource(NextState::<GameState>(Some(GameState::InGame)));
                        character_choice_event_writer.send(CharacterChoice {character: Characters::Dog});
                    }
                    Interaction::Hovered => {
                        *color = Color::GREEN.into();
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        text.sections[0].value = "Doggie".to_string();
                        *color = Color::GRAY.into();
                        border_color.0 = Color::BLACK;
                    }
                }
            },
        };

    }
}


pub fn clear_main_menu (
    mut commands: Commands,
    menu_items_query: Query<Entity, &MainMenuItem>
) {
    for item in menu_items_query.iter() {
        commands.entity(item).despawn_recursive();
    }
}