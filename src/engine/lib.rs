use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston_window::PistonWindow as Window;
use piston_window::*;
pub use uuid::Uuid;

pub mod entitycomponentsystem;

pub struct Game {
    ecs: entitycomponentsystem::ECS,
    main_window: Window,
}

impl Game {
    pub fn new(title: &str, size: (f64, f64), framerate: u64) -> Game {
        let opengl = OpenGL::V3_2;

        let mut main_window: Window = WindowSettings::new(title, [size.0, size.1])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        main_window.set_max_fps(framerate);

        Game {
            ecs: entitycomponentsystem::ECS::new(),
            main_window,
        }
    }
}

pub fn new_entity(game: &mut Game, x: f64, y: f64, size: f64, rotation: f64) -> usize {
    entitycomponentsystem::new_entity(&mut game.ecs, x, y, size, rotation)
}

pub fn add_sprite_component(game: &mut Game, entity: usize, file_name: &str) {
    entitycomponentsystem::add_sprite_component(
        &mut game.ecs,
        entity,
        file_name,
        game.main_window.factory.clone(),
        game.main_window.factory.create_command_buffer(),
    )
}

pub fn add_update_component(
    game: &mut Game,
    entity: usize,
    custom_update: fn(transform: &mut entitycomponentsystem::TransformComponent),
) {
    entitycomponentsystem::add_update_component(&mut game.ecs, entity, custom_update);
}

pub fn run(game: &mut Game) {
    let mut frames = 0;
    let mut time_passed = 0.0;
    loop {
        if let Some(event) = game.main_window.next() {
            //dbg!(&event);
            if let Some(_) = event.render_args() {
                render(game, &event);
                frames += 1;
            }
            if let Some(update_args) = event.update_args() {
                time_passed += update_args.dt;
                if time_passed > 1.0 {
                    let fps = (frames as f64) / time_passed;
                    dbg!(fps);
                    frames = 0;
                    time_passed = 0.0
                }
                update(game);
            }
            if let Some(key_pressed) = event.press_args() {
                if let Button::Keyboard(key_pressed) = key_pressed {
                    match key_pressed {
                        Key::Escape => break,
                        _ => (),
                    }
                }
            }
            if let Some(_) = event.close_args() {
                break;
            }
        }
    }
}

fn render(game: &mut Game, event: &Event) {
    entitycomponentsystem::render(&mut game.ecs, &mut game.main_window, event);
}

fn update(game: &mut Game) {
    entitycomponentsystem::update(&mut game.ecs);
}
