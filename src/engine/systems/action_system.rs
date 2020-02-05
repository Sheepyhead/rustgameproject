use crate::components::*;
use crate::physics::resources::*;
use crate::resources::*;
use nphysics2d::math::*;
use nphysics2d::object::*;
use specs::*;

pub struct ActionSystem;

impl<'a> System<'a> for ActionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, TransformComponent>,
        Read<'a, ActionContext>,
        Write<'a, MyBodySet>,
    );
    fn run(&mut self, (player, transform, action_context, mut bodies): Self::SystemData) {
        for (player, body_handle) in (&player, &transform).join() {
            let mut force = nalgebra::Vector2::new(0f64, 0f64);
            if action_context.player_action_map[&PlayerAction::MoveNorth] {
                force.y = -player.movement_speed;
            } else if action_context.player_action_map[&PlayerAction::MoveSouth] {
                force.y = player.movement_speed;
            }
            if action_context.player_action_map[&PlayerAction::MoveEast] {
                force.x = player.movement_speed;
            } else if action_context.player_action_map[&PlayerAction::MoveWest] {
                force.x = -player.movement_speed;
            }
            if !force.is_empty() {
                let body = bodies
                    .0
                    .rigid_body_mut(body_handle.0)
                    .expect("Body no longer existing for handle!")
                    as &mut RigidBody<f64>;

                body.apply_force(
                    0,
                    &Force::<f64> {
                        linear: force,
                        angular: 0.0,
                    },
                    ForceType::VelocityChange,
                    false,
                );
            }
        }
    }
}
