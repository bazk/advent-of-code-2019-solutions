mod raycast;

use std::env;
use raycast::Map;

fn main() {
    let filename = env::args().nth(1).expect("Missing input file argument");

    let mut map = Map::load(&filename);
    map.place_station();

    println!("{}", map);
    println!();
    println!("station reachability: {}", map.reachability(&map.station));
    println!("         at location: {}", map.station);
    println!();

    let vaporized = map.vaporize_until(200)
        .expect("could not vaporize until 200");

    println!("the 200th vaporized asteroid is: {}", vaporized);
    println!();
}