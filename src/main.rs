use console_engine::*;
use rand::Rng;

mod oneline;
use crate::oneline::{ Actor, Boat, Fish, Waves, Clouds };

/// Check that key pressed matches the sequence
fn catch_keys(c: &String, catch_idx: &mut usize, engine: &ConsoleEngine) -> bool {
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

fn main() {
    let width: i32 = 80;
    let height: i32 = 24;
    let framerate = 3;

    let skyline = 17;

    let mut score = 0;

    let mut rng = rand::thread_rng();

    let mut engine = ConsoleEngine::init(width as u32, height as u32, framerate).unwrap();

    let mut boat_t = Boat::new(skyline);
    // TODO: Fish Handle Input method?
    let mut fish_t = Fish::new(rng.gen_range(1..width) - 8, 20, 8, width);

    let mut wave_t = Waves::new(skyline); //Waves { wavex: 0 };
    let mut clouds_t = Clouds::new(width);

    let mut actors: Vec<&mut dyn Actor> = vec![
        &mut wave_t,
        &mut clouds_t,
    ];

    let key_opts = ['<', '>', '^', 'v'];
    let mut catch: Option<String> = None;
    let mut catch_idx: usize = 0;
    let mut fail = false;

    loop {
        // wait for next frame + capture input
        engine.wait_frame();

        // handle input
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        if engine.is_key_pressed(KeyCode::Char('c')) {
            boat_t.toggle_cast();
        }

        if !boat_t.get_cast() {
            if engine.is_key_held(KeyCode::Left) {
                boat_t.move_x(-1);
            }
            if engine.is_key_held(KeyCode::Right) {
                boat_t.move_x(1);
            }
        }

        if fish_t.is_hooked() {
            if let Some(ref c) = catch {
                if catch_idx < c.len() {
                    fail = catch_keys(c, &mut catch_idx, &mut engine);
                } else {
                    if engine.is_key_pressed(KeyCode::Char('r')) {
                        fish_t.reel_in();
                    }
                }
            }
        }

        // update
        for actor in actors.iter_mut() {
            actor.update();
        }
        
        fish_t.update();

        if !fish_t.is_hooked() && boat_t.get_cast() && fish_t.get_x() == (boat_t.get_x() + 10) {
            fish_t.set_hooked(true);
            let max: u8 = rng.gen_range(3..7);
            let keys = (1..max).map(|_|{
                let k: usize = rng.gen_range(0..4);
                
                key_opts[k]
            });
            catch = Some(keys.collect());
        }

        // render
        engine.clear_screen();

        engine.fill_rect(0, 0, width, skyline, pixel::pxl_bg(' ', Color::Black)); // ui
        engine.fill_rect(0, 1, width, skyline, pixel::pxl_bg(' ', Color::Blue)); // sky
        engine.fill_rect(0, skyline + 2, width, height, pixel::pxl_bg(' ', Color::DarkBlue)); // water

        for actor in &actors {
            actor.draw(&mut engine);
        }

        // boat
        boat_t.draw(&mut engine);

        // fish
        fish_t.draw(&mut engine);

        // draw UI
        if fish_t.is_hooked() {
            engine.print_fbg(0, 0, "Fish on!", Color::White, Color::Black);
        } 
        if fish_t.get_y() < skyline {
            engine.print_fbg(0, 0, "Caught 'em!", Color::White, Color::Black);
            score += 1;
            boat_t.toggle_cast();

            catch = None;
            catch_idx = 0;
            fish_t = Fish::new(rng.gen_range(1..width) - 8, 20, 8, width);
        }
        if let Some(ref c) = catch {
            if catch_idx < c.len() {
                engine.print_fbg(14, 0, c, Color::White, Color::Black);
                let pc = c.chars().nth(catch_idx).unwrap().to_string();
                engine.print_fbg(14 + catch_idx as i32, 0, &pc, Color::Green, Color::Black);
            } else {
                engine.print_fbg(14, 0, "r", Color::Green, Color::Black);
            }
        }

        if fail {
            engine.print_fbg(14, 14, "Game Over!", Color::Red, Color::Black);
            engine.draw();
            engine.wait_frame();
            break;
        }

        let score_s = format!("Score: {}", score);
        //engine.print_fbg(14, 0, &score_s, Color::White, Color::Black);
        engine.print_fbg(width - score_s.len() as i32, 0, &score_s, Color::White, Color::Black);

        engine.draw();
    }
}
