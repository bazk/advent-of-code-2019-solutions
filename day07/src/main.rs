extern crate permutohedron;
mod computer;

use std::env;
use computer::{Computer, State, Interruption};
use permutohedron::heap_recursive;

fn run_sequence(program: &str, phase_sequence: &[i32]) -> i32 {
    let mut bus: i32 = 0;

    for i in phase_sequence.iter() {
        let mut computer = Computer::new();
        computer.flash(program);
        computer.input(*i);
        computer.input(bus);
        computer.run();
        bus = computer.output()
            .expect("No output from computer");
    }

    bus
}

fn run_loop(program: &str, phase_sequence: &[i32]) -> i32 {
    let mut bus: i32 = 0;
    let mut computers: Vec<Computer> = vec![Computer::new(); phase_sequence.len()];
    let mut current_computer: usize = 0;

    for (i, computer) in computers.iter_mut().enumerate() {
        computer.flash(program);
        computer.input(phase_sequence[i]);
    }

    loop {
        if let State::Halted = computers[current_computer].get_state() {
            break
        }

        computers[current_computer].input(bus);

        if let Interruption::Output = computers[current_computer].run() {
            bus = computers[current_computer].output()
                .expect("No output from computer");
        }

        current_computer = (current_computer + 1) % computers.len();
    }

    bus
}

#[allow(dead_code)]
fn part1(filename: &str) {
    let mut max_output = 0;
    let mut phase_sequence = [4, 3, 2, 1, 0];

    heap_recursive(&mut phase_sequence, |permutation| {
        let output = run_sequence(filename, &permutation);

        if output >= max_output {
            max_output = output;
        }
    });

    println!("maximum output (no feedback): {}", max_output);
}

#[allow(dead_code)]
fn part2(filename: &str) {
    let mut max_output = 0;
    let mut phase_sequence = [5, 6, 7, 8, 9];

    heap_recursive(&mut phase_sequence, |permutation| {
        let output = run_loop(filename, &permutation);

        if output >= max_output {
            max_output = output;
        }
    });

    println!("maximum output (with feedback): {}", max_output);
}

fn main() {
    let filename = env::args().nth(1).expect("Missing input file argument");

    part1(&filename);
    part2(&filename);
}