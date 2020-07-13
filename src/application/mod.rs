use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::window::WindowSettings;

mod game;
use game::constants::*;
use game::Game;

mod objects;

pub fn play() {
    let opengl = OpenGL::V4_5;

    const WIDTH: u32 = COLUMNS * SQUARE_PIXEL_SIZE;
    const HEIGHT: u32 = ROWS * SQUARE_PIXEL_SIZE;

    let mut window: Window = WindowSettings::new("Snake Game", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(opengl, ROWS, COLUMNS, SQUARE_PIXEL_SIZE);
    let mut events = Events::new(EventSettings::new()).ups(10);

    game.print_start_splash();

    while let Some(e) = events.next(&mut window) {
        if !game.play_frame(e) {
            break;
        }
    }

    game.print_score();
}
