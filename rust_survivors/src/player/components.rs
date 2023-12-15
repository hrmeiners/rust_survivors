use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MoveSpeed {
    pub move_speed: f32,
}

#[derive(Component)]
pub struct Health {
    pub max_hp: f32,
    pub current_hp: f32,
    pub regen: f32,
}

#[derive(Component)]
pub struct Camera;


#[derive(PartialEq)]
pub enum Characters {
    Poe,
    Dog,
    Gun,
}