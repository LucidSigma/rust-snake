use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use super::snake::Snake;

pub struct Food {
    pub x: u32,
    pub y: u32,
}

impl Food {
    pub fn update(&mut self, snake: &Snake) -> bool {
        let head = snake.parts.front().unwrap();

        head.0 == self.x && head.1 == self.y
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        const FOOD_COLOUR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;
        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(FOOD_COLOUR, square, transform, gl);
        });
    }
}
