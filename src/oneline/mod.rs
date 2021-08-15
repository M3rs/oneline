use console_engine::ConsoleEngine;

mod fish;
mod waves;
mod clouds;
mod boat;

pub use self::fish::Fish;
pub use self::waves::Waves;
pub use self::clouds::Clouds;
pub use self::boat::Boat;

pub trait Actor {
    fn update(&mut self);
    fn draw(&self, engine: &mut ConsoleEngine);
}

pub trait Drawable {
    fn draw(&self, engine: &mut ConsoleEngine);
}
