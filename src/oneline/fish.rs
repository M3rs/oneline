use console_engine::*;

use crate::oneline::{ Actor };

pub struct Fish {
    x: i32,
    y: i32,
    dir: i32,

    left: String,
    right: String,

    hooked: bool,

    width: i32,
    screen_width: i32,
}

static FISH_LEFT: &'static str = r" _///_  
/o    \/
>_))_./\
   <    ";

static FISH_RIGHT: &'static str = r"  _\\\_ 
\/    o\
/\._))_<
    >   ";

impl Fish {
    /// Make a new fish
    /// # Arguments
    /// * `x` - x pos of the fish
    /// * `y` - y pos of the fish
    /// * `width` - width of the fish
    pub fn new(x: i32, y: i32, width: i32, screen_width: i32) -> Self {

        Self {
            x: x,
            y: y,
            width: width,
            dir: 1,
            hooked: false,
            left: String::from(FISH_LEFT),
            right: String::from(FISH_RIGHT),
            screen_width: screen_width,
        }
    }

    pub fn is_hooked(&self) -> bool {
        self.hooked
    }
    pub fn set_hooked(&mut self, hooked: bool) {
        self.hooked = hooked;
    }

    /// reel the fish in
    pub fn reel_in(&mut self) {
        self.y -= 1;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl Actor for Fish {
    fn update(&mut self) {
        if self.hooked {
            self.dir *= -1;
        } else {
            self.x += 1 * self.dir;
            if self.x + self.width > self.screen_width || self.x + self.width < 0 {
                self.dir *= -1;
            }
        }
    }

    fn draw(&self, engine: &mut ConsoleEngine) {

        engine.print_fbg(
            self.x,
            self.y,
            if self.dir > 0 {
                &self.right
            } else {
                &self.left
            },
            Color::White,
            Color::DarkBlue,
        );
    }

}



