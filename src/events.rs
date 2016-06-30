use sdl2::EventPump;

// The 'Events' object is used to get events from the SDL EventPump
// and match them to its internal set of event bools
pub struct Events {
    pump: EventPump,

    pub quit: bool,
    // pub key_escape: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Events {
    // make a new Event object by passing a SDL EventPump
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            // key_escape: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    /// Update the events record by pattern matching the EventPump's output
    pub fn pump(&mut self) {
        // If the SDL context is dropped, then poll_iter() will simply stop
        // yielding any input.
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => match keycode {
                    // Some(Escape) => self.key_escape = true,
                    Some(Up) => self.up = true,
                    Some(Down) => self.down = true,
                    Some(Left) => self.left = true,
                    Some(Right) => self.right = true,
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    // Some(Escape) => self.key_escape = false,
                    Some(Up) => self.up = false,
                    Some(Down) => self.down = false,
                    Some(Left) => self.left = false,
                    Some(Right) => self.right = false,
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
