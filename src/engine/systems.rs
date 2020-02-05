use crate::components::*;
use crate::physics::resources::*;
use crate::resources::*;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;
use nphysics2d::algebra::ForceType;
use nphysics2d::math::Force;
use nphysics2d::object::Body;
use nphysics2d::object::RigidBody;
use specs::Join;
use specs::Read;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::Write;
use std::collections::HashSet;

pub struct Draw<'a> {
    context: &'a mut Context,
}

impl<'a> Draw<'a> {
    pub fn new(context: &'a mut Context) -> Draw<'a> {
        Draw { context }
    }
}

impl<'a> System<'a> for Draw<'a> {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, Sprite>,
        Read<'a, GameOptions>,
        Read<'a, DebugInfo>,
        Read<'a, MyBodySet>,
        Read<'a, MyColliderSet>,
        ReadStorage<'a, ColliderComponent>,
    );

    fn run(
        &mut self,
        (
            delta,
            transform_storage,
            sprite_storage,
            options,
            debug_info,
            bodies,
            colliders,
            collider_storage,
        ): Self::SystemData,
    ) {
        for (transform, sprite) in (&transform_storage, &sprite_storage).join() {
            let transform = (*bodies)
                .0
                .rigid_body(transform.0)
                .expect("Body handle unusable for drawing!")
                .position();
            graphics::draw(
                self.context,
                &sprite.image,
                DrawParam {
                    dest: na::Point2::new(
                        transform.translation.x as f32,
                        transform.translation.y as f32,
                    )
                    .into(),
                    rotation: transform.rotation.angle() as f32,
                    scale: na::Vector2::new(1 as f32, 1 as f32).into(),
                    offset: na::Point2::new(0.5, 0.5).into(),
                    ..Default::default()
                },
            )
            .expect(&format!("Failed drawing sprite {:?}", sprite.image));
        }
        if options.draw_colliders {
            for (transform, collider) in (&transform_storage, &collider_storage).join() {
                let transform = (*bodies)
                    .0
                    .rigid_body(transform.0)
                    .expect("Body handle unusable for drawing!")
                    .position();
                let collider = (*colliders)
                    .0
                    .get(collider.0)
                    .expect("Collider handle unusable for drawing!");
                let color = Color::new(0.0, 1.0, 0.0, 1.0);
                let aabb = collider.shape().aabb(transform);
                let aabb_mins = aabb.mins();
                let aabb_half_extents = aabb.half_extents();

                let rectangle = graphics::Mesh::new_rectangle(
                    self.context,
                    graphics::DrawMode::stroke(1.0),
                    Rect::new(
                        aabb_mins.x as f32,
                        aabb_mins.y as f32,
                        (aabb_half_extents.x * 2.0) as f32,
                        (aabb_half_extents.y * 2.0) as f32,
                    ),
                    color,
                )
                .expect("Creating collider rectangle failed!");

                graphics::draw(self.context, &rectangle, (na::Point2::new(0.0, 0.0),)).expect(
                    "Drawing co
                llider rectangle failed!",
                );
            }
        }
    }
}

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        ReadExpect<'a, InputContext>,
        Write<'a, ActionContext>,
        Write<'a, GameOptions>,
    );
    fn run(&mut self, (input_context, mut action_context, mut options): Self::SystemData) {
        let pressed_keys = &input_context.pressed_keys;
        let last_pressed_keys = &input_context.last_pressed_keys;
        let active_mods = &input_context.active_mods;

        Input::map_keys(
            &[KeyCode::Up, KeyCode::W].iter().cloned().collect(),
            PlayerAction::MoveNorth,
            &mut action_context,
            pressed_keys,
        );
        Input::map_keys(
            &[KeyCode::Down, KeyCode::S].iter().cloned().collect(),
            PlayerAction::MoveSouth,
            &mut action_context,
            pressed_keys,
        );
        Input::map_keys(
            &[KeyCode::Left, KeyCode::A].iter().cloned().collect(),
            PlayerAction::MoveWest,
            &mut action_context,
            pressed_keys,
        );
        Input::map_keys(
            &[KeyCode::Right, KeyCode::D].iter().cloned().collect(),
            PlayerAction::MoveEast,
            &mut action_context,
            pressed_keys,
        );

        if pressed_keys.contains(&KeyCode::F1) && !last_pressed_keys.contains(&KeyCode::F1) {
            dbg!(&pressed_keys);
            options.draw_colliders = !options.draw_colliders;
        }
    }
}

impl Input {
    fn map_keys(
        keys: &HashSet<KeyCode>,
        action: PlayerAction,
        action_context: &mut ActionContext,
        pressed_keys: &HashSet<KeyCode>,
    ) {
        if pressed_keys.intersection(keys).count() > 0 {
            action_context.player_action_map.insert(action, true);
        } else {
            action_context.player_action_map.insert(action, false);
        }
    }
}

pub struct Act;

impl<'a> System<'a> for Act {
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
