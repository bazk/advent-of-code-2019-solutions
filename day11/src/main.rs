mod color;
mod grid;
mod point;
mod robot;
mod computer;

use crate::robot::Robot;
use std::env;

fn main() {
    let filename = env::args().nth(1).expect("Missing input file argument");

    let mut robot = Robot::new();
    robot.program(&filename);
    robot.start();

    println!();
    println!("{}", robot.grid());
    println!();
    println!("total number of painted panels: {}", robot.grid().total_painted());
}
