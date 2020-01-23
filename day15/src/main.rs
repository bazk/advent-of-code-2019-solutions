mod computer;
mod grid;
mod point;
mod robot;
mod tile;

use crate::robot::Robot;
use crate::tile::Tile;
use std::env;

fn part1(filename: &str) {
  let mut robot = Robot::new();
  robot.load(filename);

  let path = robot.explore(Some(Tile::Oxygen));

  println!("{}", robot.grid());
  println!();
  println!("  shortest path: {:?}", path);
  println!("            len: {:?}", path.len());
  println!();
}

fn part2(filename: &str) {
  let mut robot = Robot::new();
  robot.load(filename);

  let path = robot.explore(Some(Tile::Oxygen));
  robot.follow(&path);

  let longest_path = robot.explore(None);

  println!("{}", robot.grid());
  println!();
  println!("  longest path: {:?}", longest_path);
  println!("           len: {:?}", longest_path.len());
  println!();
}

fn main() {
  let filename = env::args().nth(1).expect("Missing input file argument");

  part1(&filename);
  part2(&filename);
}
