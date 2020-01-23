extern crate minifb;
mod computer;
mod game;
mod grid;
mod tile;
mod ui;

use crate::game::Game;
use crate::tile::Tile;
use crate::ui::UI;
use std::env;
use std::time::{Duration, Instant};

const GAME_WIDTH: usize = 43;
const GAME_HEIGHT: usize = 24;
const PIXEL_SCALE: usize = 8;
const GAME_SPEED: Duration = Duration::from_millis(0);

#[allow(dead_code)]
fn part1(filename: &str) {
  let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);
  game.load(&filename);
  game.step(0);

  let block_count = game.grid().count(Tile::Block);
  println!("count of block tiles: {}", block_count);
}

#[allow(dead_code)]
fn part2(filename: &str) {
  let mut ui = UI::new(
    "Day 13",
    GAME_WIDTH * PIXEL_SCALE,
    GAME_HEIGHT * PIXEL_SCALE,
  );

  let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);

  game.load(&filename);
  game.insert_quarters(2);
  game.step(0);

  let mut last_update = Instant::now();

  while ui.update() {
    ui.clear();

    if last_update.elapsed() >= GAME_SPEED {
      let (ball_x, _) = game.grid().find(Tile::Ball).unwrap_or((0, 0));
      let (paddle_x, _) = game.grid().find(Tile::Paddle).unwrap_or((0, 0));
      let input = (ball_x as i32 - paddle_x as i32).signum();
      game.step(input);
      last_update = Instant::now();
    }

    for (y, row) in game.grid().iter().enumerate() {
      for (x, tile) in row.iter().enumerate() {
        let color = match tile {
          Tile::Empty => 0x000000,
          Tile::Wall => 0x2c3e50,
          Tile::Block => 0xe74c3c,
          Tile::Paddle => 0x8e44ad,
          Tile::Ball => 0x2ecc71,
        };

        for dy in 0..PIXEL_SCALE {
          for dx in 0..PIXEL_SCALE {
            ui.set_pixel((x * PIXEL_SCALE) + dx, (y * PIXEL_SCALE) + dy, color);
          }
        }
      }
    }
  }
}

fn main() {
  let filename = env::args().nth(1).expect("Missing input file argument");

  part1(&filename);
  part2(&filename);
}
