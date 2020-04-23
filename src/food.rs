use ggez::{self, graphics, Context, GameResult};

use crate::position::Position;

pub struct Food {
  pub position: Position,
}

impl Food {
  pub fn new(position: Position) -> Self {
    Food { position }
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let color = [0.0, 0.0, 1.0, 1.0].into();
    let rectangle =
      graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.position.into(), color)?;

    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
  }
}
