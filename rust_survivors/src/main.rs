use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//scene const variables
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
App::new()
    //window setup and default plugins
    .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WIDTH, HEIGHT).into(),
            title: "Rust Survivors".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))

    //inspector setup
    .add_plugins(WorldInspectorPlugin::new())
    .register_type::<Enemy>()
    .register_type::<Lifetime>()

    //startup systems
    //.add_systems(Startup, asset_loading)
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_basic_scene)

    //FixedUpdate systems
    .add_systems(FixedUpdate, sprite_movement)
    //.add_systems(FixedUpdate, enemy_shooting)
    //.add_systems(FixedUpdate, bullet_despawn)
 
    .run();
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle::default())
        .insert(Camera);

}

fn spawn_basic_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //spawn enemy
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("slime.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        },
        Direction2D {
            vertical: Direction::Up,
            horizontal: Direction::Left
        },
        Enemy {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        Name::new("Slime"),
    ));

    //spawn player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("poe_0.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Name::new("Player 1"),
    ));
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction2D, &mut Transform)>
) {
    for(mut direction_bundle, mut transform) in  &mut sprite_position {
        match direction_bundle.vertical {
            Direction::Up           => transform.translation.y += 150.0 * time.delta_seconds(),
            Direction::Down         => transform.translation.y -= 150.0 * time.delta_seconds(),
            Direction::Nonmoving    => transform.translation.y = 0.0,
            _                       => panic!("you cant have a horizontal direction in the vertical"),
        }

        match direction_bundle.horizontal {
            Direction::Right        => transform.translation.x += 150.0 * time.delta_seconds(),
            Direction::Left         => transform.translation.x -= 150.0 * time.delta_seconds(),
            Direction::Nonmoving    => transform.translation.x = 0.0,
            _                      => panic!("you cant have a vertical direction in the horizontal"),
        }

        if transform.translation.y > 200.0 {
            direction_bundle.vertical = Direction::Down;
        } else if transform.translation.y < -200.0 {
            direction_bundle.vertical = Direction::Up;
        }

        if transform.translation.x > 200.0 {
            direction_bundle.horizontal = Direction::Left;
        } else if transform.translation.x < -200.0 {
            direction_bundle.horizontal = Direction::Right;
        }
    }
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
}

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nonmoving,
}

#[derive(Component)]
pub struct Direction2D {
    vertical: Direction,
    horizontal: Direction,
}


#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Enemy {
    shooting_timer: Timer,
}

#[derive(Component)]
pub struct Player;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

#[derive(Component)]
pub struct Camera;