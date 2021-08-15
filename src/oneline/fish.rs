use console_engine::*;
use std::fs;

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

impl Fish {
    /// Make a new fish
    /// # Arguments
    /// * `x` - x pos of the fish
    /// * `y` - y pos of the fish
    /// * `width` - width of the fish
    pub fn new(x: i32, y: i32, width: i32, screen_width: i32) -> Self {
        let left = fs::read_to_string("fish_left.txt").unwrap();
        let right = fs::read_to_string("fish_right.txt").unwrap();

        Self {
            x: x,
            y: y,
            width: width,
            dir: 1,
            hooked: false,
            left: left,
            right: right,
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


    /// reverse the fish
    fn get_fish_ascii(&self) -> String {
        if self.dir < 0 {
            return self.left.clone();    
        }

        // TODO: better reverse fish algorithm
        let mut s = String::new();
        for line in self.left.lines() {
            let chars: Vec<_> = line
                .chars()
                .rev()
                .map(|c| match c {
                    '\\' => '/',
                    '/' => '\\',
                    '>' => '<',
                    '<' => '>',
                    ')' => '(',
                    '(' => ')',
                    _ => c,
                })
                .collect::<_>();
            let rs: String = chars.iter().collect();

            s.push_str(&rs);
            s.push_str("\n");
        }

        return s;
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



