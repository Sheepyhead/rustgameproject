extern crate engine;

fn main() {
    // Create a new game and run it.
    let game = engine::Game::new("Game Project");

    let player = game.new_game_object(400.0, 400.0);

    let player = game.get_game_object_mut(player).unwrap();

    player.add_sprite("othersprite.png");

    game.run();
}
