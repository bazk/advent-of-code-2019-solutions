use crate::computer::{Computer, Interruption};
use crate::grid::Grid;
use crate::point::Point;
use crate::tile::Tile;

pub struct Camera {
  computer: Computer,
  pub grid: Grid,
}

impl Camera {
  pub fn new(program: &str) -> Self {
    let mut computer = Computer::new();
    computer.flash(program);

    let grid = Grid::new();

    Self { computer, grid }
  }

  pub fn scan(&mut self) {
    self.grid.clear();

    let mut current_position = Point { x: 0, y: 0 };

    loop {
      match self.computer.run() {
        Interruption::Output => {
          let output = self
            .computer
            .output()
            .expect("Failed to retrieve status from computer");

          match output {
            10 => {
              // line break
              current_position.x = 0;
              current_position.y += 1;
            }
            46 => {
              // empty space
              current_position.x += 1;
            }
            n => {
              self.grid.set(current_position, Tile::from_code(n as u32));
              current_position.x += 1;
            }
          }
        }
        Interruption::Input => {
          panic!("computer is asking for input but none was given");
        }
        Interruption::Halt => {
          return;
        }
      }
    }
  }

  pub fn align(&self) -> i32 {
    let mut total = 0;

    for (point, tile) in self.grid.iter() {
      match tile {
        Tile::Scaffold => {
          let neighbors = [
            self.grid.get(&Point::up()),
            self.grid.get(&Point::right()),
            self.grid.get(&Point::down()),
            self.grid.get(&Point::left()),
          ];

          if neighbors[0] == Tile::Scaffold
            && neighbors[1] == Tile::Scaffold
            && neighbors[2] == Tile::Scaffold
            && neighbors[3] == Tile::Scaffold
          {
            total += point.x * point.y
          }
        }
        _ => (),
      }
    }

    total
  }
}
