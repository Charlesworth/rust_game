use utility::window_io::{WindowIO, View, ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;

/// Pixels traveled by the player's ship every second, when it is moving.
const PLAYER_SPEED: f64 = 180.0;

pub struct GameView {
    player: Player,
}

impl GameView {
    pub fn new() -> GameView {
        GameView {
            player: Player {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                }
            }
        }
    }
}

impl View for GameView {
    fn render(&mut self, context: &mut WindowIO, elapsedMillis: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.quit {
            return ViewAction::Quit;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // Move the player
        let diagonal =
            (events.up ^ events.down) &&
            (events.left ^ events.right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsedMillis;

        let dx = match (events.left, events.right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (events.up, events.down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        // Render the scene
        renderer.set_draw_color(Color::RGB(200, 200, 50));
        renderer.fill_rect(self.player.rect.to_sdl());

        ViewAction::None
    }
}

struct Player {
    rect: Rectangle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`.
    pub fn to_sdl(self) -> SdlRect {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // SdlRect::new : `(i32, i32, u32, u32) -> SdlRect`
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}
