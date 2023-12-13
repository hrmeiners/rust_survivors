use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::components::*;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let player = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("poe_0.png"),
            transform:  Transform {
                            translation: Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
                            scale: Vec3::new(0.25, 0.25, 0.25),
                            ..default()
                        },
            ..default()
        }, 
        Player {
            move_speed: 100.0,
        },
        Name::new("Poe Ratcho"),
        Collider::cuboid(80.0, 100.0), //way too wide
        

    )).id();

    //spawn camera
    let camera_child = commands.spawn((
        Camera2dBundle::default(),
        Camera,
    )).id();

    //link player and camera so camera always follows player
    commands.entity(player).push_children(&[camera_child]);
}


pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut player_query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * player.move_speed * time.delta_seconds();
    }
}
