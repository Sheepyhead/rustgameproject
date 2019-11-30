use crate::components::Position;
use specs::WriteStorage;
use crate::components::Velocity;
use specs::ReadStorage;
use specs::System;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>,  WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos) : Self::SystemData) {
        use specs::Join;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        }
    }
}

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}
