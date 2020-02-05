use crate::resources::*;
use ggez::input::keyboard::*;
use specs::*;
use std::collections::HashSet;

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadExpect<'a, InputContext>,
        Write<'a, ActionContext>,
        Write<'a, GameOptions>,
    );
    fn run(&mut self, (input_context, mut action_context, mut options): Self::SystemData) {
        let pressed_keys = &input_context.pressed_keys;
        let last_pressed_keys = &input_context.last_pressed_keys;
        let active_mods = &input_context.active_mods;

        InputSystem::map_keys(
            &[KeyCode::Up, KeyCode::W].iter().cloned().collect(),
            PlayerAction::MoveNorth,
            &mut action_context,
            pressed_keys,
        );
        InputSystem::map_keys(
            &[KeyCode::Down, KeyCode::S].iter().cloned().collect(),
            PlayerAction::MoveSouth,
            &mut action_context,
            pressed_keys,
        );
        InputSystem::map_keys(
            &[KeyCode::Left, KeyCode::A].iter().cloned().collect(),
            PlayerAction::MoveWest,
            &mut action_context,
            pressed_keys,
        );
        InputSystem::map_keys(
            &[KeyCode::Right, KeyCode::D].iter().cloned().collect(),
            PlayerAction::MoveEast,
            &mut action_context,
            pressed_keys,
        );

        if pressed_keys.contains(&KeyCode::F1) && !last_pressed_keys.contains(&KeyCode::F1) {
            dbg!(&pressed_keys);
            options.draw_colliders = !options.draw_colliders;
        }
    }
}

impl InputSystem {
    fn map_keys(
        keys: &HashSet<KeyCode>,
        action: PlayerAction,
        action_context: &mut ActionContext,
        pressed_keys: &HashSet<KeyCode>,
    ) {
        if pressed_keys.intersection(keys).count() > 0 {
            action_context.player_action_map.insert(action, true);
        } else {
            action_context.player_action_map.insert(action, false);
        }
    }
}
