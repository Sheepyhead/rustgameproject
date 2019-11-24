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

static mut entities: Vec<usize> = Vec::new();
static mut transforms: Vec<TransformComponent> = Vec::new();
static mut sprites: Vec<Option<SpriteComponent>> = Vec::new();

pub struct TransformComponent {
    x: f64,
    y: f64,
    size: f64,
    rotation: f64,
}

struct SpriteComponent {
    scene: sprite::Scene<piston_window::Texture<gfx_device_gl::Resources>>,
    sprite_id: Uuid,
}

impl SpriteComponent {
    fn render(&mut self, window: &mut Window, event: &piston_window::Event) {
        window.draw_2d(event, |context, gfx, _| {
            self.scene.event(event);
            self.scene.draw(context.transform, gfx);
        });
    }

    fn update(&mut self, transform: &TransformComponent) {
        if let Some(sprite) = self.scene.child_mut(self.sprite_id) {
            sprite.set_position(transform.x, transform.y);
            sprite.set_rotation(transform.rotation);
            sprite.set_scale(transform.size, transform.size);
        }
    }
}

pub fn new_entity(x: f64, y: f64, size: f64, rotation: f64) -> usize {
    let transform = TransformComponent {
        x,
        y,
        size,
        rotation,
    };
    let position = transforms.len();
    unsafe {
        transforms.push(transform);
        sprites.push(None);
    }
    position
}

pub fn add_sprite_component(
    entity: usize,
    file_name: &str,
    factory: gfx_device_gl::Factory,
    command_buffer: gfx_device_gl::CommandBuffer,
) {
    let texture_settings = TextureSettings::new()
        .filter(Filter::Nearest)
        .mipmap(Filter::Nearest);

    let mut texture_context = TextureContext {
        factory: factory,
        encoder: command_buffer.into(),
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
    let sprite_id = scene.add_child(sprite);
    unsafe {
        sprites[entity] = Some(SpriteComponent { scene, sprite_id });
    }
}

pub fn render(window: &mut Window, event: &Event) {
    for sprite_component in sprites.iter() {
        if let Some(sprite_component) = sprite_component {
            window.draw_2d(event, |context, gfx, _| {
                sprite_component.scene.event(event);
                sprite_component.scene.draw(context.transform, gfx);
            });
        }
    }
}

pub fn update() {
    for (position, sprite_component) in sprites.iter().enumerate() {
        if let Some(sprite_component) = sprite_component {
            let mut sprite = sprite_component
                .scene
                .child_mut(sprite_component.sprite_id)
                .unwrap();
            let transform = transforms[position];
            sprite.set_position(transform.x, transform.y);
            sprite.set_rotation(transform.rotation);
            sprite.set_scale(transform.size, transform.size);
        }
    }
}

pub fn get_transform(entity: usize) -> &'static TransformComponent {
    let transform = &transforms[entity];
    transform
}

pub fn get_transform_mut(entity: usize) -> &'static mut TransformComponent {
    let transform = &mut transforms[entity];
    transform
}
