use console_engine::*;
use crate::oneline::Actor;


pub struct Boat {
    x: i32,

    cast: bool,

    boat_text: String,
    fishing_text: String,

    skyline: i32,
}

static BOAT_TEXT: &'static str = r"
              O,
\____________/_)_________/";
static FISHING_TEXT: &'static str = r"          ^
          |\  O,
\_________|_\/_)_________/";

impl Boat {
    pub fn new(skyline: i32) -> Self {

        Self {
            x: 8,
            cast: false,
            boat_text: String::from(BOAT_TEXT),
            fishing_text: String::from(FISHING_TEXT),
            skyline: skyline,
        }
    }

    pub fn get_cast(&self) -> bool { self.cast }
    pub fn toggle_cast(&mut self) { self.cast = !self.cast ;}
    pub fn get_x(&self) -> i32 { self.x }
    pub fn move_x(&mut self, x: i32) { self.x += x; }

}

impl Actor for Boat {
    fn update(&mut self) { }

    fn draw(&self, engine: &mut ConsoleEngine) {
        engine.print_fbg(
            self.x,
            self.skyline - 2,
            if self.cast { &self.fishing_text } else { &self.boat_text },
            Color::White,
            Color::Blue
        );
        if self.cast {
            // boatx + 10 (roughly where line is)
            engine.set_pxl(
                self.x + 10,
                self.skyline + 1,
                pixel::pxl_fbg('|', Color::White, Color::DarkBlue)
            );
            engine.set_pxl(
                self.x + 10,
                self.skyline + 2,
                pixel::pxl_fbg('j', Color::White, Color::DarkBlue)
            );
        }
    }
}
