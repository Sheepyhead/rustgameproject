extern crate engine;

fn main() {
    // Create a new game and run it.
    let mut game = engine::Game::new("Game Project");

    game.add_sprite("othersprite.png", (400.0, 400.0), 0.0, 2.0);

    let object = game.new_game_object(300.0, 300.0);

    let _object = game.get_game_object_mut(object).unwrap();

    game.run();
}
