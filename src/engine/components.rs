use ggez::graphics;
use specs::DenseVecStorage;
use specs::Entity;
use specs::{Component, VecStorage};

#[derive(Component, Debug, Copy, Clone)]
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

    pub fn move_until_touching(
        &self,
        self_transform: &Transform,
        self_velocity: &Velocity,
        other: &BoxCollider,
        other_transform: &Transform,
        delta: f64,
    ) -> (bool, Transform) {
        let mut future_self_transform = *self_transform;
        future_self_transform.x += self_velocity.x * delta;
        future_self_transform.y += self_velocity.y * delta;
        if self.collides_with(&future_self_transform, other, other_transform) {
            let dx = self_transform.x - other_transform.x;
            let dy = self_transform.y - other_transform.y;
            let x_speed = self_velocity.x * delta;
            let y_speed = self_velocity.y * delta;
            let x_radius = self.width / 2.0 - other.width / 2.0;
            let y_radius = self.height / 2.0 - other.height / 2.0;
            let x1 = if x_speed > 0.0 {
                (dx + x_radius) / x_speed
            } else {
                0.0
            };
            let x2 = if x_speed > 0.0 {
                (dx - x_radius) / x_speed
            } else {
                0.0
            };
            let y1 = if y_speed > 0.0 {
                (dy + y_radius) / y_speed
            } else {
                0.0
            };
            let y2 = if y_speed > 0.0 {
                (dy - y_radius) / y_speed
            } else {
                0.0
            };
            let x_min = f64::min(x1, x2);
            let x_max = f64::max(x1, x2);
            let y_min = f64::min(y1, y2);
            let y_max = f64::max(y1, y2);
            let mut collided = false;
            let mut updated_transform = *self_transform;
            if x_min < y_max && x_min > y_min {
                // Collision on the x side at x_min
                collided = true;
                updated_transform.x += x_speed * x_min;
            } else {
                updated_transform.x += x_speed * x_max;
            }
            if y_min < x_max && y_min > x_max {
                // Collision on the y side at y_min
                collided = true;
                updated_transform.y += y_speed * y_min;
            } else {
                updated_transform.y += y_speed * y_max;
            }
            (collided, updated_transform)
        } else {
            (false, future_self_transform)
        }
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct BoxCollisions {
    pub entities: Vec<Entity>,
}
