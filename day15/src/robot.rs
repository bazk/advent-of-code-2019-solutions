use crate::computer::{Computer, Interruption};
use crate::grid::Grid;
use crate::point::Point;
use crate::tile::Tile;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Movement {
  North,
  South,
  West,
  East,
}
#[allow(dead_code)]
impl Movement {
  pub fn code(&self) -> i64 {
    match self {
      Movement::North => 1,
      Movement::South => 2,
      Movement::West => 3,
      Movement::East => 4,
    }
  }

  pub fn from_code(code: i64) -> Movement {
    match code {
      3 => Movement::East,
      2 => Movement::West,
      1 => Movement::South,
      _ => Movement::North,
    }
  }

  pub fn sequence() -> Vec<Movement> {
    vec![
      Movement::North,
      Movement::East,
      Movement::South,
      Movement::West,
    ]
  }

  pub fn opposite(&self) -> Movement {
    match self {
      Movement::North => Movement::South,
      Movement::South => Movement::North,
      Movement::West => Movement::East,
      Movement::East => Movement::West,
    }
  }

  pub fn next(&self) -> Movement {
    match self {
      Movement::North => Movement::East,
      Movement::East => Movement::South,
      Movement::South => Movement::West,
      Movement::West => Movement::North,
    }
  }

  pub fn to_point(&self) -> Point {
    match self {
      Movement::North => Point { x: 0, y: 1 },
      Movement::East => Point { x: 1, y: 0 },
      Movement::South => Point { x: 0, y: -1 },
      Movement::West => Point { x: -1, y: 0 },
    }
  }
}

pub struct Robot {
  computer: Computer,
  grid: Grid,
  current_position: Point,
}

impl Robot {
  pub fn new() -> Robot {
    Robot {
      computer: Computer::new(),
      grid: Grid::new(),
      current_position: Point { x: 0, y: 0 },
    }
  }

  pub fn load(&mut self, program: &str) {
    self.computer.flash(program);
  }

  pub fn grid(&self) -> &Grid {
    &self.grid
  }

  pub fn follow(&mut self, path: &Vec<Movement>) {
    for &mov in path.iter() {
      self.step(mov);
    }
  }

  pub fn explore(&mut self, target_tile: Option<Tile>) -> Vec<Movement> {
    let mut queue: VecDeque<Vec<Movement>> = VecDeque::new();
    queue.push_back(vec![]);

    loop {
      let path = queue.pop_front().unwrap();

      self.follow(&path);

      for &mov in Movement::sequence().iter() {
        if path.len() > 0 && mov == path.last().unwrap().opposite() {
          continue;
        }

        let status = self.step(mov);

        if status > 0 {
          // Robot moved successfully to a new position

          // construct the path to this new position
          let mut new_path = path.clone();
          new_path.push(mov);

          // if we have a target and we reached it, stop
          match target_tile {
            Some(target) => {
              if Tile::from_code(status as u32) == target {
                return new_path;
              }
            }
            None => (),
          }

          // add the new path to the queue and move back to previous position to continue exploring
          queue.push_back(new_path);
          self.step(mov.opposite());
        }
      }

      let mut reversed = path.clone();
      reversed.reverse();
      for mov in reversed.iter_mut() {
        *mov = mov.opposite();
      }
      self.follow(&reversed);

      // explored everything
      if queue.len() == 0 {
        match target_tile {
          Some(_) => {
            panic!("target not reacheable");
          }
          None => {
            return path;
          }
        }
      }
    }
  }

  fn step(&mut self, movement: Movement) -> i64 {
    self.computer.input(movement.code());

    loop {
      match self.computer.run() {
        Interruption::Output => {
          let output = self
            .computer
            .output()
            .expect("Failed to retrieve status from computer");

          match output {
            2 => {
              self.current_position += movement.to_point();
              self.grid.set(self.current_position, Tile::Oxygen);
            }
            1 => {
              self.current_position += movement.to_point();
              self.grid.set(self.current_position, Tile::Empty);
            }
            _ => {
              self
                .grid
                .set(self.current_position + movement.to_point(), Tile::Wall);
            }
          }

          return output;
        }
        Interruption::Input => {
          panic!("computer is asking for input but none was given");
        }
        Interruption::Halt => {
          panic!("computer halted");
        }
      }
    }
  }
}
