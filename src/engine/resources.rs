use ggez::input::keyboard::KeyboardContext;
use ggez::input::mouse::MouseContext;
use std::collections::HashMap;

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct InputContext {
    pub keyboard_context: KeyboardContext,
    pub mouse_context: MouseContext,
}

#[derive(Default)]
pub struct ActionContext {
    player_action_map: HashMap<PlayerAction, bool>,
}

impl ActionContext {
    pub fn new() -> ActionContext {
        let mut player_action_map: HashMap<PlayerAction, bool> = [
            (PlayerAction::MoveNorth, false),
            (PlayerAction::MoveSouth, false),
            (PlayerAction::MoveWest, false),
            (PlayerAction::MoveEast, false),
        ]
        .iter()
        .cloned()
        .collect();
        ActionContext { player_action_map }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum PlayerAction {
    MoveNorth,
    MoveSouth,
    MoveWest,
    MoveEast,
}
