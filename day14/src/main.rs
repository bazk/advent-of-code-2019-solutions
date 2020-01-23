mod chemicals;
mod nanofactory;
mod tests;

use crate::nanofactory::NanoFactory;
use std::env;

fn part1(filename: &str) {
  let factory = NanoFactory::load(filename);
  let min = factory.minimum_requirements("FUEL", 1);
  println!("ore requirements: {:?}", min);
}

fn part2(filename: &str) {
  let factory = NanoFactory::load(filename);
  let max = factory.maximum_production("FUEL", 1_000_000_000_000);
  println!("maximum production: {:?}", max);
}

fn main() {
  let filename = env::args().nth(1).expect("Missing input file argument");

  part1(&filename);
  part2(&filename);
}
