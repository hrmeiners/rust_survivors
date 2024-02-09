use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::components::{Player, Health};


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
            Health {
                max_hp: 10.0,
                current_hp: 10.0,
                regen: 0.0,
            },
            Collider::cuboid(80.0, 80.0),
            CollisionGroups {
                memberships: Group::GROUP_2,    //enemy colliders will be group 2
                filters: Group::GROUP_1,        //player will be group 1
            },
        ));
    }
}


pub fn enemy_movement(
    mut enemy_positions: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_position: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_position.single();

    for mut enemy_transform in  enemy_positions.iter_mut() {
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


pub fn tick_enemy_spawn_timer(
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}


pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

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
            Health {
                max_hp: 10.0,
                current_hp: 10.0,
                regen: 0.0,
            },
            Collider::cuboid(80.0, 80.0),
            CollisionGroups {
                memberships: Group::GROUP_2,    //enemy colliders will be group 2
                filters: Group::GROUP_1,        //player will be group 1
            },
        ));
    }
}


//collision detection is not working 100%
//enemies are still deleting each other --> might be something wrong with the collision groups thing

pub fn enemy_check_collisions(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
    rapier_context: Res<RapierContext>,
    asset_server: Res<AssetServer>,
) {
    for (enemy, mut health, transform) in &mut enemy_query {

        // MIGHT NEED TO FIX CODE
        // right now just says that enemies lose health for EVERYTHING they are in contact with
        // needs to be just player
        for _contact_pair in rapier_context.contacts_with(enemy) {
            health.current_hp -= 1.0;
        }

        //enemy death code
        if health.current_hp <= 0.0 {

            // 100% chance to spawn exp gem upon death FOR TESTING
            if random::<f32>() <= 1.0 {
                spawn_exp_gem(&mut commands, transform, &asset_server);
            }

            //despawn enemy
            commands.entity(enemy).despawn_recursive();
        }
    }
}


pub fn spawn_exp_gem(
    commands: &mut Commands,
    enemy_transform: &Transform,
    asset_server: &Res<AssetServer>,
) {
    //spawn exp gemstone right where the enemy died
    let _exp_gemstone = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("exp_gem.png"),
            transform:  *enemy_transform,
            ..default()
        },
        Collider::cuboid(10.0, 10.0),
        ActiveEvents::COLLISION_EVENTS,
        ExpGem { value: 10.0 },
        Pickup,
    ));

}