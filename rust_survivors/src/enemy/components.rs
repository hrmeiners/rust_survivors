use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct ExpGem {
    pub value: f32,
}

#[derive(Component)]
pub struct Pickup;