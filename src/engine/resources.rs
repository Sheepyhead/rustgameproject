use ggez::event::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::input::mouse::MouseContext;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Debug)]
pub struct InputContext {
    pub pressed_keys: HashSet<KeyCode>,
    pub last_pressed_keys: HashSet<KeyCode>,
    pub active_mods: KeyMods,
    pub mouse_context: MouseContext,
}

#[derive(Default)]
pub struct ActionContext {
    pub player_action_map: HashMap<PlayerAction, bool>,
}

impl ActionContext {
    pub fn new() -> ActionContext {
        let player_action_map: HashMap<PlayerAction, bool> = [
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum PlayerAction {
    MoveNorth,
    MoveSouth,
    MoveWest,
    MoveEast,
}

#[derive(Default, Debug)]
pub struct GameOptions {
    pub draw_colliders: bool,
}

#[derive(Default, Debug)]
pub struct DebugInfo {
    pub info: Vec<String>,
}
