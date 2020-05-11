mod tests;

use std::env;
use std::fs;

fn part1(filename: &str) {
    let input = fs::read_to_string(filename).expect("failed to read input file");
}

fn part2(filename: &str) {
    let input = fs::read_to_string(filename).expect("failed to read input file");
}

fn main() {
    let filename = env::args().nth(1).expect("Missing input file argument");

    part1(&filename);
    part2(&filename);
}
