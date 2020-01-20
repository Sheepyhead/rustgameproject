use engine::components::*;
use engine::*;

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
    engine::create_entity(game, 400.0, 400.0, 3.0, 0.0)
    .with(Sprite { image })
    .with(BoxCollider {
        width: 100.0,
        height: 100.0,
        solid: true,
    })
    .with(BoxCollisions {
        entities: Vec::new(),
    })
    .build();
    let mut image = engine::load_image(game, "\\othersprite.png");
    image.set_filter(FilterMode::Nearest);
    engine::create_entity(game, 200.0, 200.0, 3.0, 0.0)
        .with(Sprite { image })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(Player {
            movement_speed: 1000.0,
        })
        .with(BoxCollider {
            width: 100.0,
            height: 100.0,
            solid: true,
        })
        .with(BoxCollisions {
            entities: Vec::new(),
        })
        .build()
}
