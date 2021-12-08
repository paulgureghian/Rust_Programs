use crate::draw::draw_rectangle;
use piston_window::{types::Color, Context, G2d};

const PLAYER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub struct Paddle {
    x: f64,
    y: f64,
    size: i32,
}

impl Paddle {

    pub fn new(x: f64, y: f64, size: i32) -> Self {

        Self { x, y, size }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {

        draw_rectangle(
            PLAYER_COLOR,
            self.x as f64,
            self.y as f64,
            1,
            self.size,
            con,
            g,
        );
    }

    pub fn slide(&mut self, dir: Option<Direction>, min_y: f64, max_y: f64) {

        let mut new_y: Option<f64> = None;

        if let Some(dir) = dir {
            if dir == Direction::Up {}
                let next_y = self.y - 1.0;
                if next_y > min_y {
                    new_y = Some(next_y);
                }
        } else if dir == Direction::Down {
            let next_y = self.y + 1.0;
            if next_y + (self.size as f64) < max_y {
                new_y = Some(next_y);
            }
        }               
    }
    
    if let Some(new_y) = new_y {
        self.y = new_y
    }
  }
}
