use console_engine::*;

use crate::oneline::{ Actor };

pub struct Clouds {
    x: i32,

    screen_width: i32,

    text: String,
}

static CLOUDS_TEXT: &'static str = r"              (`  ).                   _
             (     ).              .:(`  )`.
            _(       '`.          :(   .    )
        .=(`(      .   )     .--  `.  (    ) )
       ((    (..__.:'-'   .+(   )   ` _`  ) )
       `(       ) )       (   .  )     (   )  ._
         ` __.:'   )     (   (   ))     `-'.-(`  )
      ( )       --'       `- __.'         :(      ))
     (_.'          .')                    `(    )  ))
                  (_  )                     ` __.:'";

impl Clouds {
    pub fn new(screen_width: i32) -> Self {

        Self {
            x: -53,
            screen_width: screen_width,
            text: String::from(CLOUDS_TEXT),
        }
    }
}

impl Actor for Clouds {
     fn update(&mut self) {
        self.x += 1;
        if self.x > self.screen_width {
            self.x = -53;
        }
    }

    fn draw(&self, engine: &mut ConsoleEngine) {
        engine.print_fbg(self.x, 1, &self.text, Color::White, Color::Blue);
    }

}
