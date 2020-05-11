mod camera;
mod compressor;
mod computer;
mod grid;
mod path;
mod point;
mod robot;
mod suffix_tree;
mod tests;
mod tile;

use crate::camera::Camera;
use crate::path::PathFinder;
use crate::robot::Robot;
use std::env;

#[allow(dead_code)]
fn part1(filename: &str) {
  let mut camera = Camera::new(filename);
  camera.scan();
  println!("{}", camera.grid);
  println!("alignment = {}", camera.align());
}

#[allow(dead_code)]
fn part2(filename: &str) {
  let mut camera = Camera::new(filename);
  camera.scan();
  println!("grid: {}", camera.grid);

  let mut path_finder = PathFinder::new(&camera.grid);
  let path = path_finder.find();
  println!("path: {:?}", path);

  let compressed = compressor::compress(&path).expect("failed to compress path");
  println!("compressed: {:?}", compressed);

  let mut robot = Robot::new(filename);
  let result = robot.run(&compressed);
  println!("total dust collected: {}", result);
}

fn main() {
  let filename = env::args().nth(1).expect("Missing input file argument");

  // part1(&filename);
  part2(&filename);
}
