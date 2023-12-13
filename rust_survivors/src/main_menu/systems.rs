use bevy::prelude::*;

use super::components::*;
use crate::GameState;


pub fn spawn_main_menu(
    mut commands: Commands,
) {
    //spawn camera
    commands.spawn(Camera2dBundle::default()).insert(MainMenuItem);
    //spawn button
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
    .insert(MainMenuItem)
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
                "Button",
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
    mut commands: Commands,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                commands.insert_resource(NextState::<GameState>(Some(GameState::InGame)));
                println!("Transitioned to InGame");
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Play Game".to_string();
                *color = Color::GRAY.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}


pub fn clear_main_menu (
    mut commands: Commands,
    mut menu_items_query: Query<Entity, &MainMenuItem>
) {
    for item in &mut menu_items_query {
        commands.entity(item).despawn_recursive();
    }
}

