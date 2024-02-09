use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::components::{Player, Health};


pub enum EnemyType {
    Slime,
}


pub fn spawn_enemy(
    enemy_type: EnemyType,
    enemy_translation: Vec3,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let enemy = match enemy_type {
        EnemyType::Slime => {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: enemy_translation,
                        scale: Vec3::new(0.1, 0.1, 0.1),
                        ..Default::default()
                    },
                    texture: asset_server.load("slime.png"),
                    ..default()
                },
                Health {
                    max_hp: 10.0,
                    current_hp: 10.0,
                    regen: 0.0,
                },
            )).id()
        },
    };

    // common components
    commands.entity(enemy)
        .insert(Enemy)                              // marker components
        .insert((                                   // physics components
            Collider::cuboid(80.0, 80.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(
                Group::from_bits(0b1000).unwrap(),  // membership: enemy
                Group::from_bits(0b0101).unwrap()   // filters: player, wall
            )
    ));
}


pub fn spawn_initial_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..4 {
        let random_x = random::<f32>() * (window.width() / 2.0) + 200.0;
        let random_y = random::<f32>() * (window.height() / 2.0) + 200.0;

        spawn_enemy(EnemyType::Slime, Vec3::new(random_x, random_y, 0.0), &mut commands, &asset_server);
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

        spawn_enemy(EnemyType::Slime, Vec3::new(random_x, random_y, 0.0), &mut commands, &asset_server);
    }
}



// COLLISIONS STILL NOT WORKINGGGGGGGGG
// MADE POST ON THE BEVY DISCORD SERVER ASKING FOR HELP
// MAYBE SM W COLLISION GROUPS 



pub fn enemy_check_collisions(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
    rapier_context: Res<RapierContext>,
    asset_server: Res<AssetServer>,
) {
    for (enemy, mut health, transform) in &mut enemy_query {

        for _contact_pair in rapier_context.contacts_with(enemy) {
            health.current_hp -= 1.0;
        }

        //enemy death code
        if health.current_hp <= 0.0 {

            // 100% chance to spawn exp gem upon death FOR TESTING
            if random::<f32>() <= 1.0 {
                spawn_exp_gem(10, &mut commands, transform, &asset_server);
            }

            //despawn enemy
            commands.entity(enemy).despawn_recursive();
        }
    }
}


pub fn spawn_exp_gem(
    exp_value: i32,
    commands: &mut Commands,
    enemy_transform: &Transform,
    asset_server: &Res<AssetServer>,
) {
    let exp_gem = match exp_value {
        10 => { 
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("exp_gem.png"),
                    transform:  *enemy_transform,
                    ..default()
                },
                Collider::cuboid(10.0, 10.0),
                ActiveEvents::COLLISION_EVENTS,
                ExpGem { value: exp_value },
            )).id()
        },

        _ => panic!("wrong exp value"),
    };

    commands.entity(exp_gem)                           // marker components
        .insert((                                   // physics components
            Collider::cuboid(80.0, 80.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(
                Group::from_bits(0b0010).unwrap(),  // membership: pickup
                Group::from_bits(0b0100).unwrap()   // filters: player
            )
    ));
}
