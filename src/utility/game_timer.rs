use sdl2::timer::Timer;

// pub struct Game_timer<'a> {
//     timer: &'a Timer,
//     // pub fps: i32,
//     // pub run_time: i32,
// }
//
// impl Game_timer {
//     pub fn new(timer: &mut Timer) -> Game_timer {
//         Game_timer {
//             timer: timer,
//             // fps: 0,
//             // run_time: 0,
//         }
//     }
//
//     pub fn wait_next_frame(&self) {
//         // unimplemented!()
//         println!("timer fired");
//     }
// }


pub struct Game_timer;

impl Game_timer {
    pub fn new() -> (Game_timer) {
        Game_timer
    }

    pub fn wait_next_frame(&self) {
        // unimplemented!()
        println!("timer fired");
    }
}
