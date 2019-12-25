use specs::Entity;
use ggez::graphics;
use specs::DenseVecStorage;
use specs::{Component, VecStorage};
use cgmath::Vector2;

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
    pub vector: Vector2<f64>,
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

pub enum BoxColliderSide {
    Up,
    Right,
    Down,
    Left,
}

impl BoxCollider {
    pub fn collides_with(
        &self,
        self_transform: &Transform,
        other: &BoxCollider,
        other_transform: &Transform,
    ) -> bool {
        // Expression courtesy of https://stackoverflow.com/a/306332 modified to suit an upper-left coordinate system
        self_transform.x - self.width / 2.0 < other_transform.x + other.width / 2.0
            && self_transform.x + self.width / 2.0 > other_transform.x - other.width / 2.0
            && self_transform.y - self.height / 2.0 < other_transform.y + other.height / 2.0
            && self_transform.y + self.height / 2.0 > other_transform.y - other.height / 2.0
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct BoxCollisions {
    pub entities: Vec<Entity>,
}
