use crate::components::*;
use crate::resources::*;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;
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
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
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
    );

    fn run(&mut self, (delta, transform, sprite): Self::SystemData) {
        for (transform, sprite) in (&transform, &sprite).join() {
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
    }
}

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (ReadExpect<'a, InputContext>, Write<'a, ActionContext>);
    fn run(&mut self, (input_context, mut action_context): Self::SystemData) {
        let pressed_keys = &input_context.pressed_keys;
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
            action_context
                .player_action_map
                .insert(action, true);
        } else {
            action_context
                .player_action_map
                .insert(action, false);
        }
    }
}

pub struct Act;

impl<'a> System<'a> for Act {
    type SystemData = (Read<'a, ActionContext>, WriteStorage<'a, Transform>);
    fn run(&mut self, (action_context, mut transform): Self::SystemData) {
        for transform in (&mut transform).join() {
            if action_context.player_action_map[&PlayerAction::MoveNorth] {
                transform.y -= 10.0;
            }
            if action_context.player_action_map[&PlayerAction::MoveSouth] {
                transform.y += 10.0;
            }
            if action_context.player_action_map[&PlayerAction::MoveEast] {
                transform.x += 10.0;
            }
            if action_context.player_action_map[&PlayerAction::MoveWest] {
                transform.x -= 10.0;
            }
        }
    }
}
