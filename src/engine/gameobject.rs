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
    sprite_uuid: Uuid,
}

impl SpriteComponent {
    fn render(&mut self, window: &mut Window, event: &piston_window::Event) {
        window.draw_2d(event, |context, gfx, _| {
            self.scene.event(event);
            self.scene.draw(context.transform, gfx);
        });
    }

    fn update(&mut self, transform: &TransformComponent) {
        if let Some(sprite) = self.scene.child_mut(self.sprite_uuid) {
            sprite.set_position(transform.x, transform.y);
            sprite.set_rotation(transform.rotation);
            sprite.set_scale(transform.size, transform.size);
        }
    }
}

pub struct GameObject {
    pub transform: TransformComponent,
    sprite: Option<SpriteComponent>,
    factory: gfx_device_gl::Factory,
    command_buffer: gfx_device_gl::CommandBuffer,
}

impl GameObject {
    pub fn new((x, y): (f64, f64), factory: gfx_device_gl::Factory, command_buffer: gfx_device_gl::CommandBuffer) -> GameObject {
        GameObject {
            transform: TransformComponent {
                x,
                y,
                size: 1.0,
                rotation: 0.0,
            },
            factory,
            command_buffer,
            sprite: None,
        }
    }

    pub fn add_sprite(&mut self, file_name: &str) {
        let texture_settings = TextureSettings::new()
            .filter(Filter::Nearest)
            .mipmap(Filter::Nearest);

        let mut texture_context = TextureContext {
            factory: self.factory,
            encoder: self.command_buffer.into(),
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

        let sprite = Sprite::from_texture(texture);
        let mut scene = Scene::new();
        let sprite_uuid = scene.add_child(sprite);
        self.sprite = Option::from(SpriteComponent { scene, sprite_uuid });
    }

    pub fn get_transform(&self) -> &TransformComponent {
        &self.transform
    }

    pub fn update(&mut self) {
        let sprite_component = &mut self.sprite;
        let transform = &self.transform;
        
        if let Some(sprite_component) = sprite_component {
            sprite_component.update(transform);
        }
    }

    pub fn render(&mut self, event: &piston_window::Event) {
        let sprite_component = &mut self.sprite;

        if let Some(sprite_component) = sprite_component {
            sprite_component.render(window, event);
        }
    }
}
