use engine::components::*;
use engine::*;
use nalgebra::Vector2;
use ncollide2d::shape::Cuboid;
use ncollide2d::shape::ShapeHandle;

fn main() {
    // Create a new game and run it.
    let mut game = engine::new_game_state("Game Project", (800.0, 800.0));
    let player = create_player(&mut game);
    dbg!(&player);
    engine::run(&mut game);
}

fn create_player(game: &mut GameState) -> Entity {
    let mut image = engine::load_image(game, "\\othersprite.png");
    image.set_filter(FilterMode::Nearest);
    let player = engine::create_entity(game, 200.0, 200.0, 0.0)
        .with(Sprite { image })
        .with(Player {
            movement_speed: 1000.0,
        })
        .build();
    engine::add_collider(
        game,
        player,
        ShapeHandle::new(Cuboid::new(Vector2::new(10f64, 20f64))),
    );
    player
}
