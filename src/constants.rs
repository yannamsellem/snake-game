pub const GRID_SIZE: (i16, i16) = (30, 20);
pub const GRID_CELL_SIZE: (i16, i16) = (32, 32);

pub const SCREEN_SIZE: (f32, f32) = (
  GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
  GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const UPDATES_PER_SECOND: f32 = 15.0;
pub const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;
