use std::rc::Rc;
use ggez::Context;
use crate::components::*;
use crate::resources::*;
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

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Transform>;

    fn run(&mut self, transform: Self::SystemData) {
        for transform in transform.join() {
            //println!("Hello, {:?}", &transform);
        }
    }
}

pub struct Draw;

impl<'a> System<'a> for Draw {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Sprite>,
    );

    fn run(&mut self, (delta, transform, sprite): Self::SystemData) {
        for (transform, sprite) in (&transform, &sprite).join() {
            
        }
    }
}
