extern crate engine;

use engine::Game;

fn main() {
    // Create a new game and run it.
    let mut game = Game::new("Game Project");

    let player = Game::new_game_object(&mut game, 400.0, 400.0);

    let player = Game::get_game_object_mut(&mut game, player).unwrap();

    player.add_sprite("othersprite.png");

    Game::run(&mut game);
}
