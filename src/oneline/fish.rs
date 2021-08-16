use console_engine::*;
use crate::oneline::{ Actor, Boat };

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

static FISHES_WIDTHS: [i32; 3] = [8, 14, 7];

static FISHES_LEFT: [&'static str; 3] = [
r" _///_  
/o    \/
>_))_./\
   <    ",
r#"  ;,//;,    ,;/
 o:::::::;;///
>::::::::;;\\\
  ''\\\\\'" ';\"#,
r#">*}}}<>"#  
];

static FISHES_RIGHT: [&'static str; 3] = [
r#"  _\\\_ 
\/    o\
/\._))_<
    >   "#,
r#"\;,    ,;\\,; 
\\\;;:::::::o 
\\\;;::::::::<
/;' "'/////'' "#,
r#"><}}}*>"#
];

impl Fish {
    /// Make a new fish
    /// # Arguments
    /// * `x` - x pos of the fish
    /// * `y` - y pos of the fish
    /// * `width` - width of the fish
    pub fn new(x: i32, y: i32, screen_width: i32, fishi: usize) -> Self {

        Self {
            x: x - FISHES_WIDTHS[fishi],
            y: y,
            width: FISHES_WIDTHS[fishi],
            dir: 1,
            hooked: false,
            left: String::from(FISHES_LEFT[fishi]),
            right: String::from(FISHES_RIGHT[fishi]),
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

    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Check if the fish just got hooked
    /// Based on the direction
    pub fn check_hooked(&self, boat_t: &Boat) -> bool {

        return !self.hooked
            && boat_t.get_cast() 
            && if self.dir < 0 {
                self.x
            } else {
                self.x + self.width
            } == (boat_t.get_x() + 10);
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



