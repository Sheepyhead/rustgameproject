extern crate engine;

fn main() {
    // Create a new game and run it.
    let mut game = engine::Game::new("Game Project");

    game.add_sprite("othersprite.png", (400.0, 400.0), 0.0, 2.0);

    game.run();
}
