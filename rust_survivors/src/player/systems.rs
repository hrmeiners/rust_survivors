use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::components::*;
use super::events::*;
use crate::game_over::events::GameOver;
use crate::enemy::components::*;


pub fn old_spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut character_choice_event_reader: EventReader<CharacterChoice>,
) {
    let window = window_query.get_single().unwrap();

    for choice in character_choice_event_reader.read() {
        match choice.character {

            Characters::Poe => {
                //spawn Poe
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
                    Player,
                    Name::new("Poe Ratcho"),
                    MoveSpeed {
                        move_speed: 100.0,
                    },
                    Health {
                        max_hp: 100.0,
                        current_hp: 100.0,
                        regen: 1.0,
                    },
                    Collider::cuboid(80.0, 80.0),
                    ActiveEvents::COLLISION_EVENTS,
                )) 
                .insert(CollisionGroups::new(
                    Group::from_bits(0b0100).unwrap(),  // membership: player
                    Group::from_bits(0b1111).unwrap()   // filters: enemy, player, pickup, wall
                ))
                .id();
                //spawn camera
                let camera_child = commands.spawn((
                    Camera2dBundle::default(),
                    Camera,
                )).id();
                //link player and camera so camera always follows player
                commands.entity(player).push_children(&[camera_child]);
            },

            Characters::Dog => {
                //spawn Dog
                let player = commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("Dog.png"),
                        transform:  Transform {
                                        translation: Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
                                        scale: Vec3::new(0.25, 0.25, 0.25),
                                        ..default()
                                    },
                        ..default()
                    }, 
                    Player,
                    Name::new("Doggie"),
                    MoveSpeed {
                        move_speed: 300.0,
                    },
                    Health {
                        max_hp: 10.0,
                        current_hp: 10.0,
                        regen: 5.0,
                    },
                    Collider::cuboid(80.0, 80.0),
                    ActiveEvents::COLLISION_EVENTS,
                ))
                .insert(CollisionGroups::new(
                    Group::from_bits(0b0100).unwrap(),  // membership: player
                    Group::from_bits(0b1111).unwrap()   // filters: enemy, player, pickup, wall
                )) 
                .id();
                //spawn camera
                let camera_child = commands.spawn((
                    Camera2dBundle::default(),
                    Camera,
                )).id();
                //link player and camera so camera always follows player
                commands.entity(player).push_children(&[camera_child]);
            },

            Characters::Gun => {
                //spawn Gun
                let player = commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("Gun.png"),
                        transform:  Transform {
                                        translation: Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
                                        scale: Vec3::new(0.25, 0.25, 0.25),
                                        ..default()
                                    },
                        ..default()
                    }, 
                    Player,
                    Name::new("Gun"),
                    MoveSpeed {
                        move_speed: 50.0,
                    },
                    Health {
                        max_hp: 1000.0,
                        current_hp: 1000.0,
                        regen: 1000.0,
                    },
                    Collider::cuboid(80.0, 80.0),
                    ActiveEvents::COLLISION_EVENTS,
                )) 
                .insert(CollisionGroups::new(
                    Group::from_bits(0b0100).unwrap(),  // membership: player
                    Group::from_bits(0b1111).unwrap()   // filters: enemy, player, pickup, wall
                ))
                .id();
                //spawn camera
                let camera_child = commands.spawn((
                    Camera2dBundle::default(),
                    Camera,
                )).id();
                //link player and camera so camera always follows player
                commands.entity(player).push_children(&[camera_child]);
            },
        };
    }
}


// ----------------------------------------------------------------------------
// REWORKING SPAWN PLAYER

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut character_choice_event_reader: EventReader<CharacterChoice>,
) {
    let window = window_query.get_single().unwrap();

    for choice in character_choice_event_reader.read() {
        let player = match choice.character {

            Characters::Poe => {
                //spawn Poe
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("poe_0.png"),
                        transform:  Transform {
                                        translation: Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
                                        scale: Vec3::new(0.25, 0.25, 0.25),
                                        ..default()
                                    },
                        ..default()
                    }, 
                    Name::new("Poe Ratcho"),
                    MoveSpeed {
                        move_speed: 100.0,
                    },
                    Health {
                        max_hp: 100.0,
                        current_hp: 100.0,
                        regen: 1.0,
                    },
                )).id()
            },

            Characters::Dog => {
                //spawn Dog
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("Dog.png"),
                        transform:  Transform {
                                        translation: Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
                                        scale: Vec3::new(0.25, 0.25, 0.25),
                                        ..default()
                                    },
                        ..default()
                    }, 
                    Name::new("Doggie"),
                    MoveSpeed {
                        move_speed: 300.0,
                    },
                    Health {
                        max_hp: 10.0,
                        current_hp: 10.0,
                        regen: 5.0,
                    },
                )).id()
            },

            Characters::Gun => {
                //spawn Gun
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("Gun.png"),
                        transform:  Transform {
                                        translation: Vec3::new(window.width()/2.0, window.height()/2.0, 0.0),
                                        scale: Vec3::new(0.25, 0.25, 0.25),
                                        ..default()
                                    },
                        ..default()
                    }, 
                    Name::new("Gun"),
                    MoveSpeed {
                        move_speed: 50.0,
                    },
                    Health {
                        max_hp: 1000.0,
                        current_hp: 1000.0,
                        regen: 1000.0,
                    },
                )).id()
            },
        };

        commands.entity(player)
            .insert(Player)                             // marker components
            .insert((                                   // physics components
                Collider::cuboid(80.0, 80.0),
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(
                    Group::from_bits(0b0100).unwrap(),  // membership: player
                    Group::from_bits(0b1111).unwrap()   // filters: enemy, player, pickup, wall
                )
        ));
            
        //spawn camera
        let camera_child = commands.spawn((
            Camera2dBundle::default(),
            Camera,
        )).id();

        //link player and camera so camera always follows player
        commands.entity(player).push_children(&[camera_child]);
    }
}



// ----------------------------------------------------------------------



pub fn player_movement(
    mut player_query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, move_speed ) in &mut player_query {
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

        transform.translation += direction * move_speed.move_speed * time.delta_seconds();
    }
}


pub fn player_check_collisions(
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<Entity, With<Player>>,
    pickup_query: Query<Entity, With<Pickup>>,
    mut player_health_query: Query<&mut Health, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    rapier_context: Res<RapierContext>,
) {
    let player = player_query.single();
    let mut player_health = player_health_query.single_mut();

    //get all colliders in contact with player
    for contact_pair in rapier_context.contacts_with(player) {
        let mut is_enemy = false; 

        // get other entity
        let other_collider = if contact_pair.collider1() == player {
            contact_pair.collider2()
        } else {
            contact_pair.collider1()
        }; 
        
        // collision with enemy
        for enemy in enemy_query.iter() {
            if other_collider == enemy {
                player_health.current_hp -= 1.0;
                is_enemy = true;

                //if player has no more health, end the game and the function
                if player_health.current_hp <= 0.0 {
                    game_over_event_writer.send(GameOver);
                    return;
                }

                break;
            }
        }

        // collision with pickup
        if !is_enemy {

            for pickup in pickup_query.iter() {
                if other_collider == pickup {
                    // delete pickup
                    // add to exp bar
                }
            }
        }

    }

    
}
