use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use crate::player::components::Player;


pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..4 {
        let random_x = random::<f32>() * (window.width() / 2.0) + 200.0;
        let random_y = random::<f32>() * (window.height() / 2.0) + 200.0;

        commands.spawn((
            SpriteBundle {
                transform:  Transform {
                                translation: Vec3::new(random_x, random_y, 0.0),
                                scale: Vec3::new(0.1, 0.1, 0.1),
                                ..default()
                            },
                texture: asset_server.load("slime.png"),
                ..default()
            },
            Enemy,
        ));
    }
}


pub fn enemy_movement(
    time: Res<Time>,
    mut enemy_positions: Query<(&mut Transform, With<Enemy>, Without<Player>)>,
    player_position: Query<(&Transform, With<Player>)>
) {
    let (player_transform, player) = player_position.single();

    for(mut enemy_transform, _, _) in  &mut enemy_positions {
        //update enemy translation x
        if player_transform.translation.x < enemy_transform.translation.x {
            enemy_transform.translation.x -= 10.0 * time.delta_seconds();
        }
        if player_transform.translation.x > enemy_transform.translation.x {
            enemy_transform.translation.x += 10.0 * time.delta_seconds();
        }

        //update enemy translation y
        if player_transform.translation.y < enemy_transform.translation.y {
            enemy_transform.translation.y -= 10.0 * time.delta_seconds();
        }
        if player_transform.translation.y > enemy_transform.translation.y {
            enemy_transform.translation.y += 10.0 * time.delta_seconds();
        }
    }
}


