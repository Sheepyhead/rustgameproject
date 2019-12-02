use crate::components::*;
use crate::resources::*;
use ggez::Context;
use specs::Join;
use specs::Read;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;
use ggez::graphics;
use ggez::graphics::*;
use ggez::nalgebra as na;

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
            graphics::draw(self.context, &sprite.image, DrawParam::default().dest(na::Point2::new(transform.x as f32, transform.y as f32))).unwrap();
        }
    }
}
