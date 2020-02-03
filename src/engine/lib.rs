use components::*;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::event::EventsLoop;
use ggez::graphics;
pub use ggez::graphics::FilterMode;
use ggez::input::keyboard::*;
use ggez::timer;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::DefaultBodySet;
use nphysics2d::object::DefaultColliderSet;
use nphysics2d::object::RigidBodyDesc;
use nphysics2d::world::DefaultGeometricalWorld;
use nphysics2d::world::DefaultMechanicalWorld;
use resources::*;
pub use specs::world::Builder;
use specs::*;
pub use specs::{Entity, EntityBuilder};
use std::collections::HashSet;
use systems::*;
pub use uuid::Uuid;

pub mod components;
pub mod resources;
pub mod systems;

pub struct GameState<'a, 'b> {
    ecs: ECS<'a, 'b>,
    context: Context,
    event_loop: EventsLoop,
}

pub struct ECS<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

fn register_components(world: &mut World) {
    world.register::<Transform>();
    world.register::<Velocity>();
    world.register::<Sprite>();
    world.register::<Player>();
    world.register::<BoxCollider>();
}

fn insert_resources(world: &mut World) {
    world.insert(DeltaTime(0.0));
    world.insert(ActionContext::new());
    world.insert(GameOptions {
        draw_colliders: false,
    });
    world.insert(DebugInfo {
        info: vec!["".to_string()],
    });
    world
        .insert(DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)) as DefaultMechanicalWorld<f64>);
    world.insert(DefaultGeometricalWorld::new() as DefaultGeometricalWorld<f64>);
    world.insert(DefaultBodySet::new() as DefaultBodySet<f64>);
    world.insert(DefaultColliderSet::new() as DefaultColliderSet<f64>);
    world.insert(DefaultJointConstraintSet::new() as DefaultJointConstraintSet<f64>);
    world.insert(DefaultForceGeneratorSet::new() as DefaultForceGeneratorSet<f64>);
}

pub fn new_game_state(title: &str, size: (f32, f32)) -> GameState {
    let mut world = World::new();
    register_components(&mut world);
    insert_resources(&mut world);

    let (context, event_loop) = ContextBuilder::new(title, "TEST")
        .window_mode(WindowMode {
            width: size.0,
            height: size.1,
            ..Default::default()
        })
        .window_setup(WindowSetup {
            title: String::from(title),
            vsync: true,
            ..Default::default()
        })
        .build()
        .expect("Could not create ggez context!");

    let dispatcher = DispatcherBuilder::new()
        .with(Physics, "physics", &[])
        .with(Input, "input", &[])
        .with(Act, "act", &["input"])
        .build();
    GameState {
        ecs: ECS { world, dispatcher },
        context,
        event_loop,
    }
}

impl EventHandler for ECS<'_, '_> {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        {
            // Scoped so the pointer is thrown out as soon as it's no longer useful
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(timer::delta(context).as_secs_f64());
        }
        {
            let mut input = InputContext {
                pressed_keys: pressed_keys(context).clone(),
                last_pressed_keys: HashSet::new(),
                active_mods: active_mods(context),
                mouse_context: context.mouse_context.clone(),
            };
            if !self.world.has_value::<InputContext>() {
                self.world.insert(input);
            } else {
                let mut input_context = self.world.write_resource::<InputContext>();
                input.last_pressed_keys = input_context.pressed_keys.clone();
                *input_context = input;
            }
        }

        {
            let mut mechanical_world = self.world.write_resource::<DefaultMechanicalWorld<f64>>();
            let mut geometrical_world = self.world.write_resource::<DefaultGeometricalWorld<f64>>();
            let mut bodies = self.world.write_resource::<DefaultBodySet<f64>>();
            let mut colliders = self.world.write_resource::<DefaultColliderSet<f64>>();
            let mut joint_constraints = self
                .world
                .write_resource::<DefaultJointConstraintSet<f64>>();
            let mut force_generators = self.world.write_resource::<DefaultForceGeneratorSet<f64>>();

            (*mechanical_world).step(
                &mut *geometrical_world,
                &mut *bodies,
                &mut *colliders,
                &mut *joint_constraints,
                &mut *force_generators,
            );
        }

        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        {
            let mut draw_system = Draw::new(context);
            draw_system.run_now(&mut self.world);
        }

        graphics::present(context)
    }
}

pub fn create_entity<'a>(
    game_state: &'a mut GameState,
    x: f64,
    y: f64,
    rotation: f64,
) -> EntityBuilder<'a> {
    let mut body = RigidBodyDesc::new()
        .translation(Vector2::new(x, y))
        .rotation(rotation)
        .build();
    let mut body_set = *game_state.ecs.world.write_resource::<DefaultBodySet<f64>>();

    game_state.ecs.world.create_entity().with(Transform {
        body_handle: body_set.insert(body),
    })
}

pub fn run(game_state: &mut GameState) {
    match event::run(
        &mut game_state.context,
        &mut game_state.event_loop,
        &mut game_state.ecs,
    ) {
        Ok(_) => println!("Game exited cleanly"),
        Err(e) => println!("Error occurred: {}", e),
    }
}

pub fn load_image(game_state: &mut GameState, filename: &str) -> graphics::Image {
    graphics::Image::new(&mut game_state.context, filename).expect(&format!(
        "Failed loading image with file name: {}",
        filename
    ))
}
