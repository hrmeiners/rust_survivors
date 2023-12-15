use bevy::prelude::*;
use super::components::Characters;

#[derive(Event)]
pub struct CharacterChoice {
    pub character: Characters,
}
