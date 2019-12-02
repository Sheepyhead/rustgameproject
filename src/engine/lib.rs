use components::*;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::event::EventHandler;
use ggez::event::EventsLoop;
use ggez::graphics;
use ggez::timer;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use resources::*;
pub use specs::world::Builder;
pub use specs::EntityBuilder;
use specs::*;
use systems::*;
pub use uuid::Uuid;

pub mod components;
pub mod resources;
pub mod systems;

pub struct Game<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl Game<'_, '_> {
    pub fn new(title: &str, size: (f32, f32)) -> (Game, Context, EventsLoop) {
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Velocity>();
        world.register::<Sprite>();
        world.insert(DeltaTime(0.0));
        

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
        (Game { world, dispatcher }, context, event_loop)
    }
}

impl EventHandler for Game<'_, '_> {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        {
            // Scoped so the pointer is thrown out as soon as it's no longer useful
            let mut delta = self.world.write_resource::<DeltaTime>();
            *delta = DeltaTime(timer::delta(context).as_secs_f64());
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
    (game, _, _): &'a mut (Game, Context, EventsLoop),
    x: f64,
    y: f64,
    size: f64,
    rotation: f64,
) -> EntityBuilder<'a> {
    game.world.create_entity().with(Transform {
        x,
        y,
        size,
        rotation,
    })
}

pub fn run((game, context, event_loop): &mut (Game, Context, EventsLoop)) {
    match event::run(context, event_loop, game) {
        Ok(_) => println!("Game exited cleanly"),
        Err(e) => println!("Error occurred: {}", e),
    }
}

pub fn load_image(
    (_, context, _): &mut (Game, Context, EventsLoop),
    filename: &str,
) -> graphics::Image {
    graphics::Image::new(context, filename).unwrap()
}
