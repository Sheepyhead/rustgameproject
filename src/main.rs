extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle::square;
use opengl_graphics::{Filter, GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

pub struct Position {
    pub x: f64,
    pub y: f64,
}

struct Sprite {
    image: graphics::Image, // The image to draw the sprite inside
    texture: Texture,       // The texture to draw on the image
    rotation: f64,          // The rotation of the sprite
    size: f64,              // The size of the sprite
    position: Position,     // Position of the sprite
}

pub struct App {
    pub gl: GlGraphics,   // OpenGL drawing backend.
    sprites: Vec<Sprite>, // Sprites in world
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        let sprites: Vec<Sprite> = Vec::new();
        App { gl, sprites }
    }

    /// Adds sprite to world
    pub fn add_sprite(
        &mut self,
        asset_path: std::path::PathBuf,
        position: Position,
        rotation: f64,
        size: f64,
    ) -> usize {
        let image = graphics::Image::new().rect(square(0.0, 0.0, size));
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);
        let texture = Texture::from_path(asset_path, &texture_settings).unwrap();
        self.sprites.push(Sprite {
            image,
            texture,
            rotation,
            size,
            position,
        });
        self.sprites.len()
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let context = self.gl.draw_begin(args.viewport());

        // Clear the screen.
        clear(BLACK, &mut self.gl);
        for sprite in self.sprites.iter() {
            let transform = context
                .transform
                .trans(sprite.position.x, sprite.position.y)
                .rot_rad(sprite.rotation)
                .trans(-(sprite.size / 2.0), -(sprite.size / 2.0));

            self.gl.image(
                &sprite.image,
                &sprite.texture,
                &DrawState::default(),
                transform,
            );
        }

        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {
        for sprite in &mut self.sprites {
            sprite.rotation += 2.0 * args.dt;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    app.add_sprite(
        find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap()
            .join("sprite.png"),
        Position { x: 0.0, y: 0.0 },
        0.0,
        50.0,
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
