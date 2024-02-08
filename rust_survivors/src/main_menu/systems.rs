use bevy::prelude::*;

use super::components::*;
use crate::GameState;
use crate::player::{events::*, components::*};


pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    //spawn camera
    commands.spawn(Camera2dBundle::default()).insert(MainMenuItem);

    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
            ..default()
    })
    .insert(MainMenuItem)

    .with_children(|parent| {
        //title area
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })

        .with_children(|parent| {
            //left logo
            parent.spawn((ImageBundle {
                style: Style {
                    ..Default::default()
                },
                image: UiImage::new(asset_server.load("gun_brand.png")),
                ..default()
                },
            ));
            
            //title
            parent.spawn(TextBundle {
                style:  Style {
                    width: Val::Percent(30.0),
                    ..default()
                },
                text: Text::from_section(
                    "Rust Survivors",
                    TextStyle {
                        font: asset_server.load("fonts/KingArthurSpecialNormal-XGPP.ttf"),
                        font_size: 90.0,
                        color: Color::BLACK.into(),
                    })
                    .with_alignment(TextAlignment::Center),
                ..default()
            });

            //right logo
            parent.spawn((ImageBundle {
                style: Style {
                    ..Default::default()
                },
                image: UiImage {
                    texture: asset_server.load("gun_brand.png"),
                    flip_x: true,
                    ..default()
                },
                ..default()
                },
            ));
        });

        //button area
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::Center,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 500.0, 0.0),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })

        .with_children(|parent| {
            //poe button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .insert(MyButton {target: Buttons::PoeButton})
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

            //dog button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .insert(MyButton {target: Buttons::DogButton})
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
            
            //gun button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .insert(MyButton {target: Buttons::GunButton})
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