use console_engine::*;
use crate::oneline::{ Actor };

pub struct Waves {
    wavex: i32,

    text: String,
    len: i32,

    skyline: i32,
}

static WAVES_TEXT: &'static str = r"```'-.,_,.-'``'-.,_,.='``'-.,_,.-'``'-.,_,.='````'-.,_,.-'``'-.,_,.='``'-.,_,.-'``'-.,_,.='```";

impl Waves {
    pub fn new(skyline: i32) -> Waves {
        let len = WAVES_TEXT.len() as i32;
        Waves {
            wavex: -1,
            text: String::from(WAVES_TEXT),
            len: len,
            skyline: skyline,
        }
    }

}

impl Actor for Waves {
    fn update(&mut self) {
        self.wavex += 1;
        if self.wavex > self.len {
            self.wavex = 0;
        }
    }

    fn draw(&self, engine: &mut ConsoleEngine) {
        engine.print_fbg(
            self.wavex,
            self.skyline + 1,
            &self.text,
            Color::Blue,
            Color::DarkBlue,
        );
        engine.print_fbg(
            self.wavex - self.len + 1,
            self.skyline + 1,
            &self.text,
            Color::Blue,
            Color::DarkBlue,
        );
    }

}
