use bevy::{ math::vec2, math::vec3, 
    prelude::*, 
    sprite::collide_aabb::*};


//player vars
const PLAYER_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const PLAYER_SPEED: f32 = 500.0;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
App::new()
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

.add_systems(Startup, spawn_camera)
.add_systems(Startup, spawn_basic_scene)
//.add_systems(Startup, setup)

//.add_systems(Update, bevy::window::close_on_esc)

//.add_systems(FixedUpdate, (move_player, player_check_collisions))
//.add_systems(FixedUpdate, player_lose_health)
.run();
}


fn spawn_camera(mut commands: Commands) {
commands.spawn(Camera3dBundle {
transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
..default()
});
}

fn spawn_basic_scene(
mut commands: Commands,
mut meshes: ResMut<Assets<Mesh>>,
mut materials: ResMut<Assets<StandardMaterial>>,
) {
//spawn plane
commands.spawn(PbrBundle {
mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, subdivisions: 0 })),
material: materials.add(Color::rgb(0.3, 0.4, 0.3).into()),
..default()
});
//spawn cube
commands.spawn(PbrBundle {
mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
transform: Transform::from_xyz(0.0, 0.5, 0.0),
..default()
});
}
