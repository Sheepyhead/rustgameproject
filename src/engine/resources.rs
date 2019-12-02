use ggez::input::keyboard::KeyboardContext;
use ggez::input::mouse::MouseContext;

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct InputContext {
    pub keyboard_context: KeyboardContext,
    pub mouse_context: MouseContext,
}
