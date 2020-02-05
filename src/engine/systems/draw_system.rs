use self::nalgebra as na;
use crate::components::*;
use crate::physics::resources::*;
use crate::resources::*;
use ggez::*;
use graphics::*;
use specs::*;

pub struct DrawSystem<'a> {
    context: &'a mut Context,
}

impl<'a> DrawSystem<'a> {
    pub fn new(context: &'a mut Context) -> DrawSystem<'a> {
        DrawSystem { context }
    }
}

impl<'a> System<'a> for DrawSystem<'a> {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, Sprite>,
        Read<'a, GameOptions>,
        Read<'a, DebugInfo>,
        Read<'a, MyBodySet>,
        Read<'a, MyColliderSet>,
        ReadStorage<'a, ColliderComponent>,
        Read<'a, MyGeometricalWorld>,
    );

    fn run(
        &mut self,
        (
            delta,
            transform_storage,
            sprite_storage,
            options,
            debug_info,
            bodies,
            colliders,
            collider_storage,
            geometrical_world,
        ): Self::SystemData,
    ) {
        for (transform, sprite) in (&transform_storage, &sprite_storage).join() {
            let transform = (*bodies)
                .0
                .rigid_body(transform.0)
                .expect("Body handle unusable for drawing!")
                .position();
            graphics::draw(
                self.context,
                &sprite.image,
                DrawParam {
                    dest: na::Point2::new(
                        transform.translation.x as f32,
                        transform.translation.y as f32,
                    )
                    .into(),
                    rotation: transform.rotation.angle() as f32,
                    scale: na::Vector2::new(1 as f32, 1 as f32).into(),
                    offset: na::Point2::new(0.5, 0.5).into(),
                    ..Default::default()
                },
            )
            .expect(&format!("Failed drawing sprite {:?}", sprite.image));
        }
        if options.draw_colliders {
            for (transform, collider_handle) in (&transform_storage, &collider_storage).join() {
                let transform = (*bodies)
                    .0
                    .rigid_body(transform.0)
                    .expect("Body handle unusable for drawing!")
                    .position();
                let collider = (*colliders)
                    .0
                    .get(collider_handle.0)
                    .expect("Collider handle unusable for drawing!");
                let mut color = Color::new(0.0, 1.0, 0.0, 1.0);
                if let Some(contacts) =
                    geometrical_world
                        .0
                        .contacts_with(&colliders.0, collider_handle.0, true)
                {
                    if let Some(_) = contacts.peekable().peek() {
                        color.g = 0.0;
                        color.r = 1.0;
                    }
                }
                let aabb = collider.shape().aabb(transform);
                let aabb_mins = aabb.mins();
                let aabb_half_extents = aabb.half_extents();

                let rectangle = graphics::Mesh::new_rectangle(
                    self.context,
                    graphics::DrawMode::stroke(1.0),
                    Rect::new(
                        aabb_mins.x as f32,
                        aabb_mins.y as f32,
                        (aabb_half_extents.x * 2.0) as f32,
                        (aabb_half_extents.y * 2.0) as f32,
                    ),
                    color,
                )
                .expect("Creating collider rectangle failed!");

                graphics::draw(self.context, &rectangle, (na::Point2::new(0.0, 0.0),))
                    .expect("Drawing collider rectangle failed!");
            }
        }
    }
}
