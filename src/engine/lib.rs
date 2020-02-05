use crate::resources::*;
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
use ncollide2d::shape::ShapeHandle;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::material::BasicMaterial;
use nphysics2d::material::MaterialHandle;
use nphysics2d::object::BodyPartHandle;
use nphysics2d::object::ColliderDesc;
use nphysics2d::object::DefaultBodySet;
use nphysics2d::object::DefaultColliderHandle;
use nphysics2d::object::DefaultColliderSet;
use nphysics2d::object::RigidBodyDesc;
use nphysics2d::world::DefaultGeometricalWorld;
use nphysics2d::world::DefaultMechanicalWorld;
use physics::resources::*;
pub use specs::world::Builder;
use specs::*;
pub use specs::{Entity, EntityBuilder};
use std::collections::HashSet;
pub use uuid::Uuid;
use systems::input_system::InputSystem;
use systems::action_system::ActionSystem;
use systems::draw_system::DrawSystem;

pub mod components;
pub mod physics;
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
    world.register::<TransformComponent>();
    world.register::<Sprite>();
    world.register::<Player>();
    world.register::<ColliderComponent>();
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
    world.insert(MyMechanicalWorld {
        0: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
    });
    world.insert(MyGeometricalWorld {
        0: DefaultGeometricalWorld::new(),
    });
    world.insert(MyBodySet {
        0: DefaultBodySet::new(),
    });
    world.insert(MyColliderSet {
        0: DefaultColliderSet::new(),
    });
    world.insert(MyJointConstraintSet {
        0: DefaultJointConstraintSet::new(),
    });
    world.insert(MyForceGeneratorSet {
        0: DefaultForceGeneratorSet::new(),
    });
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
        .with(InputSystem, "input_system", &[])
        .with(ActionSystem, "action_system", &["input_system"])
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
            let mut mechanical_world = self.world.write_resource::<MyMechanicalWorld>();
            let mut geometrical_world = self.world.write_resource::<MyGeometricalWorld>();
            let mut bodies = self.world.write_resource::<MyBodySet>();
            let mut colliders = self.world.write_resource::<MyColliderSet>();
            let mut joint_constraints = self.world.write_resource::<MyJointConstraintSet>();
            let mut force_generators = self.world.write_resource::<MyForceGeneratorSet>();

            mechanical_world.0.step(
                &mut geometrical_world.0,
                &mut bodies.0,
                &mut colliders.0,
                &mut joint_constraints.0,
                &mut force_generators.0,
            );
        }

        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        {
            let mut draw_system = DrawSystem::new(context);
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
    let body = RigidBodyDesc::new()
        .translation(Vector2::new(x, y))
        .rotation(rotation)
        .linear_damping(100.0)
        .build();

    let transform: TransformComponent;

    {
        let mut body_set = game_state.ecs.world.write_resource::<MyBodySet>();
        transform = TransformComponent {
            0: body_set.0.insert(body),
        };
    }

    game_state.ecs.world.create_entity().with(transform)
}

pub fn add_collider<'a>(
    game_state: &mut GameState,
    entity: Entity,
    shape: ShapeHandle<f64>,
) -> DefaultColliderHandle {
    let mut collider_set = game_state.ecs.world.write_resource::<MyColliderSet>();
    let body_handle = game_state
        .ecs
        .world
        .read_component::<TransformComponent>()
        .get(entity)
        .expect("Attempted to add collider to entity without transform!")
        .0;

    let collider = ColliderDesc::new(shape)
        .ccd_enabled(true)
        .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.2)))
        .build(BodyPartHandle(body_handle, 0));

    let collider_component = ColliderComponent {
        0: collider_set.0.insert(collider),
    };
    game_state
        .ecs
        .world
        .write_component::<ColliderComponent>()
        .insert(
            entity,
            ColliderComponent {
                ..collider_component
            },
        )
        .expect("Failed to add collider component!");
    collider_component.0
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
