use console_engine::*;

mod oneline;
use crate::oneline::{ Actor, Boat, Fish, Waves, Clouds };

fn main() {
    let width: i32 = 80;
    let height: i32 = 24;
    let framerate = 3;

    let skyline = 17;

    let mut score = 0;

    let mut engine = ConsoleEngine::init(width as u32, height as u32, framerate).unwrap();

    let mut boat_t = Boat::new(skyline);
    let mut fish_t = Fish::new(10, 20, 8, width);
    let mut wave_t = Waves::new(skyline); //Waves { wavex: 0 };
    let mut clouds_t = Clouds::new(width);

    let mut actors: Vec<&mut dyn Actor> = vec![
        &mut wave_t,
        &mut clouds_t,
    ];

    loop {
        // wait for next frame + capture input
        engine.wait_frame();

        // handle input
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        if engine.is_key_pressed(KeyCode::Char('c')) {
            //cast = !cast;
            boat_t.toggle_cast();
        }
        if engine.is_key_pressed(KeyCode::Char('r')) && fish_t.is_hooked() {
            fish_t.reel_in();
        }
        if engine.is_key_held(KeyCode::Left) {
            //boatx -= 1;
            boat_t.move_x(-1);
        }
        if engine.is_key_held(KeyCode::Right) {
            //boatx += 1;
            boat_t.move_x(1);
        }

        // update
        for actor in actors.iter_mut() {
            actor.update();
        }
        fish_t.update();

        if boat_t.get_cast() && fish_t.get_x() == (boat_t.get_x() + 10) {
            fish_t.set_hooked(true);
        }

        // render
        engine.clear_screen();

        engine.fill_rect(0, 0, width, skyline, pixel::pxl_bg(' ', Color::Blue)); // sky
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
            engine.print_fbg(1, 1, "Fish on!", Color::White, Color::Blue);
        } else if fish_t.get_y() < skyline {
            engine.print_fbg(1, 1, "Caught 'em!", Color::White, Color::Blue);
        }

        engine.draw();
    }
}
