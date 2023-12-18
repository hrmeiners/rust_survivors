use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuItem;

#[derive(Component)]
pub struct MyButton {
    pub target: Buttons,
}

#[derive(PartialEq)]
pub enum Buttons {
    PoeButton,
    DogButton,
}
