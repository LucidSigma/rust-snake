use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use rand::Rng;

use super::objects::Direction;
use super::objects::Food;
use super::objects::Snake;

pub mod constants {
    pub const ROWS: u32 = 20;
    pub const COLUMNS: u32 = 30;
    pub const SQUARE_PIXEL_SIZE: u32 = 20;
}

pub struct Game {
    gl: GlGraphics,

    has_started: bool,

    rows: u32,
    columns: u32,
    square_width: u32,

    snake: Snake,
    just_eaten: bool,
    food: Food,
    score: u32,
}

impl Game {
    pub fn new(opengl: OpenGL, rows: u32, columns: u32, width: u32) -> Game {
        Game {
            gl: GlGraphics::new(opengl),
            has_started: false,
            rows,
            columns,
            square_width: width,
            snake: Snake::new(rows, columns, width),
            just_eaten: false,
            food: Food { x: 1, y: 1 },
            score: 0,
        }
    }

    pub fn play_frame(&mut self, e: Event) -> bool {
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                self.pressed(&args.button);
            }
        }

        if let Some(args) = e.update_args() {
            if !self.update(args) {
                return false;
            }
        }

        if let Some(args) = e.render_args() {
            self.render(&args);
        }

        true
    }

    pub fn print_start_splash(&self) {
        println!("Welcome to Snake!");
        println!("Press ENTER or SPACE to begin.");
    }

    pub fn print_score(&self) {
        println!("Game over! Your score was {}.", self.score);
    }

    fn pressed(&mut self, button: &Button) {
        if !self.has_started && button == &Button::Keyboard(Key::Space)
            || button == &Button::Keyboard(Key::Return)
        {
            self.has_started = true;
        } else if self.has_started {
            let last_direction = self.snake.direction.clone();

            self.snake.direction = match button {
                &Button::Keyboard(Key::Up) | &Button::Keyboard(Key::W)
                    if last_direction != Direction::Down =>
                {
                    Direction::Up
                }
                &Button::Keyboard(Key::Down) | &Button::Keyboard(Key::S)
                    if last_direction != Direction::Up =>
                {
                    Direction::Down
                }
                &Button::Keyboard(Key::Left) | &Button::Keyboard(Key::A)
                    if last_direction != Direction::Right =>
                {
                    Direction::Left
                }
                &Button::Keyboard(Key::Right) | &Button::Keyboard(Key::D)
                    if last_direction != Direction::Left =>
                {
                    Direction::Right
                }
                _ => last_direction,
            }
        }
    }

    fn update(&mut self, _args: UpdateArgs) -> bool {
        if self.has_started {
            if !self.snake.update(self.just_eaten, self.columns, self.rows) {
                return false;
            }

            if self.just_eaten {
                self.score += 1;
                self.just_eaten = false;
            }

            self.just_eaten = self.food.update(&self.snake);

            if self.just_eaten {
                let mut rng = rand::thread_rng();

                loop {
                    let food_x = rng.gen_range(0, self.columns);
                    let food_y = rng.gen_range(0, self.rows);

                    if !self.snake.is_collide(food_x, food_y) {
                        self.food = Food {
                            x: food_x,
                            y: food_y,
                        };

                        break;
                    }
                }
            }
        }

        true
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND_COLOUR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(BACKGROUND_COLOUR, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args, self.square_width);
    }
}
