use engine::components::*;
use engine::*;

fn main() {
    // Create a new game and run it.
    let mut game = Game::new("Game Project", (800.0, 800.0));
    let image = engine::load_image(&mut game, "\\othersprite.png");
    let player = engine::create_entity(&mut game, 400.0, 400.0, 3.0, 0.0)
        .with(Velocity { x: 1.0, y: 1.0 })
        .with(Sprite { image })
        .build();
    engine::run(&mut game);
}
