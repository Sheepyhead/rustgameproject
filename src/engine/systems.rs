use crate::components::*;
use crate::resources::*;
use ggez::graphics;
use ggez::graphics::*;
use ggez::nalgebra as na;
use ggez::Context;
use specs::Join;
use specs::Read;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

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
    type SystemData = (Option<Read<'a, InputContext>>, ReadStorage<'a, Transform>);
    fn run(&mut self, (input_context, transform): Self::SystemData) {
        if let Some(input_context) = input_context {
            for (transform) in (&transform).join() {

            }
        }
    }
}
