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

pub struct ECS {
    entities: Vec<usize>,
    transforms: Vec<TransformComponent>,
    sprites: Vec<Option<SpriteComponent>>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: Vec::new(),
            transforms: Vec::new(),
            sprites: Vec::new(),
        }
    }
}

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

pub fn new_entity(ecs: &mut ECS, x: f64, y: f64, size: f64, rotation: f64) -> usize {
    let transform = TransformComponent {
        x,
        y,
        size,
        rotation,
    };
    let position = ecs.transforms.len();
    ecs.transforms.push(transform);
    ecs.sprites.push(None);
    ecs.entities.push(position);
    position
}

pub fn add_sprite_component(
    ecs: &mut ECS,
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
    ecs.sprites[entity] = Some(SpriteComponent { scene, sprite_id });
}

pub fn render(ecs: &mut ECS, window: &mut Window, event: &Event) {
    for (position, _) in ecs.entities.iter().enumerate() {
        if let Some(sprite_component) = &mut ecs.sprites[position] {
            sprite_component.render(window, &event);
        }
    }
}

pub fn update(ecs: &mut ECS) {
    for (position, _) in ecs.entities.iter().enumerate() {
        if let Some(sprite_component) = &mut ecs.sprites[position] {
            sprite_component.update(&ecs.transforms[position]);
        }
    }
}

pub fn get_transform(ecs: &ECS, entity: usize) -> &TransformComponent {
    let transform = &ecs.transforms[entity];
    transform
}

pub fn get_transform_mut(ecs: &mut ECS, entity: usize) -> &mut TransformComponent {
    let transform = &mut ecs.transforms[entity];
    transform
}
