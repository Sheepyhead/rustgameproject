extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::ImageSize;
use opengl_graphics::{Filter, GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

struct Sprite {
    image: graphics::Image, // The image to draw the sprite inside
    texture: Texture,       // The texture to draw on the image
    rotation: f64,          // The rotation of the sprite
    size_factor: f64,       // The size of the sprite
    position: (f64, f64),   // Position of the sprite
}

impl Sprite {
    pub fn get_actual_size(&self) -> (f64, f64) {
        (
            self.texture.get_width() as f64 * self.size_factor,
            self.texture.get_height() as f64 * self.size_factor,
        )
    }
}

pub struct Game {
    pub gl: GlGraphics,   // OpenGL drawing backend.
    sprites: Vec<Sprite>, // Sprites in world
    main_window: Window,  // The main game window
}

impl Game {
    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.main_window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
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

        let sprites: Vec<Sprite> = Vec::new();

        Game {
            gl,
            sprites,
            main_window,
        }
    }

    /// Adds sprite to world
    pub fn add_sprite(
        &mut self,
        file_name: &str,
        (x, y): (f64, f64), // Position
        rotation: f64,
        size_factor: f64,
    ) -> usize {
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);
        let texture = Texture::from_path(
            find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join(file_name),
            &texture_settings,
        )
        .unwrap();
        let image = graphics::Image::new().rect([
            0.0,
            0.0,
            texture.get_width() as f64 * size_factor,
            texture.get_height() as f64 * size_factor,
        ]);
        self.sprites.push(Sprite {
            image,
            texture,
            rotation,
            size_factor,
            position: (x, y),
        });
        self.sprites.len()
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let context = self.gl.draw_begin(args.viewport());

        // Clear the screen.
        clear(BLACK, &mut self.gl);

        // Render all sprites
        for sprite in self.sprites.iter() {
            let (size_x, size_y) = sprite.get_actual_size();
            let transform = context
                .transform
                .trans(sprite.position.0, sprite.position.1)
                .rot_rad(sprite.rotation)
                .trans(-(size_x / 2.0), -(size_y / 2.0));

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
