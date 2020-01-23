mod computer;

use std::env;
use computer::{Computer, Interruption};

#[allow(dead_code)]
fn part1(filename: &str) {
    let mut computer = Computer::new();

    computer.flash(filename);

    computer.input(1);

    println!("running in test mode...");

    loop {
        match computer.run() {
            Interruption::Output => println!("{}", computer.output().expect("Failed to retrieve output from computer")),
            Interruption::Halt => break
        }
    }

    println!();
}

#[allow(dead_code)]
fn part2(filename: &str) {
    let mut computer = Computer::new();

    computer.flash(filename);

    computer.input(2);

    println!("running in boost mode...");

    loop {
        match computer.run() {
            Interruption::Output => println!("{}", computer.output().expect("Failed to retrieve output from computer")),
            Interruption::Halt => break
        }
    }

    println!();
}

fn main() {
    let filename = env::args().nth(1).expect("Missing input file argument");

    part1(&filename);
    part2(&filename);
}