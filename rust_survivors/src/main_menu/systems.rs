use bevy::prelude::*;

use super::components::*;
use crate::GameState;
use crate::player::{events::*, components::*};


//MAYBE HAVE TWO UI NODES INSTEAD OF ONE? A BUTTON UI NODE AND A TITLE CARD UI NODE?
//PERHAPS HAVE SCROLLING LIST OF CHARACTERS?



pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //spawn camera
    commands.spawn(Camera2dBundle::default()).insert(MainMenuItem);

    //root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            ..default()
        })
        
        //logo
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(300.0),
                        margin: UiRect::top(Val::VMin(5.0)),
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                //logo image
                UiImage::new(asset_server.load("gun_brand.png")),
            ));
        })

        //main menu title
        .with_children(|parent|{
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(250.0),
                    margin: UiRect::top(Val::VMin(5.0)),
                    justify_self: JustifySelf::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
                },
            ))
            //text
            .with_children(|parent|{
                parent.spawn(TextBundle::from_section(
                    "Rust Survivors",
                    TextStyle {
                        font_size: 80.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            });
        })

        //------------ Buttons -------------------
        //Poe button
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::ANTIQUE_WHITE),
                ..default()
            })
            //marker components
            .insert(MyButton {target: Buttons::PoeButton})
            .insert(MainMenuItem)
            //text
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Poe",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            });
        })

        //Dog Button
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::ANTIQUE_WHITE),
                ..default()
            })
            //marker components
            .insert(MyButton {target: Buttons::DogButton})
            .insert(MainMenuItem)
            //text
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Dog",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            });
        })

        //Gun Button
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::ANTIQUE_WHITE),
                ..default()
            })
            //marker components
            .insert(MyButton {target: Buttons::GunButton})
            .insert(MainMenuItem)
            //text
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Gun",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            });
        });
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
                        text.sections[0].value = "Dog".to_string();
                        *color = Color::GRAY.into();
                        border_color.0 = Color::BLACK;
                    }
                }
            },

            Buttons::GunButton => {
                match *interaction {
                    Interaction::Pressed => {
                        commands.insert_resource(NextState::<GameState>(Some(GameState::InGame)));
                        character_choice_event_writer.send(CharacterChoice {character: Characters::Gun});
                    }
                    Interaction::Hovered => {
                        *color = Color::ORANGE_RED.into();
                        border_color.0 = Color::WHITE;
                    }
                    Interaction::None => {
                        text.sections[0].value = "Gun".to_string();
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