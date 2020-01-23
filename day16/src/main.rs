mod fft;
mod tests;

use crate::fft::FFT;
use std::env;
use std::fs;

fn part1(filename: &str) {
  let input = fs::read_to_string(filename).expect("failed to read input file");
  let mut fft = FFT::from_string(input.trim());
  for _ in 0..100 {
    fft.phase();
  }
  println!(
    "first eight digits of fft after 100 phases: {}",
    &fft.to_string()[0..8]
  );
}

fn part2(filename: &str) {
  let input = fs::read_to_string(filename).expect("failed to read input file");
  let input = input.trim();

  let mut fft = FFT::from_string(&input.repeat(10000));
  for _ in 0..100 {
    fft.fast_phase();
  }

  println!("message: {}", &fft.digest());
}

fn main() {
  let filename = env::args().nth(1).expect("Missing input file argument");

  part1(&filename);
  part2(&filename);
}
