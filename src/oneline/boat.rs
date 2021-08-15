use std::fs;
use console_engine::*;
use crate::oneline::Actor;


pub struct Boat {
    x: i32,

    cast: bool,

    boat_text: String,
    fishing_text: String,

    skyline: i32,
}

impl Boat {
    pub fn new(skyline: i32) -> Self {
        let boat_ascii = fs::read_to_string("boat.txt").unwrap();
        let fishing_ascii = fs::read_to_string("boat_fishing.txt").unwrap();

        Self {
            x: 8,
            cast: false,
            boat_text: boat_ascii,
            fishing_text: fishing_ascii,
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
                pixel::pxl_bg('|', Color::DarkBlue)
            );
            engine.set_pxl(
                self.x + 10,
                self.skyline + 2,
                pixel::pxl_bg('j', Color::DarkBlue)
            );
        }
    }
}
