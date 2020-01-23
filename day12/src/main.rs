#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate num;

mod vector3d;
mod system;

use std::env;
use std::collections::HashMap;
use system::{System, Axis};

fn part1(filename: &str) {
  let mut system = System::load(filename);

  println!("Simulating the system...");

  println!("After 0 steps");
  println!("{:?}", system);

  for _ in 0..1000 {
    system.step();
  }

  println!("After 1000 steps");
  println!("{:?}", system);
  println!("Total energy = {}", system.total_energy());
}

fn part2(filename: &str) {
  let mut system = System::load(filename);

  println!("Looking for the first cycle...");
  println!();

  let mut periods: HashMap<Axis, u64> = HashMap::new();
  let mut initial_states: HashMap<Axis, Vec<(i32, i32)>> = HashMap::new();

  for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
    initial_states.insert(*axis, system.get_axis(axis));
  }

  let mut iteration: u64 = 1;
  loop {
    system.step();

    for axis in [Axis::X, Axis::Y, Axis::Z].iter() {
      if periods.contains_key(axis) {
        // period already found for the axis
        continue
      }

      let initial_state = initial_states.get(axis).unwrap();
      let state = system.get_axis(axis);
      if state == *initial_state {
        periods.insert(*axis, iteration);
      }
    }

    if periods.len() == 3 {
      let x = *periods.get(&Axis::X).unwrap();
      let y = *periods.get(&Axis::Y).unwrap();
      let z = *periods.get(&Axis::Z).unwrap();

      println!("all periods found at iteration {}", iteration);
      println!("  {} {} {}", x, y, z);
      println!("  lcm = {}", num::integer::lcm(num::integer::lcm(x, y), z));
      break
    }

    iteration += 1;
  }
}

fn main() {
  let filename = env::args().nth(1).expect("Missing input file argument");

  part1(&filename);

  println!();
  println!("---");
  println!();

  part2(&filename);
}
