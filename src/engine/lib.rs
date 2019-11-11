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
    pub gl: GlGraphics,  // OpenGL drawing backend.
    main_window: Window, // The main game window
    objects: Vec<gameobject::GameObject>,
}

impl Game {
    pub fn run(&mut self) {
        while let Some(e) = self.main_window.next() {
            self.render(&e);
            self.update();
        }
    }
    pub fn new(title: &str) -> Game {
        let opengl = OpenGL::V3_2;

        let main_window: Window = WindowSettings::new(title, [800, 800])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let gl = GlGraphics::new(opengl);

        let objects: Vec<gameobject::GameObject> = Vec::new();

        Game {
            gl,
            main_window: main_window,
            objects,
        }
    }

    fn render(&mut self, event: &Event) {
        let objects = &mut self.objects;

        for object in objects.iter_mut() {
            object.render(event);
        }
    }

    fn update(&mut self) {
        let objects = &mut self.objects;

        for object in objects.iter_mut() {
            object.update();
        }
    }

    pub fn new_game_object(&'static mut self, x: f64, y: f64) -> usize {
        let object = gameobject::GameObject::new((x, y), &mut self.main_window);
        self.objects.push(object);
        self.objects.len()
    }

    pub fn get_game_object(&self, index: usize) -> Option<&gameobject::GameObject> {
        self.objects.get(index)
    }

    pub fn get_game_object_mut(&mut self, index: usize) -> Option<&mut gameobject::GameObject> {
        self.objects.get_mut(index)
    }

    pub fn get_main_window(&self) -> &Window {
        &self.main_window
    }

    pub fn get_main_window_mut(&mut self) -> &mut Window {
        &mut self.main_window
    }
}
