extern crate sdl2;
mod events;
mod window_io;

use sdl2::pixels::Color;
use events::Events;
use window_io::{WindowIO, View, ViewAction};

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit {
            break;
        }

        if events.up {
            println!("up");
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
