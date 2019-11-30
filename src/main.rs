use engine::*;
use engine::components::*;

fn main() {
    // Create a new game and run it.
    let mut game = Game::new("Game Project", (800.0, 800.0));
    let player = engine::create_entity(&mut game, 400.0, 400.0, 3.0, 0.0)
        .with(Velocity { x: 1.0, y: 1.0 })
        .with(Sprite {image: engine::load_image(&game, "othersprite.png")})
        .build();
    engine::run(&mut game);
}
