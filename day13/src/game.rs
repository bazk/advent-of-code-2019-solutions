use crate::computer::{Computer, Interruption};
use crate::grid::Grid;
use crate::tile::Tile;

pub struct Game {
  grid: Grid,
  computer: Computer,
  over: bool,
}

#[allow(dead_code)]
impl Game {
  pub fn new(width: usize, height: usize) -> Game {
    Game {
      grid: Grid::new(width, height),
      computer: Computer::new(),
      over: false,
    }
  }

  pub fn is_over(&self) -> bool {
    self.over
  }

  pub fn grid(&self) -> &Grid {
    &self.grid
  }

  pub fn load(&mut self, filename: &str) {
    self.computer.flash(filename);
  }

  pub fn insert_quarters(&mut self, count: usize) {
    self.computer.write(0, count as i64);
  }

  pub fn step(&mut self, input: i32) {
    if self.over {
      return;
    }

    self.computer.input(input as i64);

    let mut cmd = [0; 3];
    let mut cmd_iter = 0;

    loop {
      match self.computer.run() {
        Interruption::Output => {
          cmd[cmd_iter] = self
            .computer
            .output()
            .expect("Failed to retrieve output from computer") as i32;

          cmd_iter += 1;

          // we have a full command
          if cmd_iter >= cmd.len() {
            let x = cmd[0];
            let y = cmd[1];

            if x >= 0 && y >= 0 {
              let tile = Tile::from_code(cmd[2] as u32);
              self.grid.set(x as usize, y as usize, tile);
            } else {
              println!("SCORE = {}", cmd[2]);
            }

            cmd_iter = 0;
          }
        }
        Interruption::Input => {
          break;
        }
        Interruption::Halt => {
          self.over = true;
          break;
        }
      }
    }
  }
}
