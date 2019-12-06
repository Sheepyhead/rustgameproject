use specs::NullStorage;
use ggez::graphics;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub rotation: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub image: graphics::Image,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Player {}