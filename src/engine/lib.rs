extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use piston_window::PistonWindow as Window;
use piston_window::*;
pub use uuid::Uuid;

mod gameobject;

pub struct Game {
    gl: GlGraphics,  // OpenGL drawing backend.
    objects: Vec<gameobject::GameObject>,
}

static main_window: Option<Window> = None; // The main game window

pub fn run(game: &mut Game) {
    while let Some(e) = main_window.unwrap().next() {
        render(game, &e);
        update(game);
    }
}

pub fn new(title: &str, size: (f64, f64)) -> Game {
    let opengl = OpenGL::V3_2;

    main_window = Some(WindowSettings::new(title, [size.0,size.1])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap()); 

    let gl = GlGraphics::new(opengl);

    let objects: Vec<gameobject::GameObject> = Vec::new();

    Game {
        gl,
        objects,
    }
}

fn render(game: &mut Game, event: &Event) {
    let objects = &mut game.objects;

    for object in objects.iter_mut() {
        object.render(event);
    }
}

fn update(game: &mut Game) {
    let objects = &mut game.objects;

    for object in objects.iter_mut() {
        object.update();
    }
}

pub fn new_game_object(game: &mut Game, x: f64, y: f64) -> usize {
    let object = gameobject::GameObject::new((x, y), main_window.unwrap().factory.clone(), main_window.unwrap().factory.create_command_buffer());
    game.objects.push(object);
    game.objects.len()
}

pub fn get_game_object(game: &Game, index: usize) -> Option<&gameobject::GameObject> {
    game.objects.get(index)
}

pub fn get_game_object_mut(game: &mut Game, index: usize) -> Option<&mut gameobject::GameObject> {
    game.objects.get_mut(index)
}

pub fn get_main_window(game: &Game) -> &Window {
    &main_window.unwrap()
}

pub fn get_main_window_mut(game: &mut Game) -> &mut Window {
    &mut main_window.unwrap()
}
