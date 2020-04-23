use ggez::event::KeyCode;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::collections::LinkedList;

use crate::constants::GRID_SIZE;
use crate::direction::Direction;
use crate::food::Food;
use crate::position::Position;

#[derive(Clone, Copy)]
pub enum AteState {
  Snake,
  Wall,
  Food,
}

pub struct Snake {
  body: LinkedList<Position>,
  direction: Direction,
  pub ate: Option<AteState>,
}

impl Snake {
  pub fn new(position: Position) -> Self {
    let mut body = LinkedList::new();
    body.push_front(position);
    body.push_back((position.x - 1, position.y).into());

    Snake {
      body,
      direction: Direction::Right,
      ate: None,
    }
  }

  pub fn update(&mut self, food: &Food) {
    let head = self.body.front().unwrap().clone();
    let new_position = Position::new_from_move(head, &self.direction);

    if new_position == food.position {
      self.ate = Some(AteState::Food);
    } else if self.body.contains(&new_position) {
      self.ate = Some(AteState::Snake);
    } else if new_position.x < 0
      || new_position.x > (GRID_SIZE.0 - 1)
      || new_position.y < 0
      || new_position.y > (GRID_SIZE.1 - 1)
    {
      self.ate = Some(AteState::Wall);
    } else {
      self.ate = None
    }

    match self.ate {
      Some(AteState::Food) => self.body.push_front(new_position),
      None => {
        self.body.push_front(new_position);
        self.body.pop_back();
      }
      _ => (),
    }
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    for position in self.body.iter() {
      let rectangle = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        (*position).into(),
        [0.8, 0.3, 0.0, 1.0].into(),
      )?;
      graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    }

    Ok(())
  }

  pub fn set_direction_from_key(&mut self, key: KeyCode) {
    if let Some(new_direction) = Direction::from_keycode(key) {
      if new_direction != self.direction && new_direction.inverse() != self.direction {
        self.direction = new_direction
      }
    }
  }
}
