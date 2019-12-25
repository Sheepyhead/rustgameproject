use crate::components::*;
use crate::resources::*;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;
use specs::world::EntitiesRes;
use specs::Entities;
use specs::Join;
use specs::Read;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::Write;
use specs::WriteStorage;
use std::collections::HashSet;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, BoxCollider>,
        ReadStorage<'a, BoxCollisions>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (delta, mut vel, mut transform_storage, collider_storage, collision_storage, entity_storage): Self::SystemData,
    ) {
        let delta = delta.0;
        for (entity, vel, pos) in (&entity_storage, &mut vel, &mut transform_storage).join() {
            if let Some(collider) = collider_storage.get(entity) {
                if let Some(collisions) = collision_storage.get(entity) {
                    for collided_entity in &collisions.entities {
                        if let Some(hit_collider) = collider_storage.get(*collided_entity) {
                            let hit_pos = &transform_storage.get(*collided_entity).expect("Transform missing from collided entity!");
                            if hit_collider.solid && collider.solid {
                            }
                        }
                    }
                }
            }
            pos.x += vel.vector.x * delta;
            pos.y += vel.vector.y * delta;
        }
    }
}

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
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, BoxCollider>,
        ReadStorage<'a, BoxCollisions>,
        Read<'a, GameOptions>,
    );

    fn run(
        &mut self,
        (delta, transform_storage, sprite_storage, collider_storage, collision_storage, options): Self::SystemData,
    ) {
        for (transform, sprite) in (&transform_storage, &sprite_storage).join() {
            graphics::draw(
                self.context,
                &sprite.image,
                DrawParam {
                    dest: na::Point2::new(transform.x as f32, transform.y as f32).into(),
                    rotation: transform.rotation as f32,
                    scale: na::Vector2::new(transform.size as f32, transform.size as f32).into(),
                    offset: na::Point2::new(0.5, 0.5).into(),
                    ..Default::default()
                },
            )
            .expect(&format!("Failed drawing sprite {:?}", sprite.image));
        }
        if options.draw_colliders {
            for (transform, collider, collision) in
                (&transform_storage, &collider_storage, &collision_storage).join()
            {
                let mut color = Color::new(0.0, 1.0, 0.0, 1.0);
                if collision.entities.len() > 0 {
                    color.g = 0.0;
                    color.r = 1.0;
                }
                let rectangle = graphics::Mesh::new_rectangle(
                    self.context,
                    graphics::DrawMode::stroke(1.0),
                    Rect::new(
                        (transform.x - collider.width / 2.0) as f32,
                        (transform.y - collider.height / 2.0) as f32,
                        collider.width as f32,
                        collider.height as f32,
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
        WriteStorage<'a, Velocity>,
        Read<'a, ActionContext>,
    );
    fn run(&mut self, (player, mut velocity, action_context): Self::SystemData) {
        for (player, velocity) in (&player, &mut velocity).join() {
            if action_context.player_action_map[&PlayerAction::MoveNorth] {
                velocity.vector.y = -player.movement_speed;
            } else if action_context.player_action_map[&PlayerAction::MoveSouth] {
                velocity.vector.y = player.movement_speed;
            } else {
                velocity.vector.y = 0.0;
            }
            if action_context.player_action_map[&PlayerAction::MoveEast] {
                velocity.vector.x = player.movement_speed;
            } else if action_context.player_action_map[&PlayerAction::MoveWest] {
                velocity.vector.x = -player.movement_speed;
            } else {
                velocity.vector.x = 0.0;
            }
        }
    }
}

pub struct Collide;

impl<'a> System<'a> for Collide {
    type SystemData = (
        ReadStorage<'a, BoxCollider>,
        ReadStorage<'a, Transform>,
        Entities<'a>,
        WriteStorage<'a, BoxCollisions>,
    );

    fn run(
        &mut self,
        (collider_storage, transform_storage, entity_storage, mut collisions_storage): Self::SystemData,
    ) {
        for (from_collider, from_transform, from_entity) in
            (&collider_storage, &transform_storage, &*entity_storage).join()
        {
            let collisions = collisions_storage
                .get_mut(from_entity)
                .expect("Entity with box collider missing box collision component!");
            collisions.entities.clear();
            for (to_collider, to_transform, to_entity) in
                (&collider_storage, &transform_storage, &*entity_storage).join()
            {
                if from_entity != to_entity {
                    if from_collider.collides_with(from_transform, to_collider, to_transform) {
                        collisions.entities.push(to_entity);
                    }
                }
            }
        }
    }
}
