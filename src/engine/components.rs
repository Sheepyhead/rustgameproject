use ggez::graphics;
use specs::DenseVecStorage;
use specs::{Component, VecStorage};
use nphysics2d::object::DefaultBodyHandle;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub body_handle: DefaultBodyHandle
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
#[storage(DenseVecStorage)]
pub struct Player {
    pub movement_speed: f64,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct BoxCollider {
    pub width: f64,
    pub height: f64,
    pub solid: bool,
}
