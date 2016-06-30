use utility::window_io::{WindowIO, View, ViewAction};
use sdl2::pixels::Color;

pub struct MenuView;

impl View for MenuView {
    fn render(&mut self, context: &mut WindowIO, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.quit == true {
            return ViewAction::Quit;
        }

        if events.left && events.down {
            renderer.set_draw_color(Color::RGB(0, 0, 255));
        } else {
            renderer.set_draw_color(Color::RGB(255, 0, 0));
        }

        renderer.clear();

        ViewAction::None
    }
}
