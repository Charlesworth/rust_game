use sdl2::render::Renderer;
use events::Events;

/// Bundles the Phi abstractions in a single structure which
/// can be passed easily between functions.
pub struct WindowIO<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    fn render(&mut self, context: &mut WindowIO, elapsedSec: f64) -> ViewAction;
}
