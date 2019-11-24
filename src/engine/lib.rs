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

mod entitycomponentsystem;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
}

static mut main_window: Option<Window> = None; // The main game window

pub fn run(game: &mut Game) {
    unsafe {
        if let Some(window) = &mut main_window {
            while let Some(e) = window.next() {
                render(game, &e);
                update(game);
            }
        }
    }
}

pub fn new(title: &str, size: (f64, f64)) -> Game {
    let opengl = OpenGL::V3_2;

    unsafe {
        main_window = Some(
            WindowSettings::new(title, [size.0, size.1])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
        );
    }

    let gl = GlGraphics::new(opengl);

    Game { gl }
}

fn render(game: &mut Game, event: &Event) {
    unsafe {
        if let Some(window) = &mut main_window {
            entitycomponentsystem::render(window, event);
        }
    }
}

fn update(game: &mut Game) {
    entitycomponentsystem::update();
}
