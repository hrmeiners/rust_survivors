use bevy::prelude::*;
use Vec3;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


fn main() {
App::new()
    //window setup and default plugins
    .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1280.0, 720.0).into(),
            title: "Rust Survivors".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))

    //add game states
    .add_state::<GameState>()

    //inspector setup
    .add_plugins(WorldInspectorPlugin::new())

    //startup systems
    .add_systems(OnEnter(GameState::MainMenu), main_menu_controls)
    .add_systems(OnExit(GameState::MainMenu), clear_main_menu)
    

    .add_systems(FixedUpdate, button_system.run_if(in_state(GameState::MainMenu)))

    //FixedUpdate systems
    //.add_systems(Startup, spawn_basic_scene.run_if(in_state(GameState::InGame)))
    //.add_systems(FixedUpdate, player_movement.run_if(in_state(GameState::InGame)))
    //.add_systems(FixedUpdate, enemy_movement.run_if(in_state(GameState::InGame)))
 
    .run();
}

fn button_system (
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
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = Color::GREEN.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = Color::DARK_GRAY.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = Color::GRAY.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn clear_main_menu (
    mut commands: Commands,
    mut menu_items_query: Query<Entity, &MainMenuItem>
) {
    for item in &mut menu_items_query {
        commands.entity(item).despawn_recursive();
    }
}



fn main_menu_controls(
    mut commands: Commands,
) {
   commands.spawn(Camera2dBundle::default()).insert(MainMenuItem);
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
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BackgroundColor(Color::GRAY),
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



fn spawn_basic_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //spawn enemy
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("slime.png"),
            transform:  Transform {
                            translation: Vec3::new(100.0, 0.0, 0.0),
                            scale: Vec3::new(0.1, 0.1, 0.1),
                            ..default()
                        },
            ..default()
        },
        Enemy,
        Name::new("Slime"),
    ));

    //spawn player
    let player = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("poe_0.png"),
            transform:  Transform {
                            translation: Vec3::new(0.0, 0.0, 0.0),
                            scale: Vec3::new(0.25, 0.25, 0.25),
                            ..default()
                        },
            ..default()
        },
        Player {
            move_speed: 100.0,
        },
        Name::new("Poe Ratcho"),
    )).id();

    //spawn camera
    let camera_child = commands.spawn((
        Camera2dBundle::default(),
        Camera,
    )).id();

    //link player and camera so camera always follows player
    commands.entity(player).push_children(&[camera_child]);
}


fn enemy_movement(
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


fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_position: Query<(&Player, &mut Transform)>
) {
    for(player, mut transform) in  &mut player_position {

        if keys.pressed(KeyCode::W) {
            transform.translation.y += player.move_speed * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            transform.translation.y -= player.move_speed * time.delta_seconds()
        }
        if keys.pressed(KeyCode::D) {
            transform.translation.x += player.move_speed * time.delta_seconds()
        }
        if keys.pressed(KeyCode::A) {
            transform.translation.x -= player.move_speed * time.delta_seconds()
        } 
    }
}


//------- COMPONENTS ------

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Player {
    move_speed: f32,
}

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct MainMenuItem;

#[derive(Debug, Hash, States, Default, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default] MainMenu,
    PauseMenu,
    InGame,
}







/*
    BULLET SHOOTING CODE --> MAYBE REFACTOR INTO SOMETHING USABLE BUT IDK




#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}


fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}



fn enemy_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut enemies: Query<&mut Enemy>,
    time: Res<Time>
) {
    for mut enemy in &mut enemies {
        enemy.shooting_timer.tick(time.delta());
        if enemy.shooting_timer.just_finished() {
            //initial transform position of bullet
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0));
            
            //spawn bullet
            commands.spawn(SceneBundle {
                scene: bullet_assets.bullet_scene.clone(),
                transform: spawn_transform,
                ..Default::default()
            })
            .insert(Lifetime {
                timer: Timer::from_seconds(0.5, TimerMode::Once)
            })
            .insert(Name::new("Bullet"));
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}*/