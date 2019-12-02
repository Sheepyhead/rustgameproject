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
use std::cmp::Eq;
use std::collections::HashMap;
use systems::*;
pub use uuid::Uuid;

pub mod components;
pub mod resources;
pub mod systems;

pub struct GameState<'a, 'b> {
    ecs: ECS<'a, 'b>,
    context: Context,
    event_loop: EventsLoop,
    keyboard_maps: KeyboardMaps,
}

pub struct ECS<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

fn register_components(world: &mut World) {
    world.register::<Transform>();
    world.register::<Velocity>();
    world.register::<Sprite>();
}

fn insert_resources(world: &mut World) {
    world.insert(DeltaTime(0.0));
    world.insert(None::<InputContext>);
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
        .build();
    GameState {
        ecs: ECS { world, dispatcher },
        context,
        event_loop,
        keyboard_maps: KeyboardMaps::new(),
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
            let mut input_context = self.world.write_resource::<InputContext>();
            *input_context = InputContext {
                keyboard_context: context.keyboard_context.clone(),
                mouse_context: context.mouse_context.clone(),
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

pub fn add_button_down_mapping(
    game_state: &mut GameState,
    key_code: KeyCode,
    key_mods: KeyMods,
    function: fn(&Transform),
) {
    game_state
        .keyboard_maps
        .button_down
        .push(KeyboardMapping(key_code, key_mods, function))
}

pub fn add_button_up_mapping(
    game_state: &mut GameState,
    key_code: KeyCode,
    key_mods: KeyMods,
    function: fn(&Transform),
) {
    game_state
        .keyboard_maps
        .button_up
        .push(KeyboardMapping(key_code, key_mods, function))
}

pub fn add_button_repeat_mapping(
    game_state: &mut GameState,
    key_code: KeyCode,
    key_mods: KeyMods,
    function: fn(&Transform),
) {
    game_state
        .keyboard_maps
        .button_repeat
        .push(KeyboardMapping(key_code, key_mods, function))
}

pub struct KeyboardMaps {
    pub button_down: Vec<KeyboardMapping>,
    pub button_up: Vec<KeyboardMapping>,
    pub button_repeat: Vec<KeyboardMapping>,
}

impl KeyboardMaps {
    fn new() -> KeyboardMaps {
        KeyboardMaps {
            button_down: Vec::new(),
            button_up: Vec::new(),
            button_repeat: Vec::new(),
        }
    }
}

pub struct KeyboardMapping(KeyCode, KeyMods, fn(&Transform));
