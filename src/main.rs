extern crate engine;

use engine::Game;

fn main() {
    // Create a new game and run it.
    let mut game = Game::new("Game Project", (800.0, 800.0));
    
    let player = engine::new_entity(&mut game, 400.0, 400.0, 1.0, 0.0);
    
    engine::add_sprite_component(&mut game, player, "othersprite.png");
    
    engine::run(&mut game);
}
