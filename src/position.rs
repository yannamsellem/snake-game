use ggez::graphics;
use rand::{thread_rng, Rng};

use crate::constants::GRID_CELL_SIZE;
use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
  pub x: i16,
  pub y: i16,
}

impl Position {
  pub fn new(x: i16, y: i16) -> Position {
    Position { x, y }
  }

  pub fn random(max_x: i16, max_y: i16) -> Self {
    let mut rng = thread_rng();
    (
      rng.gen_range::<i16, i16, i16>(0, max_x),
      rng.gen_range::<i16, i16, i16>(0, max_y),
    )
      .into()
  }

  pub fn new_from_move(position: Position, direction: &Direction) -> Self {
    match direction {
      Direction::Up => Position::new(position.x, position.y - 1),
      Direction::Down => Position::new(position.x, position.y + 1),
      Direction::Left => Position::new(position.x - 1, position.y),
      Direction::Right => Position::new(position.x + 1, position.y),
    }
  }
}

impl From<Position> for graphics::Rect {
  fn from(position: Position) -> Self {
    graphics::Rect::new_i32(
      position.x as i32 * GRID_CELL_SIZE.0 as i32,
      position.y as i32 * GRID_CELL_SIZE.1 as i32,
      GRID_CELL_SIZE.0 as i32,
      GRID_CELL_SIZE.1 as i32,
    )
  }
}

impl From<(i16, i16)> for Position {
  fn from(position: (i16, i16)) -> Self {
    Position {
      x: position.0,
      y: position.1,
    }
  }
}
