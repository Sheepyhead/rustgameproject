extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate sprite;
extern crate uuid;

use opengl_graphics::{Filter, TextureSettings};
use piston_window::PistonWindow as Window;
use piston_window::*;
use sprite::*;
use std::rc::Rc;
pub use uuid::Uuid;

pub struct TransformComponent {
    x: f64,
    y: f64,
    size: f64,
    rotation: f64,
}

struct SpriteComponent {
    scene: sprite::Scene<piston_window::Texture<gfx_device_gl::Resources>>,
    window: Window,
    parent_transform: &TransformComponent,
}

impl SpriteComponent {
    fn render(&mut self, window: &mut Window, event: &piston_window::Event) {
        window.draw_2d(event, |context, gfx, _| {
            self.scene.event(event);
            self.scene.draw(context.transform, gfx);
        });
    }

    fn update(&mut self, args: &RenderArgs) {
        if let Some(sprite) = self.scene.children().first() {
            sprite.set_position(self.parent_transform.x, self.parent_transform.y);
            sprite.set_rotation(self.parent_transform.rotation);
            sprite.set_scale(self.parent_transform.size, self.parent_transform.size);
        }
    }
}

pub struct GameObject {
    pub transform: TransformComponent,
    sprite: Option<SpriteComponent>,
}

impl GameObject {
    pub fn new((x, y): (f64, f64)) -> GameObject {
        GameObject {
            transform: TransformComponent {
                x,
                y,
                size: 1.0,
                rotation: 0.0,
            },
            sprite: None,
        }
    }

    pub fn add_sprite(&mut self, file_name: &str, window: Window) {
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);

        self.sprite = Option::from(SpriteComponent {
            scene: Scene::new(),
            window,
            parent_transform: self.get_transform(),
        });

        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };

        let texture = Rc::new(
            Texture::from_path(
                &mut texture_context,
                find_folder::Search::ParentsThenKids(3, 3)
                    .for_folder("assets")
                    .unwrap()
                    .join(file_name),
                Flip::None,
                &texture_settings,
            )
            .unwrap(),
        );

        let mut sprite = Sprite::from_texture(texture);

        self.sprite.unwrap().scene.add_child(sprite);
    }

    pub fn get_transform(&self) -> &TransformComponent {
        &self.transform
    }
}
