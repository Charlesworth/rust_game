use window_io::{WindowIO, View, ViewAction};
use sdl2::pixels::Color;

pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, context: &mut WindowIO, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

    
        if events.quit {
            return ViewAction::Quit;
        }

        if events.up {
            println!("up");
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}
