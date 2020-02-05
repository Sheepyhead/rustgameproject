use ggez::graphics;
use nphysics2d::object::DefaultBodyHandle;
use nphysics2d::object::DefaultColliderHandle;
use specs::DenseVecStorage;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformComponent(pub DefaultBodyHandle);

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

#[derive(Component)]
#[storage(VecStorage)]
pub struct ColliderComponent(pub DefaultColliderHandle);
