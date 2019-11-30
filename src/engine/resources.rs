use ggez::Context;

#[derive(Default)]
pub struct DeltaTime(pub f64);

pub struct DrawContext<'a>(pub &'a mut Context);
