use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::{conf, timer, Context, ContextBuilder, GameResult};
use std::time::{Duration, Instant};

mod constants;
mod direction;
mod food;
mod position;
mod snake;

use crate::constants::{GRID_SIZE, MILLIS_PER_UPDATE, SCREEN_SIZE};
use crate::food::Food;
use crate::position::Position;
use crate::snake::{AteState, Snake};

struct GameState {
    last_update: Instant,
    food: Food,
    snake: Snake,
    game_over: bool,
    score: u16,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        let food_position = Position::random(GRID_SIZE.0, GRID_SIZE.1);

        let snake_position = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();

        GameState {
            last_update: Instant::now(),
            food: Food::new(food_position),
            snake: Snake::new(snake_position),
            game_over: false,
            score: 0,
        }
    }

    fn reset(&mut self) {
        let food_position = Position::random(GRID_SIZE.0, GRID_SIZE.1);
        let snake_position = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();

        self.last_update = Instant::now();
        self.food = Food::new(food_position);
        self.snake = Snake::new(snake_position);
        self.game_over = false;
        self.score = 0;
    }

    fn draw_score(&self, ctx: &mut Context) -> GameResult<()> {
        let score = graphics::Text::new(format!("Score: {}", self.score));

        graphics::draw(
            ctx,
            &score,
            (ggez::mint::Point2 { x: 10.0, y: 10.0 }, graphics::WHITE),
        )
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.game_over {
                self.snake.update(&self.food);

                if let Some(snake_ate) = self.snake.ate {
                    match snake_ate {
                        AteState::Food => {
                            let new_food_position = Position::random(GRID_SIZE.0, GRID_SIZE.1);
                            self.food.position = new_food_position;
                            self.score = self.score + 1
                        }
                        AteState::Snake => self.game_over = true,
                        AteState::Wall => self.game_over = true,
                    }
                }
            }

            self.last_update = Instant::now()
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
        self.draw_score(ctx)?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        mods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Q => {
                if mods.contains(KeyMods::CTRL) || mods.contains(KeyMods::LOGO) {
                    event::quit(ctx)
                }
            }
            KeyCode::Space => {
                if self.game_over {
                    self.reset();
                }
            }
            KeyCode::Up => self.snake.set_direction_from_key(keycode),
            KeyCode::Down => self.snake.set_direction_from_key(keycode),
            KeyCode::Left => self.snake.set_direction_from_key(keycode),
            KeyCode::Right => self.snake.set_direction_from_key(keycode),
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Snake", "Yann")
        .window_setup(conf::WindowSetup::default().title("Snake!"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let mut game_state = GameState::new(&mut ctx);

    event::run(&mut ctx, &mut event_loop, &mut game_state)
}
