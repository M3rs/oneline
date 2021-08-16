use console_engine::{ ConsoleEngine, KeyCode };

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

/// Check that key pressed matches the sequence
pub fn catch_keys(c: &String, catch_idx: &mut usize, engine: &ConsoleEngine) -> bool {
    let rk = c.chars().nth(*catch_idx).unwrap();

    let up = engine.is_key_pressed(KeyCode::Up);
    let down = engine.is_key_pressed(KeyCode::Down);
    let left = engine.is_key_pressed(KeyCode::Left);
    let right = engine.is_key_pressed(KeyCode::Right);

    match rk {
        '<' => {
            if left {
                *catch_idx += 1;
            } else if right || up || down {
                return true;
            }
        },
        '>' => {
            if right { 
                *catch_idx += 1;
            }
            else if left || up || down {
                return true;
            }
        },
        '^' => {
            if up { 
                *catch_idx += 1;
            } else if down || left || right {
                return true;
            }
        },
        'v' => {
            if down {
                *catch_idx += 1;
            } else if up || left || right {
                return true;
            }
        },
        _ => {},
    }

    false
}


