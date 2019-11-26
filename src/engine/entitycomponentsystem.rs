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
    updates: Vec<Option<UpdateComponent>>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: Vec::new(),
            transforms: Vec::new(),
            sprites: Vec::new(),
            updates: Vec::new(),
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
    ecs.entities.push(position);
    ecs.transforms.push(transform);
    ecs.sprites.push(None);
    ecs.updates.push(None);
    position
}

pub struct TransformComponent {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub rotation: f64,
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

struct UpdateComponent {
    custom_update: fn(transform: &mut TransformComponent),
}

impl UpdateComponent {
    fn update(&self, transform: &mut TransformComponent) {
        (self.custom_update)(transform);
    }
}

pub fn add_update_component(
    ecs: &mut ECS,
    entity: usize,
    custom_update: fn(transform: &mut TransformComponent),
) {
    ecs.updates[entity] = Some(UpdateComponent { custom_update });
}

pub fn render(ecs: &mut ECS, window: &mut Window, event: &Event) {
    window.draw_2d(event, |_, gfx, _| {
        clear([0.0, 0.0, 0.0, 0.0], gfx);
    });
    for (position, _) in ecs.entities.iter().enumerate() {
        if let Some(sprite_component) = &mut ecs.sprites[position] {
            sprite_component.render(window, &event);
        }
    }
}

pub fn update(ecs: &mut ECS) {
    for (position, _) in (&ecs.entities).iter().enumerate() {
        if let Some(sprite_component) = &mut ecs.sprites[position] {
            sprite_component.update(&ecs.transforms[position]);
        }
        if let Some(update_component) = &ecs.updates[position] {
            update_component.update(&mut ecs.transforms[position]);
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
