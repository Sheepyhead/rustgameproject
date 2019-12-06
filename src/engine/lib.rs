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
use resources::*;
pub use specs::world::Builder;
use specs::*;
pub use specs::{Entity, EntityBuilder};
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
}

fn insert_resources(world: &mut World) {
    world.insert(DeltaTime(0.0));
    world.insert(ActionContext::new())
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
        .with(UpdatePos, "update_pos", &[])
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
            let input = InputContext {
                pressed_keys: pressed_keys(context).clone(),
                active_mods: active_mods(context),
                mouse_context: context.mouse_context.clone(),
            };
            if !self.world.has_value::<InputContext>() {
                self.world.insert(input);
            } else {
                let mut input_context = self.world.write_resource::<InputContext>();
                // dbg!(input.as_ref());
                *input_context = input;
            }
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
    size: f64,
    rotation: f64,
) -> EntityBuilder<'a> {
    game_state.ecs.world.create_entity().with(Transform {
        x,
        y,
        size,
        rotation,
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
