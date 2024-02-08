use bevy::prelude::*;

use super::components::*;

pub fn spawn_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            ..default()
    })
    .insert(UIItem)
    
    .with_children(|parent|{
        //top UI area
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
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
            //health area
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            })
            .insert(UIHealth)
            
            .with_children(|parent| {
                //heart 1
                parent.spawn((ImageBundle {
                    style: Style {
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("ui_heart.png")),
                    ..default()
                    },
                ));
                //heart 2
                parent.spawn((ImageBundle {
                    style: Style {
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("ui_heart.png")),
                    ..default()
                    },
                ));
                //heart 3
                parent.spawn((ImageBundle {
                    style: Style {
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("ui_heart.png")),
                    ..default()
                    },
                ));
                //heart 4
                parent.spawn((ImageBundle {
                    style: Style {
                        ..Default::default()
                    },
                    image: UiImage::new(asset_server.load("ui_heart.png")),
                    ..default()
                    },
                ));
            });

            //exp area
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            });

            //another area
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::GREEN),
                ..default()
            });

            //fourth area
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::GOLD),
                ..default()
            });
        });
    });
}

