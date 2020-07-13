use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub parts: LinkedList<SnakePiece>,
    pub direction: Direction,
    width: u32,
}

#[derive(Clone)]
pub struct SnakePiece(pub u32, pub u32);

impl Snake {
    pub fn new(rows: u32, columns: u32, width: u32) -> Snake {
        let mut rng = rand::thread_rng();

        let direction = match rng.gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        };

        Snake {
            parts: LinkedList::from_iter((vec![SnakePiece(columns / 2, rows / 2)]).into_iter()),
            width,
            direction,
        }
    }

    pub fn update(&mut self, just_eaten: bool, columns: u32, rows: u32) -> bool {
        let mut new_head = (*self.parts.front().expect("Snake has no body.")).clone();

        if (self.direction == Direction::Up && new_head.1 == 0)
            || (self.direction == Direction::Left && new_head.0 == 0)
            || (self.direction == Direction::Down && new_head.1 == rows - 1)
            || (self.direction == Direction::Right && new_head.0 == columns - 1)
        {
            return false;
        }

        match self.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }

        if !just_eaten {
            self.parts.pop_back().unwrap();
        }

        if self.is_collide(new_head.0, new_head.1) {
            return false;
        }

        self.parts.push_front(new_head);

        true
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const SNAKE_COLOUR: [f32; 4] = [1.0; 4];

        let squares: Vec<types::Rectangle> = self
            .parts
            .iter()
            .map(|part| SnakePiece(part.0 * self.width, part.1 * self.width))
            .map(|part| rectangle::square(part.0 as f64, part.1 as f64, self.width as f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| rectangle(SNAKE_COLOUR, square, transform, gl));
        });
    }

    #[inline]
    pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.parts.iter().any(|part| x == part.0 && y == part.1)
    }
}
