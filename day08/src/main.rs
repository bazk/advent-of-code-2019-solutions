mod image;

use std::env;
use image::Image;

#[allow(dead_code)]
fn part1(filename: &str, width: usize, height: usize) {
    let image = Image::open(filename, width, height);
    println!("image checksum: {}", image.checksum());
}

#[allow(dead_code)]
fn part2(filename: &str, width: usize, height: usize) {
    let image = Image::open(filename, width, height);
    println!("{}", image);
}

fn main() {
    let filename = env::args().nth(1).expect("Missing input file argument");
    let width: usize = env::args().nth(2).expect("Missing width argument")
        .trim().parse().expect("Invalid height value");
    let height: usize = env::args().nth(3).expect("Missing height argument")
        .trim().parse().expect("Invalid height value");

    part1(&filename, width, height);
    part2(&filename, width, height);
}