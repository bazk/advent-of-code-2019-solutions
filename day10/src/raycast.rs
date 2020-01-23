use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops;
use std::cmp;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x={} y={}]", self.x, self.y)
    }
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl ops::Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl Position {
    pub fn is_box_bounded(&self, aa: &Position, bb: &Position) -> bool {
        self.x >= aa.x && self.y >= aa.y && self.x < bb.x && self.y < bb.y
    }
}

pub struct Map {
    pub asteroids: HashMap<Position, bool>,
    pub width: i32,
    pub height: i32,
    pub station: Position
}

impl Map {
    pub fn load(filename: &str) -> Map {
        println!("loading {}", filename);

        let data = fs::read_to_string(filename).expect("Failed to read map file");

        let mut asteroids: HashMap<Position, bool> = HashMap::new();

        let mut x = 0;
        let mut y = 0;
        for line in data.split('\n') {
            x = 0;
            for col in line.chars() {
                match col {
                    '#' => {
                        asteroids.insert(Position { x, y }, true);
                        x += 1;
                    }
                    '.' => {
                        x += 1;
                    }
                    _ => (),
                }
            }
            y += 1;
        }

        Map {
            asteroids,
            width: x,
            height: y,
            station: Position { x: 0, y: 0 }
        }
    }

    pub fn raycast(&self, from: &Position, to: &Position) -> bool {
        fn greatest_common_divisor(n1: i32, n2: i32) -> i32 {
            let mut a = n1;
            let mut b = n2;

            while b != 0 {
               let aux = b;
               b = a % b;
               a = aux;
            }

            return a;
        }

        let dir_x = to.x - from.x;
        let dir_y = to.y - from.y;
        let divisor = greatest_common_divisor(dir_x.abs(), dir_y.abs());
        let step = Position {
            x: dir_x / divisor,
            y: dir_y / divisor
        };

        let mut current_pos = from.clone();
        loop {
            current_pos += step.clone();

            let out_of_bounds = !current_pos.is_box_bounded(
                &Position { x: 0, y: 0 },
                &Position {
                    x: self.width,
                    y: self.height,
                },
            );

            if current_pos == *to || out_of_bounds {
                return true;
            }

            if self.asteroids.contains_key(&current_pos) {
                return false;
            }
        }
    }

    pub fn reachability(&self, from: &Position) -> i32 {
        let mut res = 0;

        for destination in self.asteroids.keys() {
            if destination == from {
                continue;
            }

            if self.raycast(from, destination) {
                res += 1;
            }
        }

        res
    }

    pub fn place_station(&mut self) {
        let mut max: i32 = 0;

        for origin in self.asteroids.keys() {
            let reachable = self.reachability(origin);

            if reachable >= max {
                max = reachable;
                self.station = origin.clone();
            }
        }
    }

    pub fn vaporize(&mut self) -> Vec<Position> {
        let mut vaporized: Vec<Position> = Vec::new();

        for destination in self.asteroids.keys() {
            if *destination == self.station {
                continue;
            }

            if self.raycast(&self.station, destination) {
                vaporized.push(destination.clone());
            }
        }

        fn angle(a: Position) -> f64 {
            let angle = (a.y as f64 / a.x as f64).abs().atan();

            if a.x >= 0 && a.y >= 0 { // Q1
                (std::f64::consts::PI / 2.0) + angle
            } else if a.x >= 0 && a.y < 0 { // Q2
                (std::f64::consts::PI / 2.0) - angle
            } else if a.x < 0 && a.y < 0 { // Q3
                (3.0 * std::f64::consts::PI) / 2.0 + angle
            } else { // Q4
                (3.0 * std::f64::consts::PI) / 2.0 - angle
            }
        }

        vaporized.sort_by(|a, b| {
            let angle_a = angle(*a - self.station);
            let angle_b = angle(*b - self.station);

            if angle_a > angle_b {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            }
        });

        for v in vaporized.iter() {
            self.asteroids.remove(&v);
        }

        vaporized
    }

    pub fn vaporize_until(&mut self, until: usize) -> Result<Position, ()> {
        let mut offset: usize = 0;

        loop {
            let vaporized = self.vaporize();

            if vaporized.len() == 0 {
                return Result::Err(());
            }

            if offset + vaporized.len() >= until {
                let asteroid = vaporized.get((until - offset) - 1)
                    .expect("failed to retrieve asteroid from vaporized list");
                return Result::Ok(asteroid.clone());
            }

            offset += vaporized.len();
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    ").unwrap();
        for x in 0..self.width {
            write!(f, " {}", x % 10).unwrap();
        }
        write!(f, "\n").unwrap();

        for y in 0..self.height {
            write!(f, " {:>2} ", y).unwrap();
            for x in 0..self.width {
                let pos = Position { x, y };

                if self.station == pos {
                    write!(f, "\u{2591}\u{2591}").unwrap();
                } else if self.asteroids.contains_key(&pos) {
                    write!(f, "\u{2588}\u{2588}").unwrap();
                } else {
                    write!(f, "  ").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }

        write!(f, "")
    }
}
