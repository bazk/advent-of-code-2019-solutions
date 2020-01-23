use std::fmt;

use crate::color::Color;
use crate::computer::{Computer, Interruption};
use crate::grid::Grid;
use crate::point::Point;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn to_vector(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
        }
    }

    pub fn rotate(&self, rotation_direction: Direction) -> Direction {
        match rotation_direction {
            Direction::Left => {
                match self {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                }
            }
            Direction::Right => {
                match self {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
            _ => *self,
        }
    }
}

pub struct Robot {
    grid: Grid,
    position: Point,
    direction: Direction,
    computer: Computer,
}

impl fmt::Debug for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Robot {{ position: {:?}, direction: {:?}, computer: {:?} }}",
            self.position, self.direction, self.computer
        )
    }
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            grid: Grid::new(),
            position: Point { x: 0, y: 0 },
            direction: Direction::Up,
            computer: Computer::new(),
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn program(&mut self, filename: &str) {
        self.computer.flash(filename);
    }

    pub fn start(&mut self) {
        let mut painted = false;

        self.grid.paint(self.position, Color::White);

        self.computer
            .input(self.grid.get(self.position).code() as i64);

        loop {
            match self.computer.run() {
                Interruption::Output => {
                    let output = self
                        .computer
                        .output()
                        .expect("Failed to retrieve output from computer")
                        as u32;

                    if !painted {
                        self.paint_grid(output);
                        painted = true;
                    } else {
                        self.move_robot(output);
                        self.computer
                            .input(self.grid.get(self.position).code() as i64);
                        painted = false;
                    }
                }
                Interruption::Halt => break,
            }
        }
    }

    fn paint_grid(&mut self, value: u32) {
        self.grid.paint(self.position, Color::from_code(value));
    }

    fn move_robot(&mut self, value: u32) {
        match value {
            0 => self.direction = self.direction.rotate(Direction::Left),
            1 => self.direction = self.direction.rotate(Direction::Right),
            _ => (),
        }

        self.position += self.direction.to_vector();
    }
}
